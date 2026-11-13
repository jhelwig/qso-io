use std::ops::Range;

use bigdecimal::{BigDecimal, ParseBigDecimalError};

#[derive(Debug, Clone)]
pub(crate) struct NumberFailure {
    pub span: Range<usize>,
    pub cause: Option<ParseBigDecimalError>,
}

pub(crate) fn parse_number(bytes: &[u8]) -> Result<BigDecimal, NumberFailure> {
    let mut digits = 0;
    let mut point = false;

    for (index, &byte) in bytes.iter().enumerate() {
        match byte {
            b'-' if index == 0 => {}
            b'0'..=b'9' => digits += 1,
            b'.' if !point => point = true,
            _ => {
                return Err(NumberFailure {
                    span: index..index + 1,
                    cause: None,
                });
            }
        }
    }

    if digits == 0 {
        return Err(NumberFailure {
            span: bytes.len()..bytes.len(),
            cause: None,
        });
    }

    // The grammar above admits ASCII only.
    std::str::from_utf8(bytes)
        .expect("number should be UTF-8 because its grammar check accepted only ASCII bytes")
        .parse()
        .map_err(|cause| NumberFailure {
            span: 0..bytes.len(),
            cause: Some(cause),
        })
}

#[derive(Debug, Clone)]
pub(crate) enum NumericCause {
    Decimal(ParseBigDecimalError),
    Integer(bigdecimal::num_bigint::ParseBigIntError),
}

impl NumericCause {
    pub fn boxed(self) -> Box<dyn std::error::Error + Send + Sync> {
        match self {
            Self::Decimal(error) => Box::new(error),
            Self::Integer(error) => Box::new(error),
        }
    }
}

impl From<ParseBigDecimalError> for NumericCause {
    fn from(value: ParseBigDecimalError) -> Self {
        Self::Decimal(value)
    }
}
