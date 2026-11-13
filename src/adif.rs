//! Amateur Data Interchange Format (ADIF) types and ADI parsing.
//!
//! Start with [`adi::Parser`] and [`ParseOpts`] to read ADI records while
//! preserving their original bytes and source locations. Parsing yields complete
//! headers and records. Structural errors, including text-decoding failures,
//! terminate iteration while leaving previously yielded sections available.
//!
//! Reference enums describe ADIF value categories and enumeration members.
//!
//! # Declared values
//!
//! Explicit ADI indicators select stored values. Untyped contents retain decoded
//! text. A supplied zero-length field has no value; lookup distinguishes it from
//! an absent field. Original spelling and source bytes remain available.
//!
//! ```
//! use qso_io::adif::{adi::{Parser, Section}, ParseOpts, Value};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut parser = Parser::new(b"<FREQ:6:N>14.074<NOTE:0><EOR>", ParseOpts::default());
//! let record = match parser
//!     .next()
//!     .expect("example should yield a record because it supplies fields followed by EOR")?
//! {
//!     Section::Record(record) => record,
//!     other => panic!("expected record, got {other:?}"),
//! };
//! let frequency = record
//!     .field("freq")
//!     .expect("record should contain FREQ from the supplied descriptor");
//! assert!(matches!(frequency.value(), Some(Value::Number(_))), "expected Number, got {:?}", frequency.value());
//! assert_eq!(frequency.text(), "14.074");
//! assert!(
//!     record
//!         .field("NOTE")
//!         .expect("record should contain NOTE because the example supplies <NOTE:0>")
//!         .value()
//!         .is_none()
//! );
//! assert!(record.field("MISSING").is_none());
//! # Ok(())
//! # }
//! ```
//!
//! # Declared text and encodings
//!
//! [`ParseOpts::encoding`] selects both decoding and the character
//! repertoire for declared String (`S`), MultilineString (`M`) and Enumeration
//! (`E`) values. These are qso-io's explicit compatibility rules:
//!
//! | Encoding | String and Enumeration | MultilineString |
//! | --- | --- | --- |
//! | ASCII | U+0020–U+007E | Same characters plus CRLF pairs |
//! | Latin-1 | U+0020–U+007E and U+00A0–U+00FF | Same characters plus CRLF pairs |
//! | UTF-8 | Valid Unicode except CR and LF | Same characters plus CRLF pairs |
//!
//! All modes reject lone CR/LF in MultilineString. UTF-8 permits other controls,
//! including TAB, NUL, DEL and C1; ASCII and Latin-1 use the narrower ranges
//! above. The UTF-8 compatibility rule follows ADIF's international-text
//! definitions. The `I` and `G` indicators themselves are unavailable in ADI.
//! Untyped field contents use decoding alone. USERDEF enumeration members use
//! the String rules above, retaining each member's spelling and order.
//!
//! # User-field declarations
//!
//! Headers expose ordinary fields and structured USERDEFn declarations through
//! `entries()`. A declaration describes a record field; its optional indicator
//! and parsed constraints are retained separately from its original text.
//!
//! ```
//! use qso_io::adif::{adi::{HeaderEntry, Parser, Section}, DataType, ParseOpts};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut parser = Parser::new(b"P<USERDEF1:3:N>EPC<EOH>", ParseOpts::default());
//! let header = match parser
//!     .next()
//!     .expect("example should yield a header because it starts with a preamble")?
//! {
//!     Section::Header(header) => header,
//!     other => panic!("expected header, got {other:?}"),
//! };
//! let definition = match &header.entries()[0] {
//!     HeaderEntry::UserDefinition(definition) => definition,
//!     other => panic!("expected USERDEF declaration, got {other:?}"),
//! };
//! assert_eq!(definition.field_name(), "EPC");
//! assert_eq!(definition.data_type(), Some(DataType::Number));
//! assert_eq!(definition.text(), "EPC");
//! # Ok(())
//! # }
//! ```

use std::{borrow::Cow, ops::Range, str::Utf8Error};

use chrono::NaiveDate;
use thiserror::Error;

pub use value::{AdifLocation, AdifTime, Hemisphere, TimePrecision, Value, ValueError};

/// Borrowed ADI sections, their parser, and structural diagnostics.
pub mod adi;
mod number;
mod parser;
mod text;
mod value;

/// Explicit text encoding and declared-length interpretation for one parser.
///
/// Defaults to ASCII and byte lengths. The selected settings apply to the
/// entire input.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct ParseOpts {
    /// Encoding used to decode field values and header preamble text.
    pub encoding: ParseEncoding,
    /// Unit used to interpret each descriptor's declared value length.
    pub field_length: LengthUnit,
}

/// How raw field values and preamble bytes are decoded into Unicode text.
///
/// The selected encoding also determines declared text character rules; see
/// the [declared text table](crate::adif#declared-text-and-encodings).
///
/// Latin-1 and UTF-8 provide explicit compatibility modes for non-ASCII input.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ParseEncoding {
    /// Accept bytes 0x00–0x7F; report the first byte outside that range.
    #[default]
    Ascii,
    /// Map every byte directly to U+0000–U+00FF, including C1 controls.
    ///
    /// This is ISO-8859-1/Latin-1 mapping, not Windows-1252.
    Latin1,
    /// Validate UTF-8 without replacing invalid or incomplete sequences.
    Utf8,
}

/// Unit of the value length declared in an ADI field descriptor.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum LengthUnit {
    /// Count original input bytes before attempting text decoding.
    #[default]
    Bytes,
    /// Count Unicode scalar values, rather than grapheme clusters or UTF-16 units.
    ///
    /// ASCII and Latin-1 use one byte per scalar. UTF-8 boundaries require valid
    /// scalar traversal; malformed input can therefore cause a structural error.
    UnicodeScalars,
}

/// A complete length-framed field with immutable bytes and cached text decoding.
///
/// Name casing, raw value bytes and decoded spelling are preserved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field<'a> {
    specifier: DataSpecifier<'a>,
    data_type: Option<DataType>,
    raw_data: &'a [u8],
    descriptor_span: Range<usize>,
    value_span: Range<usize>,
    text: Option<Cow<'a, str>>,
    value: Option<Value<'a>>,
}

impl<'a> Field<'a> {
    /// Returns the field name with its original casing.
    pub fn name(&self) -> &str {
        &self.specifier.name
    }

    /// Returns the descriptor's length in the unit selected by [`ParseOpts`].
    pub fn declared_length(&self) -> usize {
        self.specifier.length
    }

    /// Returns the resolved ADIF type, or None for unresolved untyped contents.
    pub fn data_type(&self) -> Option<DataType> {
        self.data_type
    }

    /// Returns the type named by the optional inline descriptor indicator.
    pub fn type_indicator(&self) -> Option<DataType> {
        self.specifier.type_indicator
    }

    /// Borrows the original value bytes.
    pub fn raw_data(&self) -> &'a [u8] {
        self.raw_data
    }

    /// Returns the original byte range from the descriptor's `<` through `>`.
    ///
    /// The end is exclusive; value bytes are not included.
    pub fn descriptor_span(&self) -> Range<usize> {
        self.descriptor_span.clone()
    }

    /// Returns the original value's byte range with an exclusive end.
    ///
    /// An empty value has an empty range at its actual input position, and spans
    /// always count bytes even when the declared length counts scalars.
    pub fn value_span(&self) -> Range<usize> {
        self.value_span.clone()
    }

    /// Borrows the original decoded spelling retained during parsing.
    pub fn text(&self) -> &str {
        match &self.value {
            Some(Value::Text(text)) => text,
            _ => self.text.as_deref().unwrap_or(""),
        }
    }

    /// Borrows the stored value, or returns None for supplied zero-length contents.
    pub fn value(&self) -> Option<&Value<'a>> {
        self.value.as_ref()
    }
}

/// Text-decoding cause retained by a located structural error.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DecodeError {
    #[error("Non-ASCII byte at offset {offset}")]
    /// A byte cannot be represented by the selected ASCII encoding.
    NonAscii {
        /// Zero-based byte offset of the first non-ASCII byte within the value
        /// or preamble, not within the whole document.
        offset: usize,
    },
    #[error("UTF-8 decode: {0}")]
    /// Invalid or incomplete UTF-8 in field data or a header preamble.
    Utf8(
        /// Standard UTF-8 error; its offsets are relative to those raw bytes.
        #[from]
        Utf8Error,
    ),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DataSpecifier<'a> {
    name: Cow<'a, str>,
    length: usize,
    type_indicator: Option<DataType>,
}

/// ADIF value categories and their optional descriptor indicators.
///
/// See [ADIF 3.1.8 data types](https://www.adif.org.uk/318/ADIF_318.htm#Data_Types).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataType {
    /// a comma-delimited list of members of the Award enumeration
    AwardList,
    /// a comma-delimited list where each list item is either:
    ///
    /// * A member of the Credit enumeration.
    /// * A member of the Credit enumeration followed by a colon and an
    ///   ampersand-delimited list of members of the QSL_Medium enumeration.
    ///
    /// For example IOTA,WAS:LOTW&CARD,DXCC:CARD
    CreditList,
    /// a comma-delimited list of members of the Sponsored_Award enumeration
    SponsoredAwardList,
    /// * if True, the single ASCII character Y or y
    /// * if False, the single ASCII character N or n
    Boolean,
    /// an ASCII character whose code lies in the range of 48 through 57,
    /// inclusive
    Digit,
    /// a sequence of one or more Digits representing a decimal integer,
    /// optionally preceded by a minus sign (ASCII code 45).  Leading zeroes are
    /// allowed.
    Integer,
    /// a sequence of one or more Digits representing a decimal number,
    /// optionally preceded by a minus sign (ASCII code 45) and optionally
    /// including a single decimal point  (ASCII  code 46)
    Number,
    /// an unsigned sequence of one or more Digits representing a decimal
    /// integer that has a value greater than 0.  Leading zeroes are allowed.
    PositiveInteger,
    /// an ASCII character whose code lies in the range of 32 through 126,
    /// inclusive
    Character,
    /// a Unicode character (encoded with UTF-8) excluding line break CR
    /// (code 13) and LF (code 10) characters
    IntlCharacter,
    /// 8 Digits representing a UTC date in YYYYMMDD format, where
    ///
    /// * YYYY is a 4-Digit year specifier, where 1930 <= YYYY
    /// * MM is a 2-Digit month specifier, where 1 <= MM <= 12 [use leading
    ///   zeroes]
    /// * DD is a 2-Digit day specifier, where 1 <= DD <= DaysInMonth(MM)  [use
    ///   leading zeroes]
    Date,
    /// 6 Digits representing a UTC time in HHMMSS format or 4 Digits
    /// representing a time in HHMM format, where
    ///
    /// * HH is a 2-Digit hour specifier, where 0 <= HH <= 23  [use leading
    ///   zeroes]
    /// * MM is a 2-Digit minute specifier, where 0 <= MM <= 59  [use leading
    ///   zeroes]
    /// * SS is a 2-Digit second specifier, where 0 <= SS <= 59  [use leading
    ///   zeroes]
    Time,
    /// IOTA designator, in format CC-XXX, where
    ///
    /// * CC is a member of the Continent enumeration
    /// * XXX is the island group designator, where 1 <= XXX <= 999  [use
    ///   leading zeroes]
    IOTARefNo,
    /// a sequence of Characters
    String,
    /// a sequence of International Characters.  Fields of type IntlString must
    /// only be used in ADX files
    IntlString,
    /// a sequence of Characters and line-breaks, where a line break is an ASCII
    /// CR (code 13) followed immediately by an ASCII LF (code 10)
    MultilineString,
    /// a sequence of International Characters and line breaks.  Fields of type
    /// IntlMultilineString must only be used in ADX files
    IntlMultilineString,
    /// an explicit list of legal case-insensitive values represented in ASCII
    /// set forth in set notation, e.g. {A, B, C, D}, or defined in a table,
    /// from which a single value may be selected.
    Enumeration,
    /// a case-insensitive 2-character, 4-character, 6-character, or 8-character
    /// Maidenhead locator.
    ///
    /// Specific fields impose additional restrictions on the number of
    /// characters; see the field descriptions for the allowed numbers of
    /// characters.
    GridSquare,
    /// For a 10-character Maidenhead locator, contains characters 9 and 10.
    /// For a 12-character Maidenhead locator, contains characters 9, 10, 11 and
    /// 12.
    ///
    /// Characters 9 and 10 are case-insensitive ASCII letters in the range A-X.
    /// Characters 11 and 12 are Digits in the range 0-9.
    GridSquareExt,
    /// a comma-delimited list of GridSquare items
    GridSquareList,
    /// a sequence of 11 characters representing a latitude or longitude in XDDD
    /// MM.MMM format, where
    ///
    /// * X is a directional Character from the set {E, W, N, S}
    /// * DDD is a 3-Digit degrees specifier, where 0 <= DDD <= 180 [use leading
    ///   zeroes]
    /// * There is a single space character in between DDD and MM.MMM
    /// * MM.MMM is an unsigned Number minutes specifier with its decimal point
    ///   in the third position, where 00.000 <= MM.MMM <= 59.999  [use leading
    ///   and trailing zeroes]
    Location,
    /// a sequence of case-insensitive Characters representing a Parks on the
    /// Air park reference in the form `xxxx-nnnnn[@yyyyyy]` comprising 6 to 17
    /// characters where:

    /// * xxxx is the POTA national program and is 1 to 4 characters in length,
    ///   typically the default callsign prefix of the national program (rather
    ///   than the DX entity)
    /// * nnnnn represents the unique number within the national program and is
    ///   either 4 or 5 characters in length (use the exact format listed on the
    ///   POTA website)
    /// * yyyyyy **Optional** is the 4 to 6 character ISO 3166-2 code to
    ///   differentiate which state/province/prefecture/primary administration
    ///   location the contact represents, in the case that the park reference
    ///   spans more than one location (such as a trail).
    POTARef,
    /// a comma-delimited list of one or more POTARef items.
    POTARefList,
    /// a colon-delimited list of two or more members of the
    /// `Secondary_Administrative_Subdivision` enumeration.  E.g.:
    ///
    /// `MA,Franklin:MA,Hampshire`
    SecondarySubdivisionList,
    /// An unordered, semicolon-delimited list of one or more alternate secondary
    /// administrative subdivisions, each written as `enumeration-name:enumeration-code`.
    ///
    /// A code covering multiple localities separates them with `/`. Each
    /// enumeration name may occur only once and must apply to the DXCC entity.
    /// For example: `NZ_Regions:Hawkes Bay/Wairoa`.
    ///
    /// Used by `CNTY_ALT` and `MY_CNTY_ALT`; this type has no descriptor indicator.
    /// See [ADIF 3.1.8 data types](https://www.adif.org.uk/318/ADIF_318.htm#Data_Types).
    SecondaryAdministrativeSubdivisionListAlt,
    /// An International SOTA reference formed from an ITU prefix, an optional
    /// SOTA subdivision, `/`, and a SOTA reference number, in that order.
    ///
    /// Examples: `W2/WE-003`, `G/LD-003`.
    SOTARef,
    /// A case-insensitive International WWFF reference using `xxFF-nnnn`.
    ///
    /// The national program identifier `xx` occupies 1–4 characters, followed
    /// by the literal `FF-` and a four-digit program-local number padded with
    /// leading zeros. The complete reference occupies 8–11 characters.
    ///
    /// Examples: `KFF-4655`, `3DAFF-0002`.
    WWFFRef,
}

impl DataType {
    /// Reports whether this data type is defined only for importing legacy data.
    ///
    /// Returns `true` for [`Self::AwardList`] and `false` for other types.
    pub fn import_only(self) -> bool {
        self == DataType::AwardList
    }

    /// Returns the uppercase ADIF indicator, or `None` for types without one.
    ///
    /// The international string indicators `I` and `G` designate ADX types.
    pub fn type_indicator(self) -> Option<&'static str> {
        match self {
            DataType::Boolean => Some("B"),
            DataType::Number => Some("N"),
            DataType::Date => Some("D"),
            DataType::Time => Some("T"),
            DataType::String => Some("S"),
            DataType::IntlString => Some("I"),
            DataType::MultilineString => Some("M"),
            DataType::IntlMultilineString => Some("G"),
            DataType::Enumeration => Some("E"),
            DataType::Location => Some("L"),
            DataType::AwardList
            | DataType::CreditList
            | DataType::SponsoredAwardList
            | DataType::Digit
            | DataType::Integer
            | DataType::PositiveInteger
            | DataType::Character
            | DataType::IntlCharacter
            | DataType::IOTARefNo
            | DataType::GridSquare
            | DataType::GridSquareExt
            | DataType::GridSquareList
            | DataType::POTARef
            | DataType::POTARefList
            | DataType::SecondarySubdivisionList
            | DataType::SecondaryAdministrativeSubdivisionListAlt
            | DataType::SOTARef
            | DataType::WWFFRef => None,
        }
    }
}

/// Failure to recognize or decode a supplied ADIF type indicator.
#[derive(Debug, Error)]
pub enum DataTypeError {
    #[error("Unknown type specifier: {0}")]
    /// The supplied text is not a recognized single type indicator.
    UnknownTypeSpecifier(
        /// Original unrecognized indicator text, including its supplied casing.
        String,
    ),
    #[error("UTF-8 error: {0}")]
    /// The bytes used for an indicator could not be decoded as UTF-8.
    Utf8(
        /// Standard error retaining the invalid indicator bytes.
        #[from]
        std::string::FromUtf8Error,
    ),
}

impl TryFrom<&str> for DataType {
    type Error = DataTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "B" | "b" => Ok(DataType::Boolean),
            "N" | "n" => Ok(DataType::Number),
            "D" | "d" => Ok(DataType::Date),
            "T" | "t" => Ok(DataType::Time),
            "S" | "s" => Ok(DataType::String),
            "I" | "i" => Ok(DataType::IntlString),
            "M" | "m" => Ok(DataType::MultilineString),
            "G" | "g" => Ok(DataType::IntlMultilineString),
            "E" | "e" => Ok(DataType::Enumeration),
            "L" | "l" => Ok(DataType::Location),
            u => Err(DataTypeError::UnknownTypeSpecifier(u.to_owned())),
        }
    }
}

/// ADIF antenna-path codes describing how the signal propagated.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AntPath {
    /// Grayline
    G,
    /// Other
    O,
    /// Short path
    S,
    /// Long path
    L,
}

/// A partial reference list of ARRL section identifiers and their metadata.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ArrlSection {
    /// Alabama.
    AL,
    /// Alaska.
    AK,
    /// Alberta.
    AB,
    /// Arkansas.
    AR,
    /// Arizona.
    AZ,
    /// British Columbia.
    BC,
    /// Colorado.
    CO,
}

impl ArrlSection {
    /// Returns the human-readable section name.
    pub fn name(self) -> &'static str {
        match self {
            ArrlSection::AL => "Alabama",
            ArrlSection::AK => "Alaska",
            ArrlSection::AB => "Alberta",
            ArrlSection::AR => "Arkansas",
            ArrlSection::AZ => "Arizona",
            ArrlSection::BC => "British Columbia",
            ArrlSection::CO => "Colorado",
        }
    }

    /// Returns the associated ADIF DXCC entity code.
    pub fn dxcc_entity_code(self) -> u16 {
        match self {
            ArrlSection::AL | ArrlSection::AR | ArrlSection::AZ | ArrlSection::CO => 291,
            ArrlSection::AK => 6,
            ArrlSection::AB => 1,
            ArrlSection::BC => 1,
        }
    }

    /// Returns the section's From Date from the ADIF enumeration, or `None`
    /// when the specification leaves it unspecified.
    pub fn from_date(self) -> Option<NaiveDate> {
        match self {
            ArrlSection::AL
            | ArrlSection::AK
            | ArrlSection::AB
            | ArrlSection::AR
            | ArrlSection::AZ
            | ArrlSection::BC
            | ArrlSection::CO => None,
        }
    }

    /// Returns the section's Deleted Date from the ADIF enumeration, or `None`
    /// when the specification leaves it unspecified.
    pub fn deleted_date(self) -> Option<NaiveDate> {
        match self {
            ArrlSection::AL
            | ArrlSection::AK
            | ArrlSection::AB
            | ArrlSection::AR
            | ArrlSection::AZ
            | ArrlSection::BC
            | ArrlSection::CO => None,
        }
    }
}

/// Award identifiers from ADIF's import-only Award enumeration.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Award {
    /// ADIF `AJA` identifier (import-only).
    Aja,
    /// ADIF `CQDX` identifier (import-only).
    Cqdx,
    /// ADIF `CQDXFIELD` identifier (import-only).
    Cqdxfield,
    /// ADIF `CQWAZ_MIXED` identifier (import-only).
    CqwazMixed,
    /// ADIF `CQWAZ_CW` identifier (import-only).
    CqwazCw,
    /// ADIF `CQWAZ_PHONE` identifier (import-only).
    CqwazPhone,
    /// ADIF `CQWAZ_RTTY` identifier (import-only).
    CqwazRtty,
    /// ADIF `CQWAZ_160m` identifier (import-only).
    Cqwaz160m,
    /// ADIF `CQWPX` identifier (import-only).
    Cqwpx,
    /// ADIF `DARC_DOK` identifier (import-only).
    DarcDok,
    /// ADIF `DXCC` identifier (import-only).
    Dxcc,
    /// ADIF `DXCC_MIXED` identifier (import-only).
    DxccMixed,
    /// ADIF `DXCC_CW` identifier (import-only).
    DxccCw,
    /// ADIF `DXCC_PHONE` identifier (import-only).
    DxccPhone,
    /// ADIF `DXCC_RTTY` identifier (import-only).
    DxccRtty,
    /// ADIF `IOTA` identifier (import-only).
    Iota,
    /// ADIF `JCC` identifier (import-only).
    Jcc,
    /// ADIF `JCG` identifier (import-only).
    Jcg,
    /// ADIF `MARATHON` identifier (import-only).
    Marathon,
    /// ADIF `RDA` identifier (import-only).
    Rda,
    /// ADIF `WAB` identifier (import-only).
    Wab,
    /// ADIF `WAC` identifier (import-only).
    Wac,
    /// ADIF `WAE` identifier (import-only).
    Wae,
    /// ADIF `WAIP` identifier (import-only).
    Waip,
    /// ADIF `WAJA` identifier (import-only).
    Waja,
    /// ADIF `WAS` identifier (import-only).
    Was,
    /// ADIF `WAZ` identifier (import-only).
    Waz,
    /// ADIF `USACA` identifier (import-only).
    Usaca,
    /// ADIF `VUCC` identifier (import-only).
    Vucc,
}
