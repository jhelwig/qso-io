use super::{DataType, ParseEncoding, ValueError, ValueFailure, source_range};
use crate::adif::text::check_text;

pub(super) fn check(
    kind: DataType,
    text: &str,
    raw: &[u8],
    encoding: ParseEncoding,
) -> Result<(), ValueFailure> {
    let cause = match kind {
        DataType::GridSquare => ValueError::InvalidGridSquare,
        DataType::GridSquareExt => ValueError::InvalidGridSquareExt,
        DataType::IOTARefNo => ValueError::InvalidIOTARefNo,
        DataType::POTARef => ValueError::InvalidPOTARef,
        DataType::SOTARef => ValueError::InvalidSOTARef,
        DataType::WWFFRef => ValueError::InvalidWWFFRef,
        _ => ValueError::InvalidString,
    };
    let fail = |range| ValueFailure {
        cause,
        enumeration: None,
        span: source_range(text, range, encoding),
    };

    check_text(raw, encoding, false).map_err(|span| ValueFailure {
        cause,
        enumeration: None,
        span,
    })?;

    match kind {
        DataType::GridSquare | DataType::GridSquareExt => {
            let start = if kind == DataType::GridSquare { 0 } else { 4 };
            for (index, byte) in raw.iter().enumerate() {
                let position = start + index;
                let valid = match position {
                    0 | 1 => (b'A'..=b'R').contains(&byte.to_ascii_uppercase()),
                    4 | 5 => (b'A'..=b'X').contains(&byte.to_ascii_uppercase()),
                    2 | 3 | 6 | 7 => byte.is_ascii_digit(),
                    _ => false,
                };
                if !valid {
                    return Err(ValueFailure {
                        cause,
                        enumeration: None,
                        span: index..index + 1,
                    });
                }
            }
            let valid_width = if start == 0 {
                matches!(raw.len(), 2 | 4 | 6 | 8)
            } else {
                matches!(raw.len(), 2 | 4)
            };
            if !valid_width {
                return Err(ValueFailure {
                    cause,
                    enumeration: None,
                    span: raw.len()..raw.len(),
                });
            }
        }
        DataType::IOTARefNo => {
            if raw.len() < 2 {
                return Err(fail(text.len()..text.len()));
            }
            if !std::str::from_utf8(&raw[..2])
                .is_ok_and(|prefix| crate::adif::Enumeration::Continent.contains(prefix))
            {
                return Err(ValueFailure {
                    cause,
                    enumeration: Some("Continent"),
                    span: 0..2,
                });
            }
            if raw.get(2) != Some(&b'-') {
                return Err(ValueFailure {
                    cause,
                    enumeration: None,
                    span: 2.min(raw.len())..3.min(raw.len()),
                });
            }
            for (index, byte) in raw.iter().enumerate().skip(3) {
                if !byte.is_ascii_digit() || index >= 6 {
                    return Err(ValueFailure {
                        cause,
                        enumeration: None,
                        span: index..index + 1,
                    });
                }
            }
            if raw.len() < 6 {
                return Err(fail(text.len()..text.len()));
            }
            if &raw[3..] == b"000" {
                return Err(ValueFailure {
                    cause,
                    enumeration: None,
                    span: 3..6,
                });
            }
        }
        DataType::SOTARef => {
            let Some((left, right)) = text.split_once('/') else {
                return Err(fail(text.len()..text.len()));
            };
            if left.is_empty() {
                return Err(fail(0..0));
            }
            if right.is_empty() {
                return Err(fail(text.len()..text.len()));
            }
        }
        DataType::POTARef | DataType::WWFFRef => {
            let (base, suffix) = if kind == DataType::POTARef {
                match text.split_once('@') {
                    Some((base, suffix)) => (base, Some(suffix)),
                    None => (text, None),
                }
            } else {
                (text, None)
            };
            let Some((program, number)) = base.rsplit_once('-') else {
                return Err(fail(base.len()..base.len()));
            };
            let prefix = if kind == DataType::WWFFRef {
                let Some(end) = program.len().checked_sub(2) else {
                    return Err(fail(0..program.len()));
                };
                let Some(marker) = program.get(end..) else {
                    return Err(fail(0..program.len()));
                };
                if !marker.eq_ignore_ascii_case("FF") {
                    return Err(fail(end..program.len()));
                }
                &program[..end]
            } else {
                program
            };
            if !(1..=4).contains(&prefix.chars().count()) {
                return Err(fail(0..program.len()));
            }
            let number_start = program.len() + 1;
            for (index, byte) in number.bytes().enumerate() {
                if !byte.is_ascii_digit() {
                    // A malformed Unicode scalar is reported in full, not split at
                    // an invalid UTF-8 boundary before Latin-1 source mapping.
                    let start = number_start + index;
                    let end = text[start..]
                        .chars()
                        .next()
                        .map_or(start, |c| start + c.len_utf8());
                    return Err(fail(start..end));
                }
            }
            let valid_width = if kind == DataType::WWFFRef {
                number.len() == 4
            } else {
                matches!(number.len(), 4 | 5)
            };
            if !valid_width {
                return Err(fail(number_start..base.len()));
            }
            if let Some(suffix) = suffix
                && (!(4..=6).contains(&suffix.chars().count()) || suffix.contains('@'))
            {
                return Err(fail(base.len() + 1..text.len()));
            }
        }
        _ => return Err(fail(0..text.len())),
    }

    Ok(())
}
