//! Read length-delimited ADI input as complete borrowed headers and records.
//!
//! [`Parser`](crate::adif::adi::Parser) yields [`Section`](crate::adif::adi::Section)
//! values in input order. Structural failures are
//! terminal, including text-decoding failures in fields or header preambles. All spans use original byte offsets and exclusive ends.

use std::{borrow::Cow, ops::Range};

use crate::adif::Field;

pub use header::{HeaderEntry, UserDefinition, UserDefinitionConstraint, UserDefinitionError};
pub use parser::Parser;
pub use parser::error::{StructuralError, StructuralErrorKind, SyntaxErrorKind};

mod header;
mod parser;

/// A complete header or record from an ADI input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Section<'a> {
    /// A complete decoded header terminated by EOH.
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
/// separators. Its preamble and fields have successfully decoded text.
/// Field names are unique within the header under ASCII case-insensitive
/// comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    span: Range<usize>,
    text_bytes: &'a [u8],
    text_span: Range<usize>,
    text: Cow<'a, str>,
    entries: Vec<HeaderEntry<'a>>,
}

impl<'a> Header<'a> {
    /// Borrows header fields and declarations in their original order.
    pub fn entries(&self) -> &[HeaderEntry<'a>] {
        &self.entries
    }

    /// Finds an entry by its ASCII case-insensitive descriptor name.
    pub fn entry(&self, name: &str) -> Option<&HeaderEntry<'a>> {
        self.entries
            .iter()
            .find(|e| e.name().eq_ignore_ascii_case(name))
    }

    /// Finds an ordinary field; USERDEFn declarations are exposed through entries.
    pub fn field(&self, name: &str) -> Option<&Field<'a>> {
        match self.entry(name)? {
            HeaderEntry::Field(f) => Some(f),
            _ => None,
        }
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

    /// Borrows the original decoded preamble retained during parsing.
    pub fn text(&self) -> &str {
        &self.text
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
