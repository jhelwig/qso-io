use std::{error::Error as StdError, fmt, num::ParseIntError, ops::Range, str::Utf8Error};

use thiserror::Error;
use winnow::error::{ContextError, StrContext, StrContextValue};

use crate::adif::{DataTypeError, LengthUnit};

/// The syntactic requirement that an ADI input failed to satisfy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SyntaxErrorKind {
    MalformedDescriptor,
    InvalidLength,
    LengthOverflow,
    UnknownTypeIndicator,
    MissingEoh,
    MissingEor,
    InvalidNameEncoding,
}

/// Structured details of a terminal ADI parsing failure.
#[derive(Debug)]
#[non_exhaustive]
pub enum StructuralErrorKind {
    Syntax {
        reason: SyntaxErrorKind,
    },
    TruncatedValue {
        field_name: String,
        declared: usize,
        available: usize,
        unit: LengthUnit,
    },
    DuplicateField {
        field_name: String,
        first_span: Range<usize>,
        second_span: Range<usize>,
    },
    InvalidUtf8 {
        cause: Utf8Error,
    },
}

/// A structural error located in the original input byte slice.
///
/// Spans have exclusive ends. EOF failures have an empty span at input length.
/// Display reports metadata, without rendering the source input.
#[derive(Debug)]
pub struct StructuralError {
    span: Range<usize>,
    kind: StructuralErrorKind,
    cause: Option<Box<dyn StdError + Send + Sync>>,
}

impl StructuralError {
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }
    pub fn kind(&self) -> &StructuralErrorKind {
        &self.kind
    }

    pub(super) fn from_context(error: ContextError, offset: usize, input_len: usize) -> Self {
        let point = offset..offset.saturating_add(1).min(input_len);
        if let Some(cause) = error.cause().and_then(|e| e.downcast_ref::<AdiError>()) {
            let (kind, span, source): (_, _, Option<Box<dyn StdError + Send + Sync>>) =
                match cause.clone() {
                    AdiError::TruncatedValue {
                        field_name,
                        declared,
                        available,
                        unit,
                    } => (
                        StructuralErrorKind::TruncatedValue {
                            field_name,
                            declared,
                            available,
                            unit,
                        },
                        offset..offset,
                        None,
                    ),
                    AdiError::DuplicateField {
                        field_name,
                        first_span,
                        second_span,
                    } => (
                        StructuralErrorKind::DuplicateField {
                            field_name,
                            first_span,
                            second_span: second_span.clone(),
                        },
                        second_span,
                        None,
                    ),
                    AdiError::InvalidUtf8 { span, cause } => {
                        (StructuralErrorKind::InvalidUtf8 { cause }, span, None)
                    }
                    AdiError::Syntax {
                        reason,
                        span,
                        cause,
                    } => (
                        StructuralErrorKind::Syntax { reason },
                        span,
                        cause.map(|e| Box::new(e) as _),
                    ),
                };
            return Self {
                span,
                kind,
                cause: source,
            };
        }
        if let Some(DataTypeError::UnknownTypeSpecifier(indicator)) = error
            .cause()
            .and_then(|e| e.downcast_ref::<DataTypeError>())
        {
            return Self {
                span: offset..offset.saturating_add(indicator.len().max(1)).min(input_len),
                kind: StructuralErrorKind::Syntax {
                    reason: SyntaxErrorKind::UnknownTypeIndicator,
                },
                cause: Some(Box::new(DataTypeError::UnknownTypeSpecifier(
                    indicator.clone(),
                ))),
            };
        }
        if let Some(cause) = error
            .cause()
            .and_then(|e| e.downcast_ref::<std::string::FromUtf8Error>())
        {
            let start = offset + 1 + cause.utf8_error().valid_up_to();
            let end = start
                + cause
                    .utf8_error()
                    .error_len()
                    .unwrap_or(cause.as_bytes().len() - cause.utf8_error().valid_up_to());
            return Self {
                span: start..end,
                kind: StructuralErrorKind::Syntax {
                    reason: SyntaxErrorKind::InvalidNameEncoding,
                },
                cause: Some(Box::new(cause.clone())),
            };
        }
        // These are structured expected tokens, not rendered error messages.
        let reason = error
            .context()
            .find_map(|c| match c {
                StrContext::Expected(StrContextValue::StringLiteral("<EOH>")) => {
                    Some(SyntaxErrorKind::MissingEoh)
                }
                StrContext::Expected(StrContextValue::StringLiteral("<EOR>")) => {
                    Some(SyntaxErrorKind::MissingEor)
                }
                _ => None,
            })
            .unwrap_or(SyntaxErrorKind::MalformedDescriptor);
        Self {
            span: point,
            kind: StructuralErrorKind::Syntax { reason },
            cause: None,
        }
    }
}

impl fmt::Display for StructuralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ADI error at bytes {}..{}: ",
            self.span.start, self.span.end
        )?;
        match &self.kind {
            StructuralErrorKind::Syntax { reason } => write!(
                f,
                "{}",
                match reason {
                    SyntaxErrorKind::MalformedDescriptor => "malformed data descriptor",
                    SyntaxErrorKind::InvalidLength => "invalid field length",
                    SyntaxErrorKind::LengthOverflow =>
                        "field length exceeds the supported integer range",
                    SyntaxErrorKind::UnknownTypeIndicator => "unknown data type indicator",
                    SyntaxErrorKind::MissingEoh => "expected <EOH> before EOF",
                    SyntaxErrorKind::MissingEor => "expected <EOR> before EOF",
                    SyntaxErrorKind::InvalidNameEncoding => "invalid UTF-8 in field name",
                }
            ),
            StructuralErrorKind::TruncatedValue {
                field_name,
                declared,
                available,
                unit,
            } => write!(
                f,
                "truncated value for {field_name}: declared {declared}, available {available} ({unit:?})"
            ),
            StructuralErrorKind::DuplicateField {
                field_name,
                first_span,
                ..
            } => write!(
                f,
                "duplicate field {field_name}; first descriptor at bytes {}..{}",
                first_span.start, first_span.end
            ),
            StructuralErrorKind::InvalidUtf8 { cause } => {
                write!(f, "cannot determine scalar value boundary: {cause}")
            }
        }
    }
}

impl StdError for StructuralError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        if let StructuralErrorKind::InvalidUtf8 { cause } = &self.kind {
            return Some(cause);
        }
        self.cause.as_deref().map(|e| e as _)
    }
}

#[derive(Debug, Error, Clone)]
pub(super) enum AdiError {
    #[error("duplicate field {field_name}")]
    DuplicateField {
        field_name: String,
        first_span: Range<usize>,
        second_span: Range<usize>,
    },
    #[error(
        "truncated value for {field_name}: declared {declared}, found {available} before EOF (unit: {unit:?})"
    )]
    TruncatedValue {
        field_name: String,
        declared: usize,
        available: usize,
        unit: LengthUnit,
    },
    #[error("invalid UTF-8 during scalar framing: {cause}")]
    InvalidUtf8 {
        span: Range<usize>,
        #[source]
        cause: Utf8Error,
    },
    #[error("invalid field length: {reason:?}")]
    Syntax {
        reason: SyntaxErrorKind,
        span: Range<usize>,
        #[source]
        cause: Option<ParseIntError>,
    },
}
