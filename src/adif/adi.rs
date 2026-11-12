use std::{borrow::Cow, ops::Range};

use crate::adif::{DecodeError, Field};

mod parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdiFile<'a> {
    header: Option<Header<'a>>,
    records: Vec<Record<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    text_bytes: &'a [u8],
    text_span: Range<usize>,
    text: Result<Cow<'a, str>, DecodeError>,
    fields: Vec<Field<'a>>,
}

impl<'a> Header<'a> {
    pub fn text(&self) -> Result<&str, &DecodeError> {
        self.text.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<'a> {
    fields: Vec<Field<'a>>,
}
