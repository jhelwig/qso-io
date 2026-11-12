use std::{borrow::Cow, collections::HashMap, ops::Range};

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
    adi::{Header, Record, Section, parser::error::AdiError},
};

pub(super) mod error;
#[cfg(test)]
mod tests;
#[cfg(test)]
use super::AdiFile;

#[derive(Debug)]
struct State {
    parser_options: ParseOpts,
}

type Stream<'is> = Stateful<LocatingSlice<&'is [u8]>, State>;

#[derive(Debug, PartialEq, Eq)]
enum Phase {
    Start,
    Records,
    Finished,
}

/// Reads complete ADI sections from a borrowed byte slice.
///
/// Earlier sections remain usable if a later section fails. A structural error
/// is yielded once, followed by permanent exhaustion. Decoding errors remain
/// stored on their reliably framed fields or header preamble.
///
/// A nonempty input whose first byte is not `<` starts with a header preamble
/// and requires an EOH terminator. Input starting with `<` is parsed as records
/// without a header. Text between fields or sections is ignored up to the next
/// `<`; declared field-value lengths take precedence over embedded delimiters.
///
/// ```
/// use qso_io::adif::{adi::{Parser, Section}, ParseOpts, ParseEncoding, LengthUnit};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let options = ParseOpts { encoding: ParseEncoding::Latin1, field_length: LengthUnit::Bytes };
/// for section in Parser::new(b"<CALL:4>W1AW<EOR>", options) {
///     if let Section::Record(record) = section? {
///         assert_eq!(record.field("call").unwrap().text().unwrap(), "W1AW");
///     }
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Parser<'a> {
    input: Stream<'a>,
    input_len: usize,
    phase: Phase,
}

impl<'a> Parser<'a> {
    /// Constructs a parser using the supplied input and options.
    ///
    /// Parsing begins when the iterator advances.
    pub fn new(input: &'a [u8], options: ParseOpts) -> Self {
        Self {
            input: Stateful {
                input: LocatingSlice::new(input),
                state: State {
                    parser_options: options,
                },
            },
            input_len: input.len(),
            phase: Phase::Start,
        }
    }

    fn next_section(&mut self) -> Result<Option<Section<'a>>> {
        if self.phase == Phase::Start {
            self.phase = Phase::Records;
            if self.input.peek_token().is_some_and(|byte| byte != b'<') {
                return header
                    .parse_next(&mut self.input)
                    .map(Section::Header)
                    .map(Some);
            }
        }
        take_till(0.., b'<').void().parse_next(&mut self.input)?;
        if self.input.peek_token().is_none() {
            return Ok(None);
        }
        record
            .parse_next(&mut self.input)
            .map(Section::Record)
            .map(Some)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = std::result::Result<Section<'a>, error::StructuralError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.phase == Phase::Finished {
            return None;
        }
        match self.next_section() {
            Ok(Some(section)) => Some(Ok(section)),
            Ok(None) => {
                self.phase = Phase::Finished;
                None
            }
            Err(context) => {
                self.phase = Phase::Finished;
                Some(Err(error::StructuralError::from_context(
                    context,
                    self.input.current_token_start(),
                    self.input_len,
                )))
            }
        }
    }
}

impl std::iter::FusedIterator for Parser<'_> {}

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
    let mut seen = HashMap::new();
    let parse_opts = input.state.parser_options;
    let ((text_bytes, text_span), (fields, end_span)) = seq!(
        take_till(0.., b'<').with_span().context(StrContext::Label("header text")),
        _: multispace0,
        repeat_till(0.., terminated(preceded(
            not(eof)
                .context(StrContext::Label("header"))
                .context(StrContext::Expected(
                    StrContextValue::StringLiteral("<EOH>"),
                )),
            |input: &mut Stream<'a>| unique_field(input, &mut seen),
        ), take_till(0.., b'<')), eoh)
            .context(StrContext::Label("header field")),
        _: take_till(0.., b'<'),
    )
    .parse_next(input)?;
    let text = decode_text(text_bytes, parse_opts.encoding);

    Ok(Header {
        span: text_span.start..end_span.end,
        text_bytes,
        text_span,
        text,
        fields,
    })
}

fn eor<'a>(input: &mut Stream<'a>) -> Result<Range<usize>> {
    terminated(Caseless("<EOR>").span(), opt(line_ending))
        .context(StrContext::Label("end of record"))
        .parse_next(input)
}

fn eoh<'a>(input: &mut Stream<'a>) -> Result<Range<usize>> {
    Caseless("<EOH>")
        .span()
        .context(StrContext::Label("end of header"))
        .parse_next(input)
}

fn field<'a>(input: &mut Stream<'a>) -> Result<Field<'a>> {
    let (specifier, descriptor_span) = data_specifier.with_span().parse_next(input)?;
    if specifier.length == 0 {
        return Ok(Field {
            specifier,
            descriptor_span,
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
            let mut remaining: &[u8] = input.input.as_ref();
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
                        let start = input.current_token_start();
                        let cause = std::str::from_utf8(&remaining[..remaining.len().min(4)])
                            .expect_err("decode_utf8 rejected this prefix");
                        let mut error = ContextError::from_external_error(
                            input,
                            AdiError::InvalidUtf8 {
                                span: start
                                    ..start + cause.error_len().unwrap_or(remaining.len().min(4)),
                                cause,
                            },
                        );
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
        descriptor_span,
        raw_data,
        value_span,
        text,
    })
}

fn declared_length(input: &mut Stream<'_>) -> Result<usize> {
    let checkpoint = input.checkpoint();
    let start = input.current_token_start();
    let digits: &[u8] = match digit1::<_, ContextError>.parse_next(input) {
        Ok(digits) => digits,
        Err(_) => {
            let span = start..start + usize::from(input.eof_offset() != 0);
            let mut error = ContextError::from_external_error(
                input,
                error::AdiError::Syntax {
                    reason: error::SyntaxErrorKind::InvalidLength,
                    span,
                    cause: None,
                },
            );
            error.push(StrContext::Label("length"));
            return Err(error);
        }
    };
    // digit1 on this byte stream returns ASCII digits only.
    let number = std::str::from_utf8(digits).expect("ASCII digits");
    number.parse::<usize>().map_err(|cause| {
        input.reset(&checkpoint);
        let mut error = ContextError::from_external_error(
            input,
            error::AdiError::Syntax {
                reason: error::SyntaxErrorKind::LengthOverflow,
                span: start..start + digits.len(),
                cause: Some(cause),
            },
        );
        error.push(StrContext::Label("length"));
        error
    })
}

fn data_specifier<'a>(input: &mut Stream<'a>) -> Result<DataSpecifier<'a>> {
    seq!(
        _: '<',
        take_until(0.., ":").context(StrContext::Label("name")),
        _: ':',
        declared_length,
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

fn unique_field<'a>(
    input: &mut Stream<'a>,
    seen: &mut HashMap<String, Range<usize>>,
) -> Result<Field<'a>> {
    let checkpoint = input.checkpoint();
    let parsed = field.parse_next(input)?;
    let key = parsed.name().to_ascii_uppercase();
    if let Some(first_span) = seen.get(&key) {
        let cause = AdiError::DuplicateField {
            field_name: parsed.name().to_owned(),
            first_span: first_span.clone(),
            second_span: parsed.descriptor_span(),
        };
        input.reset(&checkpoint);
        return Err(ContextError::from_external_error(input, cause));
    }
    seen.insert(key, parsed.descriptor_span());
    Ok(parsed)
}

fn record<'a>(input: &mut Stream<'a>) -> Result<Record<'a>> {
    let mut seen = HashMap::new();
    take_until(0.., '<').void().parse_next(input)?;
    let start = input.current_token_start();
    repeat_till(
        1..,
        terminated(
            preceded(
                not(eof)
                    .context(StrContext::Label("record"))
                    .context(StrContext::Expected(StrContextValue::StringLiteral(
                        "<EOR>",
                    ))),
                |input: &mut Stream<'a>| unique_field(input, &mut seen),
            ),
            take_till(0.., b'<'),
        ),
        eor,
    )
    .map(|(fields, end_span)| Record {
        fields,
        span: start..end_span.end,
    })
    .context(StrContext::Label("record"))
    .parse_next(input)
}

#[cfg(test)]
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
                descriptor_span: 0..8,
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
                span: 0..180,
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
                        descriptor_span: 49..61,
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
                        descriptor_span: 67..80,
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
                        descriptor_span: 88..102,
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
                        descriptor_span: 111..126,
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
                        descriptor_span: 146..159,
                        value_span: 159..174,
                        text: Ok(Cow::Owned("ShoeSize,{5:20}".to_owned())),
                    }
                ],
            },
            parsed,
        )
    }
}
