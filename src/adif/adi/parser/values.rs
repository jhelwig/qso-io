use std::mem;

use bigdecimal::num_bigint::BigInt;
use chrono::{NaiveDate, NaiveTime};

use super::*;
use crate::adif::{
    AdifLocation, AdifTime, Hemisphere, TimePrecision, Value, ValueError, number::parse_number,
    text::check_text,
};

mod identifiers;
mod lists;

#[derive(Debug, Clone)]
pub(super) struct ValueFailure {
    pub cause: ValueError,
    pub enumeration: Option<&'static str>,
    pub span: Range<usize>,
}

pub(super) fn source_range(
    text: &str,
    range: Range<usize>,
    encoding: ParseEncoding,
) -> Range<usize> {
    if encoding == ParseEncoding::Latin1 {
        text[..range.start].chars().count()..text[..range.end].chars().count()
    } else {
        range
    }
}

fn digits(bytes: &[u8], start: usize, end: usize) -> std::result::Result<u32, Range<usize>> {
    let component = &bytes[start.min(bytes.len())..end.min(bytes.len())];

    if let Some(offset) = component.iter().position(|byte| !byte.is_ascii_digit()) {
        let index = start + offset;
        return Err(index..index + 1);
    }
    if component.len() < end - start {
        return Err(bytes.len()..bytes.len());
    }

    std::str::from_utf8(component)
        .expect("component should be UTF-8 because every byte was checked as an ASCII digit")
        .parse()
        .map_err(|_| start..end)
}

fn width(bytes: &[u8], size: usize) -> std::result::Result<(), Range<usize>> {
    if bytes.len() > size {
        Err(size..size + 1)
    } else if bytes.len() < size {
        Err(bytes.len()..bytes.len())
    } else {
        Ok(())
    }
}

pub(super) fn parse<'a>(
    input: &Stream<'a>,
    f: FramedField<'a>,
    resolved_type: Option<DataType>,
) -> Result<Field<'a>> {
    let raw = f.raw_data;
    let fail = |cause, span: Range<usize>, numeric_cause| {
        ContextError::from_external_error(
            input,
            AdiError::InvalidValue {
                field_name: f.specifier.name.to_string(),
                cause,
                span: f.value_span.start + span.start..f.value_span.start + span.end,
                numeric_cause,
                enumeration: None,
            },
        )
    };

    let mut text = f.text;
    let value = if raw.is_empty() {
        None
    } else {
        Some(match resolved_type {
            None => Value::Text(mem::take(&mut text)),
            Some(kind @ (DataType::String | DataType::MultilineString | DataType::Enumeration)) => {
                let reason = match kind {
                    DataType::String => ValueError::InvalidString,
                    DataType::MultilineString => ValueError::InvalidMultilineString,
                    _ => ValueError::InvalidEnumeration,
                };
                check_text(
                    raw,
                    input.state.parser_options.encoding,
                    kind == DataType::MultilineString,
                )
                .map_err(|span| fail(reason, span, None))?;

                Value::Text(mem::take(&mut text))
            }
            Some(DataType::Boolean) => {
                let value = match raw[0] {
                    b'Y' | b'y' => true,
                    b'N' | b'n' => false,
                    _ => return Err(fail(ValueError::InvalidBoolean, 0..1, None)),
                };

                width(raw, 1).map_err(|span| fail(ValueError::InvalidBoolean, span, None))?;

                Value::Boolean(value)
            }
            Some(kind @ (DataType::Integer | DataType::PositiveInteger)) => {
                let reason = if kind == DataType::Integer {
                    ValueError::InvalidInteger
                } else {
                    ValueError::InvalidPositiveInteger
                };
                for (index, byte) in raw.iter().enumerate() {
                    if !byte.is_ascii_digit()
                        && !(kind == DataType::Integer && index == 0 && *byte == b'-')
                    {
                        return Err(fail(reason, index..index + 1, None));
                    }
                }
                if raw == b"-" {
                    return Err(fail(reason, raw.len()..raw.len(), None));
                }
                let spelling = str::from_utf8(raw).expect("integer should be UTF-8 because all bytes were checked as ASCII digits or an initial minus");
                let integer: BigInt = spelling.parse().map_err(|cause| {
                    fail(
                        reason,
                        0..raw.len(),
                        Some(crate::adif::number::NumericCause::Integer(cause)),
                    )
                })?;
                if kind == DataType::PositiveInteger && integer <= BigInt::default() {
                    return Err(fail(reason, 0..raw.len(), None));
                }
                Value::Integer(integer)
            }
            Some(
                kind @ (DataType::GridSquare
                | DataType::GridSquareExt
                | DataType::IOTARefNo
                | DataType::POTARef
                | DataType::SOTARef
                | DataType::WWFFRef),
            ) => {
                identifiers::check(kind, &text, raw, input.state.parser_options.encoding).map_err(
                    |error| {
                        ContextError::from_external_error(
                            input,
                            AdiError::InvalidValue {
                                field_name: f.specifier.name.to_string(),
                                cause: error.cause,
                                span: f.value_span.start + error.span.start
                                    ..f.value_span.start + error.span.end,
                                numeric_cause: None,
                                enumeration: error.enumeration,
                            },
                        )
                    },
                )?;
                Value::Text(mem::take(&mut text))
            }
            Some(
                kind @ (DataType::AwardList
                | DataType::CreditList
                | DataType::SponsoredAwardList
                | DataType::GridSquareList
                | DataType::POTARefList
                | DataType::SecondarySubdivisionList
                | DataType::SecondaryAdministrativeSubdivisionListAlt),
            ) => {
                let (members, whole) = lists::parse(
                    kind,
                    mem::take(&mut text),
                    raw,
                    input.state.parser_options.encoding,
                )
                .map_err(|error| {
                    ContextError::from_external_error(
                        input,
                        AdiError::InvalidValue {
                            field_name: f.specifier.name.to_string(),
                            cause: error.cause,
                            span: f.value_span.start + error.span.start
                                ..f.value_span.start + error.span.end,
                            numeric_cause: None,
                            enumeration: error.enumeration,
                        },
                    )
                })?;
                text = whole;
                Value::List(members)
            }
            Some(DataType::Number) => {
                Value::Number(parse_number(raw).map_err(|e| {
                    fail(ValueError::InvalidNumber, e.span, e.cause.map(Into::into))
                })?)
            }
            Some(DataType::Date) => {
                let reason = ValueError::InvalidDate;

                let year = digits(raw, 0, 4).map_err(|span| fail(reason, span, None))?;
                let month = digits(raw, 4, 6).map_err(|span| fail(reason, span, None))?;
                let day = digits(raw, 6, 8).map_err(|span| fail(reason, span, None))?;
                width(raw, 8).map_err(|span| fail(reason, span, None))?;

                if year < 1930 {
                    return Err(fail(reason, 0..4, None));
                }
                if !(1..=12).contains(&month) {
                    return Err(fail(reason, 4..6, None));
                }

                let date = NaiveDate::from_ymd_opt(year as i32, month, day)
                    .ok_or_else(|| fail(reason, 6..8, None))?;
                Value::Date(date)
            }
            Some(DataType::Time) => {
                let reason = ValueError::InvalidTime;
                let size = if raw.len() <= 4 { 4 } else { 6 };

                let hour = digits(raw, 0, 2).map_err(|span| fail(reason, span, None))?;
                let minute = digits(raw, 2, 4).map_err(|span| fail(reason, span, None))?;
                let second = if size == 6 {
                    digits(raw, 4, 6).map_err(|span| fail(reason, span, None))?
                } else {
                    0
                };

                width(raw, size).map_err(|span| fail(reason, span, None))?;

                if hour > 23 {
                    return Err(fail(reason, 0..2, None));
                }
                if minute > 59 {
                    return Err(fail(reason, 2..4, None));
                }
                if second > 59 {
                    return Err(fail(reason, 4..6, None));
                }

                Value::Time(AdifTime {
                    time: NaiveTime::from_hms_opt(hour, minute, second).expect("time should be valid because hour, minute, and second ranges were checked locally"),
                    precision: if size == 4 {
                        TimePrecision::Minute
                    } else {
                        TimePrecision::Second
                    },
                })
            }
            Some(DataType::Location) => {
                let reason = ValueError::InvalidLocation;

                let hemisphere = match raw[0] {
                    b'N' => Hemisphere::North,
                    b'S' => Hemisphere::South,
                    b'E' => Hemisphere::East,
                    b'W' => Hemisphere::West,
                    _ => return Err(fail(reason, 0..1, None)),
                };

                let degrees = digits(raw, 1, 4).map_err(|span| fail(reason, span, None))?;
                if raw.get(4) != Some(&b' ') {
                    return Err(fail(reason, 4.min(raw.len())..5.min(raw.len()), None));
                }

                let minutes = digits(raw, 5, 7).map_err(|span| fail(reason, span, None))?;
                if raw.get(7) != Some(&b'.') {
                    return Err(fail(reason, 7.min(raw.len())..8.min(raw.len()), None));
                }

                digits(raw, 8, 11).map_err(|span| fail(reason, span, None))?;
                width(raw, 11).map_err(|span| fail(reason, span, None))?;

                if degrees > 180 {
                    return Err(fail(reason, 1..4, None));
                }
                if minutes > 59 {
                    return Err(fail(reason, 5..11, None));
                }

                let minutes = parse_number(&raw[5..11]).map_err(|e| {
                    fail(
                        reason,
                        5 + e.span.start..5 + e.span.end,
                        e.cause.map(Into::into),
                    )
                })?;

                Value::Location(AdifLocation {
                    hemisphere,
                    degrees: degrees as u16,
                    minutes,
                })
            }
            // Only descriptor indicators can reach this parser. ADX-only types are
            // rejected before framing; the other categories have no indicator.
            Some(kind) => {
                return Err(ContextError::from_external_error(
                    input,
                    AdiError::TypeUnavailableInAdi {
                        data_type: kind,
                        span: f.descriptor_span.clone(),
                    },
                ));
            }
        })
    };

    let text = if matches!(&value, None | Some(Value::Text(_))) {
        None
    } else {
        Some(text)
    };

    Ok(Field {
        specifier: f.specifier,
        data_type: resolved_type,
        raw_data: raw,
        descriptor_span: f.descriptor_span,
        value_span: f.value_span,
        text,
        value,
    })
}
