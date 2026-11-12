use std::{borrow::Cow, ops::Range, str::Utf8Error};

use chrono::NaiveDate;
use thiserror::Error;

pub mod adi;
mod parser;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct ParseOpts {
    encoding: ParseEncoding,
    field_length: LengthUnit,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ParseEncoding {
    #[default]
    Ascii,
    Latin1,
    Utf8,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum LengthUnit {
    #[default]
    Bytes,
    UnicodeScalars,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field<'a> {
    specifier: DataSpecifier<'a>,
    raw_data: &'a [u8],
    value_span: Range<usize>,
    text: Result<Cow<'a, str>, DecodeError>,
}

impl<'a> Field<'a> {
    pub fn text(&self) -> Result<&str, &DecodeError> {
        self.text.as_deref()
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DecodeError {
    #[error("Non-ASCII byte at offset {offset}")]
    NonAscii { offset: usize },
    #[error("UTF-8 decode: {0}")]
    Utf8(#[from] Utf8Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DataSpecifier<'a> {
    name: Cow<'a, str>,
    length: usize,
    data_type: Option<DataType>,
}

/// https://adif.org.uk/318/ADIF_318.htm#Data_Types_Enumerations_and_Fields
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
    /// Air park reference in the form xxxx-nnnnn[@yyyyyy] comprising 6 to 17
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
    Secondary,
    Administrative,
    SubdivisionListAlt,
    SOTARef,
    WWFFRef,
}

impl DataType {
    /// Deprecated fields and enumeration values shall be accepted when
    /// importing an ADIF file, but shall not be emitted when exporting
    /// an ADIF file.  Deprecated items in this specification are characterized
    /// as "Import-only".
    pub fn import_only(self) -> bool {
        self == DataType::AwardList
    }

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
            | DataType::Secondary
            | DataType::Administrative
            | DataType::SubdivisionListAlt
            | DataType::SOTARef
            | DataType::WWFFRef => None,
        }
    }
}

#[derive(Debug, Error)]
pub enum DataTypeError {
    #[error("Unknown type specifier: {0}")]
    UnknownTypeSpecifier(String),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ArrlSection {
    AL,
    AK,
    AB,
    AR,
    AZ,
    BC,
    CO,
}

impl ArrlSection {
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

    pub fn dxcc_entity_code(self) -> u16 {
        match self {
            ArrlSection::AL | ArrlSection::AR | ArrlSection::AZ | ArrlSection::CO => 291,
            ArrlSection::AK => 6,
            ArrlSection::AB => 1,
            ArrlSection::BC => 1,
        }
    }

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Award {
    Aja,
    Cqdx,
    Cqdxfield,
    CqwazMixed,
    CqwazCw,
    CqwazPhone,
    CqwazRtty,
    Cqwaz160m,
    Cqwpx,
    DarcDok,
    Dxcc,
    DxccMixed,
    DxccCw,
    DxccPhone,
    DxccRtty,
    Iota,
    Jcc,
    Jcg,
    Marathon,
    Rda,
    Wab,
    Wac,
    Wae,
    Waip,
    Waja,
    Was,
    Waz,
    Usaca,
    Vucc,
}
