use std::{borrow::Cow, ops::Range};

use bigdecimal::{BigDecimal, num_bigint::BigUint};
use thiserror::Error;

use crate::adif::{DataSpecifier, DataType, Field};

/// An ordinary header field or a structured user-field declaration, in source order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderEntry<'a> {
    /// An ordinary field whose indicator describes its own contents.
    Field(Field<'a>),
    /// A declaration whose indicator describes a named record field.
    UserDefinition(UserDefinition<'a>),
}

impl HeaderEntry<'_> {
    /// Returns the original descriptor name, including its supplied casing.
    pub fn name(&self) -> &str {
        match self {
            Self::Field(f) => f.name(),
            Self::UserDefinition(d) => d.name(),
        }
    }

    /// Returns the descriptor's original byte range, with an exclusive end.
    pub fn descriptor_span(&self) -> Range<usize> {
        match self {
            Self::Field(f) => f.descriptor_span(),
            Self::UserDefinition(d) => d.descriptor_span(),
        }
    }

    /// Returns the payload's original byte range, with an exclusive end.
    pub fn value_span(&self) -> Range<usize> {
        match self {
            Self::Field(f) => f.value_span(),
            Self::UserDefinition(d) => d.value_span(),
        }
    }
}

/// A parsed USERDEFn header declaration, retaining its original spelling and bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserDefinition<'a> {
    pub(super) specifier: DataSpecifier<'a>,
    pub(super) number: BigUint,
    pub(super) field_name: String,
    pub(super) constraint: Option<UserDefinitionConstraint>,
    pub(super) target_span: Range<usize>,
    pub(super) constraint_span: Option<Range<usize>>,
    pub(super) raw_data: &'a [u8],
    pub(super) text: Cow<'a, str>,
    pub(super) descriptor_span: Range<usize>,
    pub(super) value_span: Range<usize>,
}

impl<'a> UserDefinition<'a> {
    /// Returns the original USERDEFn descriptor name.
    pub fn name(&self) -> &str {
        &self.specifier.name
    }

    /// Returns the positive declaration number.
    pub fn number(&self) -> &BigUint {
        &self.number
    }

    /// Returns the declared record-field name with its supplied spelling.
    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    /// Returns the supplied record-field type indicator, or None when omitted.
    pub fn data_type(&self) -> Option<DataType> {
        self.specifier.type_indicator
    }

    /// Borrows the optional parsed enumeration or numeric range.
    pub fn constraint(&self) -> Option<&UserDefinitionConstraint> {
        self.constraint.as_ref()
    }

    /// Returns the descriptor length in the selected parsing unit.
    pub fn declared_length(&self) -> usize {
        self.specifier.length
    }

    /// Borrows the original declaration payload bytes.
    pub fn raw_data(&self) -> &'a [u8] {
        self.raw_data
    }

    /// Borrows the original decoded declaration spelling.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the descriptor's original byte range, with an exclusive end.
    pub fn descriptor_span(&self) -> Range<usize> {
        self.descriptor_span.clone()
    }

    /// Returns the payload's original byte range, with an exclusive end.
    pub fn value_span(&self) -> Range<usize> {
        self.value_span.clone()
    }
}

/// An enumeration or exact numeric range declared by USERDEFn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserDefinitionConstraint {
    /// String members in supplied order, preserving spelling; members must be nonempty.
    Enumeration(Vec<String>),
    /// A range whose upper bound is strictly greater than its lower bound.
    Range {
        /// Exact lower bound.
        lower: BigDecimal,
        /// Exact upper bound.
        upper: BigDecimal,
    },
}

/// The part of a USERDEFn declaration that failed parsing.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UserDefinitionError {
    /// The declaration number is not positive.
    #[error("invalid declaration number")]
    InvalidDeclarationNumber,
    /// The target is a standard ADIF field name.
    #[error("user-defined target is a standard ADIF field name")]
    StandardFieldName,
    /// Declaration delimiters or its constraint envelope are malformed.
    #[error("malformed declaration")]
    MalformedDeclaration,
    /// An enumeration member fails the selected String character rules.
    #[error("invalid enumeration member")]
    InvalidEnumeration,
    /// A range bound fails the ADIF Number grammar.
    #[error("invalid numeric range bound")]
    InvalidRangeBound,
    /// The upper bound is equal to or smaller than the lower bound.
    #[error("upper range bound must exceed lower bound")]
    InvalidRangeOrder,
}

/// A supplied nonempty value fails its document USERDEF constraint.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum UserDefinitionValueError {
    /// Contents required for a range are not an ADIF Number.
    #[error("range value is not an ADIF Number: {cause}")]
    InvalidRangeValue {
        /// Type-specific numeric grammar reason.
        cause: crate::adif::ValueError,
    },
    /// A Number falls outside the declaration's inclusive bounds.
    #[error("value is outside inclusive range {lower} through {upper}")]
    OutOfRange {
        /// Exact inclusive lower bound.
        lower: BigDecimal,
        /// Exact inclusive upper bound.
        upper: BigDecimal,
    },
    /// The original spelling matches no member under default Unicode folding.
    #[error("value is not a member of the declared enumeration")]
    NotInEnumeration,
}
