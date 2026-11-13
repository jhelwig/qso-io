use std::{error::Error as StdError, fmt, num::ParseIntError, ops::Range};

use thiserror::Error;
use winnow::error::{ContextError, StrContext, StrContextValue};

use crate::adif::{
    DataType, DataTypeError, DecodeError, LengthUnit, ValueError, adi::UserDefinitionError,
};

/// The syntactic requirement that an ADI input failed to satisfy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SyntaxErrorKind {
    /// A field descriptor is incomplete or does not match the ADI syntax.
    MalformedDescriptor,
    /// The required unsigned ASCII decimal length is missing or malformed.
    InvalidLength,
    /// The decimal length cannot be represented by the parser's `usize`.
    LengthOverflow,
    /// A supplied indicator is not one of the recognized ADIF type indicators.
    UnknownTypeIndicator,
    /// Input ended between header fields without the required EOH tag.
    MissingEoh,
    /// Input ended after complete record fields without the required EOR tag.
    MissingEor,
    /// The supplied field-name bytes are not valid UTF-8.
    InvalidNameEncoding,
}

/// Structured details of a terminal ADI parsing failure.
#[derive(Debug)]
#[non_exhaustive]
pub enum StructuralErrorKind {
    /// Malformed syntax or a required section terminator missing at EOF.
    Syntax {
        /// The specific syntactic requirement that was not satisfied.
        reason: SyntaxErrorKind,
    },
    /// Input ended before a field supplied its declared number of units.
    TruncatedValue {
        /// Field name as supplied in the affected descriptor.
        field_name: String,
        /// Length declared by the descriptor, measured in `unit`.
        declared: usize,
        /// Complete units present before EOF, measured in `unit`.
        available: usize,
        /// Selected length interpretation; both counts use this unit.
        unit: LengthUnit,
    },
    /// A header or record repeats a field name, ignoring ASCII case.
    DuplicateField {
        /// Field name as supplied in the affected descriptor.
        field_name: String,
        /// First descriptor's original byte range, with exclusive end.
        first_span: Range<usize>,
        /// Repeated descriptor's original byte range and primary error location.
        second_span: Range<usize>,
    },
    /// Nonempty field contents fail their explicit ADIF type grammar.
    InvalidValue {
        /// Original field name.
        field_name: String,
        /// Declared type whose grammar failed.
        cause: ValueError,
    },
    /// A USERDEFn declaration failed its own grammar or range requirements.
    InvalidUserDefinition {
        /// Original descriptor name.
        field_name: String,
        /// Declaration-specific reason.
        cause: UserDefinitionError,
    },
    /// A recognized type indicator is unavailable in ADI.
    TypeUnavailableInAdi {
        /// Recognized ADIF data type.
        data_type: DataType,
    },
    /// Text decoding failed in a field or header preamble.
    Decoding {
        /// Decoding cause, with offsets relative to the field or preamble.
        cause: DecodeError,
    },
}

/// A structural error located in the original input byte slice.
///
/// Spans have exclusive ends. EOF failures have an empty span at input length.
/// Display reports the byte range and error details.
#[derive(Debug)]
pub struct StructuralError {
    span: Range<usize>,
    kind: StructuralErrorKind,
    cause: Option<Box<dyn StdError + Send + Sync>>,
}

impl StructuralError {
    /// Returns the primary error range in original input bytes.
    ///
    /// The end is exclusive. Truncation and missing-terminator errors point to
    /// an empty range at EOF; duplicate errors identify the second descriptor.
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    /// Borrows structured details independently of the formatted message.
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
                    AdiError::InvalidValue {
                        field_name,
                        span,
                        cause,
                        numeric_cause,
                    } => (
                        StructuralErrorKind::InvalidValue { field_name, cause },
                        span,
                        numeric_cause.map(|e| Box::new(e) as _),
                    ),
                    AdiError::InvalidUserDefinition {
                        field_name,
                        span,
                        cause,
                        numeric_cause,
                    } => (
                        StructuralErrorKind::InvalidUserDefinition { field_name, cause },
                        span,
                        numeric_cause.map(|e| Box::new(e) as _),
                    ),
                    AdiError::TypeUnavailableInAdi { data_type, span } => (
                        StructuralErrorKind::TypeUnavailableInAdi { data_type },
                        span,
                        None,
                    ),
                    AdiError::Decoding { span, cause } => {
                        (StructuralErrorKind::Decoding { cause }, span, None)
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
            StructuralErrorKind::InvalidValue { field_name, cause } => {
                write!(f, "invalid value for {field_name}: {cause}")
            }
            StructuralErrorKind::InvalidUserDefinition { field_name, cause } => {
                write!(f, "invalid declaration {field_name}: {cause}")
            }
            StructuralErrorKind::TypeUnavailableInAdi { data_type } => {
                write!(f, "type {data_type:?} is unavailable in ADI")
            }
            StructuralErrorKind::Decoding { cause } => {
                write!(f, "text decoding failed: {cause}")
            }
        }
    }
}

impl StdError for StructuralError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        if let StructuralErrorKind::Decoding { cause } = &self.kind {
            return Some(cause);
        }
        if let Some(cause) = self.cause.as_deref() {
            return Some(cause);
        }
        if let StructuralErrorKind::InvalidValue { cause, .. } = &self.kind {
            return Some(cause);
        }
        if let StructuralErrorKind::InvalidUserDefinition { cause, .. } = &self.kind {
            return Some(cause);
        }
        None
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
    #[error("invalid value for {field_name}: {cause}")]
    InvalidValue {
        field_name: String,
        span: Range<usize>,
        cause: ValueError,
        numeric_cause: Option<bigdecimal::ParseBigDecimalError>,
    },
    #[error("invalid declaration {field_name}: {cause}")]
    InvalidUserDefinition {
        field_name: String,
        span: Range<usize>,
        cause: UserDefinitionError,
        numeric_cause: Option<bigdecimal::ParseBigDecimalError>,
    },
    #[error("type {data_type:?} is unavailable in ADI")]
    TypeUnavailableInAdi {
        data_type: DataType,
        span: Range<usize>,
    },
    #[error("text decoding failed: {cause}")]
    Decoding {
        span: Range<usize>,
        #[source]
        cause: DecodeError,
    },
    #[error("invalid field length: {reason:?}")]
    Syntax {
        reason: SyntaxErrorKind,
        span: Range<usize>,
        #[source]
        cause: Option<ParseIntError>,
    },
}
