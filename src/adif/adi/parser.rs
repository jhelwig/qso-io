use std::{borrow::Cow, ops::Range};

use winnow::{
    LocatingSlice, Parser as _, Result, Stateful,
    ascii::{Caseless, digit1, line_ending, multispace0},
    combinator::{eof, not, opt, preceded, repeat_till, seq, terminated},
    error::{ContextError, FromExternalError, StrContext, StrContextValue},
    stream::{AsChar, Location, Stream as _},
    token::{take_till, take_until, take_while},
};

use crate::adif::{
    DataSpecifier, DataType, DecodeError, Field, LengthUnit, ParseEncoding, ParseOpts,
    adi::{AdiFile, Header, Record, parser::error::AdiError},
};

mod error;
#[cfg(test)]
mod tests;

#[derive(Debug)]
struct State {
    parser_options: ParseOpts,
}

type Stream<'is> = Stateful<LocatingSlice<&'is [u8]>, State>;

fn decode_text<'a>(bytes: &'a [u8], encoding: ParseEncoding) -> Result<Cow<'a, str>, DecodeError> {
    match encoding {
        ParseEncoding::Ascii => {
            if let Some(offset) = bytes.iter().position(|byte| !byte.is_ascii()) {
                return Err(DecodeError::NonAscii { offset });
            }
            str::from_utf8(bytes).map(Cow::Borrowed).map_err(Into::into)
        }
        ParseEncoding::Utf8 => str::from_utf8(bytes).map(Cow::Borrowed).map_err(Into::into),
        ParseEncoding::Latin1 => Ok(encoding_rs::mem::decode_latin1(bytes)),
    }
}

fn header<'a>(input: &mut Stream<'a>) -> Result<Header<'a>> {
    let parse_opts = input.state.parser_options;
    let ((text_bytes, text_span), fields) = seq!(
        take_till(0.., b'<').with_span().context(StrContext::Label("header text")),
        _: multispace0,
        repeat_till(0.., terminated(preceded(
            not(eof)
                .context(StrContext::Label("header"))
                .context(StrContext::Expected(
                    StrContextValue::StringLiteral("<EOH>"),
                )),
            field,
        ), take_till(0.., b'<')), eoh)
            .map(|(f, _)| f)
            .context(StrContext::Label("header field")),
        _: take_till(0.., b'<'),
    )
    .parse_next(input)?;
    let text = decode_text(text_bytes, parse_opts.encoding);

    Ok(Header {
        text_bytes,
        text_span,
        text,
        fields,
    })
}

fn eor<'a>(input: &mut Stream<'a>) -> Result<()> {
    (Caseless("<EOR>"), opt(line_ending))
        .void()
        .context(StrContext::Label("end of record"))
        .parse_next(input)
}

fn eoh<'a>(input: &mut Stream<'a>) -> Result<()> {
    Caseless("<EOH>")
        .void()
        .context(StrContext::Label("end of header"))
        .parse_next(input)
}

fn field<'a>(input: &mut Stream<'a>) -> Result<Field<'a>> {
    let specifier = data_specifier.parse_next(input)?;
    if specifier.length == 0 {
        return Ok(Field {
            specifier,
            raw_data: &[],
            value_span: Range {
                start: input.current_token_start(),
                end: input.current_token_start(),
            },
            text: Ok(Cow::Borrowed("")),
        });
    }
    let parse_opts = input.state.parser_options;
    let truncation = |available| AdiError::TruncatedValue {
        field_name: specifier.name.to_string(),
        declared: specifier.length,
        available,
        unit: parse_opts.field_length,
    };

    let byte_count = match (parse_opts.encoding, parse_opts.field_length) {
        (ParseEncoding::Utf8, LengthUnit::UnicodeScalars) => {
            let mut remaining: &[u8] = *input.input.as_ref();
            let mut byte_count = 0;

            for idx in 0..specifier.length {
                let (character, width) = bstr::decode_utf8(remaining);
                if character.is_none() {
                    let _ = input.next_slice(byte_count);

                    let error = if remaining.is_empty() {
                        let mut error = ContextError::from_external_error(input, truncation(idx));
                        error.push(StrContext::Label("field data"));

                        error
                    } else {
                        let mut error = ContextError::new();
                        error.push(StrContext::Label("field data"));
                        error.push(StrContext::Expected(StrContextValue::Description(
                            "a complete, valid UTF-8 scalar",
                        )));

                        error
                    };

                    return Err(error);
                }

                byte_count += width;
                remaining = &remaining[width..];
            }
            byte_count
        }
        _ => {
            let available_bytes = input.eof_offset();
            let byte_count = specifier.length;

            if available_bytes < byte_count {
                let _ = input.next_slice(available_bytes);

                let mut error =
                    ContextError::from_external_error(input, truncation(available_bytes));
                error.push(StrContext::Label("field data"));
                return Err(error);
            }

            byte_count
        }
    };
    let (raw_data, value_span) = winnow::token::take(byte_count)
        .with_span()
        .context(StrContext::Label("field data"))
        .parse_next(input)?;
    let text = decode_text(raw_data, parse_opts.encoding);

    Ok(Field {
        specifier,
        raw_data,
        value_span,
        text,
    })
}

fn data_specifier<'a>(input: &mut Stream<'a>) -> Result<DataSpecifier<'a>> {
    seq!(
        _: '<',
        take_until(0.., ":").context(StrContext::Label("name")),
        _: ':',
        digit1
            .parse_to::<usize>()
            .context(StrContext::Label("length")),
        optional_data_type,
        _: '>',
    )
    .try_map(|(name, length, data_type)| {
        let name = Cow::Owned(String::from_utf8(name.to_owned())?);
        Ok::<_, std::string::FromUtf8Error>(DataSpecifier {
            name,
            length,
            data_type,
        })
    })
    .context(StrContext::Label("data specifier"))
    .parse_next(input)
}

fn data_type<'a>(input: &mut Stream<'a>) -> Result<DataType> {
    take_while(0.., AsChar::is_alpha)
        .try_map(|code: &'a [u8]| {
            let str_code = String::from_utf8(code.to_owned())?;
            DataType::try_from(str_code.as_ref())
        })
        .context(StrContext::Label("data type"))
        .parse_next(input)
}

fn optional_data_type<'a>(input: &mut Stream<'a>) -> Result<Option<DataType>> {
    if opt(':').parse_next(input)?.is_some() {
        data_type.parse_next(input).map(Some)
    } else {
        Ok(None)
    }
}

fn record<'a>(input: &mut Stream<'a>) -> Result<Record<'a>> {
    (
        take_until(0.., '<').void(),
        repeat_till(
            1..,
            terminated(
                preceded(
                    not(eof)
                        .context(StrContext::Label("record"))
                        .context(StrContext::Expected(StrContextValue::StringLiteral(
                            "<EOR>",
                        ))),
                    field,
                ),
                take_till(0.., b'<'),
            ),
            eor,
        ),
    )
        .map(|(_, (fields, _))| Record { fields })
        .context(StrContext::Label("record"))
        .parse_next(input)
}

fn adi_file<'a>(input: &mut Stream<'a>) -> Result<AdiFile<'a>> {
    let header = match input.peek_token() {
        None | Some(b'<') => None,
        Some(_) => Some(header.parse_next(input)?),
    };

    let records = repeat_till(0.., record, eof)
        .map(|(r, _)| r)
        .context(StrContext::Label("adi file"))
        .parse_next(input)?;

    Ok(AdiFile { header, records })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_data_type() {
        let mut basic = Stream {
            input: LocatingSlice::new("N".as_bytes()),
            state: State {
                parser_options: ParseOpts::default(),
            },
        };
        let parsed = data_type.parse_next(&mut basic).expect("should be valid");

        assert_eq!(DataType::Number, parsed,);
    }

    #[test]
    fn basic_data_specifier() {
        let mut basic = Stream {
            input: LocatingSlice::new("<NAME:3>Ray".as_bytes()),
            state: State {
                parser_options: ParseOpts::default(),
            },
        };
        let parsed = data_specifier(&mut basic).expect("should be valid");

        assert_eq!(
            DataSpecifier {
                name: Cow::Owned("NAME".to_owned()),
                length: 3,
                data_type: None
            },
            parsed
        )
    }

    #[test]
    fn basic_field() {
        let mut basic = Stream {
            input: LocatingSlice::new("<NAME:3>Ray".as_bytes()),
            state: State {
                parser_options: ParseOpts::default(),
            },
        };
        let parsed = field(&mut basic).expect("should be valid");

        assert_eq!(
            Field {
                specifier: DataSpecifier {
                    name: Cow::Owned("NAME".to_owned()),
                    length: 3,
                    data_type: None,
                },
                raw_data: "Ray".as_bytes(),
                value_span: 8..11,
                text: Ok(Cow::Owned("Ray".to_owned())),
            },
            parsed
        )
    }

    #[test]
    fn basic_header() {
        let mut header_text = Stream {
            input: LocatingSlice::new(
                "Generated on 2011-11-22 at 02:15:23Z for WN4AZY

<adif_ver:5>3.0.5
<programid:7>MonoLog
<USERDEF1:8:N>QRP_ARCI
<USERDEF2:19:E>SweaterSize,{S,M,L}
<USERDEF3:15>ShoeSize,{5:20}
<EOH>"
                    .as_bytes(),
            ),
            state: State {
                parser_options: ParseOpts::default(),
            },
        };
        let parsed = header(&mut header_text).expect("should be valid");

        assert_eq!(
            Header {
                text_bytes: "Generated on 2011-11-22 at 02:15:23Z for WN4AZY\n\n".as_bytes(),
                text_span: 0..49,
                text: Ok("Generated on 2011-11-22 at 02:15:23Z for WN4AZY\n\n".into()),
                fields: vec![
                    Field {
                        specifier: DataSpecifier {
                            name: Cow::Owned("adif_ver".to_owned()),
                            length: 5,
                            data_type: None
                        },
                        raw_data: "3.0.5".as_bytes(),
                        value_span: 61..66,
                        text: Ok(Cow::Owned("3.0.5".to_owned())),
                    },
                    Field {
                        specifier: DataSpecifier {
                            name: Cow::Owned("programid".to_owned()),
                            length: 7,
                            data_type: None
                        },
                        raw_data: "MonoLog".as_bytes(),
                        value_span: 80..87,
                        text: Ok(Cow::Owned("MonoLog".to_owned())),
                    },
                    Field {
                        specifier: DataSpecifier {
                            name: Cow::Owned("USERDEF1".to_owned()),
                            length: 8,
                            data_type: Some(DataType::Number)
                        },
                        raw_data: "QRP_ARCI".as_bytes(),
                        value_span: 102..110,
                        text: Ok(Cow::Owned("QRP_ARCI".to_owned())),
                    },
                    Field {
                        specifier: DataSpecifier {
                            name: Cow::Owned("USERDEF2".to_owned()),
                            length: 19,
                            data_type: Some(DataType::Enumeration)
                        },
                        raw_data: "SweaterSize,{S,M,L}".as_bytes(),
                        value_span: 126..145,
                        text: Ok(Cow::Owned("SweaterSize,{S,M,L}".to_owned())),
                    },
                    Field {
                        specifier: DataSpecifier {
                            name: Cow::Owned("USERDEF3".to_owned()),
                            length: 15,
                            data_type: None
                        },
                        raw_data: "ShoeSize,{5:20}".as_bytes(),
                        value_span: 159..174,
                        text: Ok(Cow::Owned("ShoeSize,{5:20}".to_owned())),
                    }
                ],
            },
            parsed,
        )
    }
}
