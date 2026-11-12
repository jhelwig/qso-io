use std::{borrow::Cow, ops::Range};

use crate::adif::{DecodeError, Field};

mod parser;

pub use parser::Parser;
pub use parser::error::{StructuralError, StructuralErrorKind, SyntaxErrorKind};

/// A complete header or record from an ADI input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Section<'a> {
    Header(Header<'a>),
    Record(Record<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdiFile<'a> {
    header: Option<Header<'a>>,
    records: Vec<Record<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    span: Range<usize>,
    text_bytes: &'a [u8],
    text_span: Range<usize>,
    text: Result<Cow<'a, str>, DecodeError>,
    fields: Vec<Field<'a>>,
}

impl<'a> Header<'a> {
    pub fn fields(&self) -> &[Field<'a>] {
        &self.fields
    }
    pub fn field(&self, name: &str) -> Option<&Field<'a>> {
        self.fields
            .iter()
            .find(|f| f.name().eq_ignore_ascii_case(name))
    }
    pub fn raw_text(&self) -> &'a [u8] {
        self.text_bytes
    }
    pub fn text_span(&self) -> Range<usize> {
        self.text_span.clone()
    }
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    pub fn text(&self) -> Result<&str, &DecodeError> {
        self.text.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<'a> {
    span: Range<usize>,
    fields: Vec<Field<'a>>,
}

impl<'a> Record<'a> {
    pub fn fields(&self) -> &[Field<'a>] {
        &self.fields
    }
    pub fn field(&self, name: &str) -> Option<&Field<'a>> {
        self.fields
            .iter()
            .find(|f| f.name().eq_ignore_ascii_case(name))
    }
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }
}
