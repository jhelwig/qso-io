//! Read length-delimited ADI input as complete borrowed headers and records.
//!
//! [`Parser`](crate::adif::adi::Parser) yields [`Section`](crate::adif::adi::Section)
//! values in input order. Structural failures are
//! terminal, while text-decoding errors remain stored in their complete fields
//! or header preamble. All spans use original byte offsets and exclusive ends.

use std::{borrow::Cow, ops::Range};

use crate::adif::{DecodeError, Field};

mod parser;

pub use parser::Parser;
pub use parser::error::{StructuralError, StructuralErrorKind, SyntaxErrorKind};

/// A complete header or record from an ADI input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Section<'a> {
    /// A header terminated by EOH, possibly containing stored decoding errors.
    Header(Header<'a>),
    /// A nonempty field sequence terminated by EOR.
    Record(Record<'a>),
}

/// An ADI file containing an optional header and an ordered sequence of records.
///
/// Public parsing currently uses [`Parser`] and [`Section`]; this container
/// does not yet expose a public constructor or accessors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdiFile<'a> {
    header: Option<Header<'a>>,
    records: Vec<Record<'a>>,
}

/// A complete ADI header with its original preamble and ordered fields.
///
/// Its source span ends after EOH's closing `>` and excludes following
/// separators. Preamble and field decoding errors are stored in the header.
/// Field names are unique within the header under ASCII case-insensitive
/// comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    span: Range<usize>,
    text_bytes: &'a [u8],
    text_span: Range<usize>,
    text: Result<Cow<'a, str>, DecodeError>,
    fields: Vec<Field<'a>>,
}

impl<'a> Header<'a> {
    /// Borrows all fields in their original order.
    pub fn fields(&self) -> &[Field<'a>] {
        &self.fields
    }
    /// Finds a field by ASCII case-insensitive name, or returns `None`.
    ///
    /// The returned field retains its original name casing.
    pub fn field(&self, name: &str) -> Option<&Field<'a>> {
        self.fields
            .iter()
            .find(|f| f.name().eq_ignore_ascii_case(name))
    }
    /// Borrows the original preamble bytes, including line endings and blanks.
    pub fn raw_text(&self) -> &'a [u8] {
        self.text_bytes
    }
    /// Returns the preamble's original byte range, ending before the first `<`.
    pub fn text_span(&self) -> Range<usize> {
        self.text_span.clone()
    }
    /// Returns the byte range from the preamble's start through EOH's closing `>`.
    ///
    /// Offsets refer to the original input slice and the end is exclusive.
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    /// Borrows the cached preamble text or its decoding error without re-decoding.
    ///
    /// Add [`Self::text_span`]'s start to a decoding error's relative byte offset
    /// to locate it within the original input.
    pub fn text(&self) -> Result<&str, &DecodeError> {
        self.text.as_deref()
    }
}

/// A complete ADI record containing at least one field.
///
/// Field names are unique within the record under ASCII case-insensitive
/// comparison. Its span starts at the first descriptor and ends after EOR's
/// closing `>`, excluding surrounding ignored text and line endings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<'a> {
    span: Range<usize>,
    fields: Vec<Field<'a>>,
}

impl<'a> Record<'a> {
    /// Borrows all fields in their original order.
    pub fn fields(&self) -> &[Field<'a>] {
        &self.fields
    }
    /// Finds a field by ASCII case-insensitive name, or returns `None`.
    ///
    /// The returned field retains its original name casing.
    pub fn field(&self, name: &str) -> Option<&Field<'a>> {
        self.fields
            .iter()
            .find(|f| f.name().eq_ignore_ascii_case(name))
    }
    /// Returns the section's syntactic byte range, excluding surrounding separators.
    ///
    /// Offsets refer to the original input slice and the end is exclusive.
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }
}
