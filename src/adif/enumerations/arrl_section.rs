//! ARRL sections, including historical identities, from ADIF III.B.2.

use strum::{EnumIter, EnumString, IntoStaticStr};

use chrono::NaiveDate;

/// An ARRL section used in contest exchanges and award records.
///
/// Historical identities remain distinct from their replacements. Some sections
/// cover several DXCC entities, and entries may have effective or deletion dates.
/// See [ADIF III.B.2](https://www.adif.org.uk/318/ADIF_318.htm#ARRL_Section_Enumeration).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum ArrlSection {
    /// Alabama.
    #[strum(serialize = "AL")]
    AL,
    /// Alaska.
    #[strum(serialize = "AK")]
    AK,
    /// Alberta.
    #[strum(serialize = "AB")]
    AB,
    /// Arkansas.
    #[strum(serialize = "AR")]
    AR,
    /// Arizona.
    #[strum(serialize = "AZ")]
    AZ,
    /// British Columbia.
    #[strum(serialize = "BC")]
    BC,
    /// Colorado.
    #[strum(serialize = "CO")]
    CO,
    /// Connecticut.
    #[strum(serialize = "CT")]
    CT,
    /// Delaware.
    #[strum(serialize = "DE")]
    DE,
    /// East Bay.
    #[strum(serialize = "EB")]
    EB,
    /// Eastern Massachusetts.
    #[strum(serialize = "EMA")]
    EMA,
    /// Eastern New York.
    #[strum(serialize = "ENY")]
    ENY,
    /// Eastern Pennsylvania.
    #[strum(serialize = "EPA")]
    EPA,
    /// Eastern Washington.
    #[strum(serialize = "EWA")]
    EWA,
    /// Georgia.
    #[strum(serialize = "GA")]
    GA,
    /// Golden Horseshoe, replacing Greater Toronto Area from 2023-01-01.
    #[strum(serialize = "GH")]
    GH,
    /// Greater Toronto Area, replaced by Golden Horseshoe on 2023-01-01.
    #[strum(serialize = "GTA")]
    GTA,
    /// Idaho.
    #[strum(serialize = "ID")]
    ID,
    /// Illinois.
    #[strum(serialize = "IL")]
    IL,
    /// Indiana.
    #[strum(serialize = "IN")]
    IN,
    /// Iowa.
    #[strum(serialize = "IA")]
    IA,
    /// Kansas.
    #[strum(serialize = "KS")]
    KS,
    /// Kentucky.
    #[strum(serialize = "KY")]
    KY,
    /// Los Angeles.
    #[strum(serialize = "LAX")]
    LAX,
    /// Louisiana.
    #[strum(serialize = "LA")]
    LA,
    /// Maine.
    #[strum(serialize = "ME")]
    ME,
    /// Manitoba.
    #[strum(serialize = "MB")]
    MB,
    /// Maritime, replaced by New Brunswick and Nova Scotia on 2023-01-01.
    #[strum(serialize = "MAR")]
    MAR,
    /// Maryland-DC.
    #[strum(serialize = "MDC")]
    MDC,
    /// Michigan.
    #[strum(serialize = "MI")]
    MI,
    /// Minnesota.
    #[strum(serialize = "MN")]
    MN,
    /// Mississippi.
    #[strum(serialize = "MS")]
    MS,
    /// Missouri.
    #[strum(serialize = "MO")]
    MO,
    /// Montana.
    #[strum(serialize = "MT")]
    MT,
    /// Nebraska.
    #[strum(serialize = "NE")]
    NE,
    /// Nevada.
    #[strum(serialize = "NV")]
    NV,
    /// New Brunswick, effective 2023-01-01.
    #[strum(serialize = "NB")]
    NB,
    /// New Hampshire.
    #[strum(serialize = "NH")]
    NH,
    /// New Mexico.
    #[strum(serialize = "NM")]
    NM,
    /// New York City-Long Island.
    #[strum(serialize = "NLI")]
    NLI,
    /// Newfoundland/Labrador.
    #[strum(serialize = "NL")]
    NL,
    /// North Carolina.
    #[strum(serialize = "NC")]
    NC,
    /// North Dakota.
    #[strum(serialize = "ND")]
    ND,
    /// North Texas.
    #[strum(serialize = "NTX")]
    NTX,
    /// Northern Florida.
    #[strum(serialize = "NFL")]
    NFL,
    /// Northern New Jersey.
    #[strum(serialize = "NNJ")]
    NNJ,
    /// Northern New York.
    #[strum(serialize = "NNY")]
    NNY,
    /// Northwest Territories/Yukon/Nunavut, replacing NWT and later replaced by TER.
    #[strum(serialize = "NT")]
    NT,
    /// Northwest Territories/Yukon/Nunavut, replaced by NT on 2003-11-01.
    #[strum(serialize = "NWT")]
    NWT,
    /// Nova Scotia, effective 2023-01-01.
    #[strum(serialize = "NS")]
    NS,
    /// Ohio.
    #[strum(serialize = "OH")]
    OH,
    /// Oklahoma.
    #[strum(serialize = "OK")]
    OK,
    /// Ontario, replaced by GTA, ONE, ONN and ONS on 2012-09-01.
    #[strum(serialize = "ON")]
    ON,
    /// Ontario East, effective 2012-09-01.
    #[strum(serialize = "ONE")]
    ONE,
    /// Ontario North, effective 2012-09-01.
    #[strum(serialize = "ONN")]
    ONN,
    /// Ontario South, effective 2012-09-01.
    #[strum(serialize = "ONS")]
    ONS,
    /// Orange.
    #[strum(serialize = "ORG")]
    ORG,
    /// Oregon.
    #[strum(serialize = "OR")]
    OR,
    /// Pacific; covers multiple DXCC entities.
    #[strum(serialize = "PAC")]
    PAC,
    /// Prince Edward Island, effective 2020-04-01.
    #[strum(serialize = "PE")]
    PE,
    /// Puerto Rico; includes Desecheo Island.
    #[strum(serialize = "PR")]
    PR,
    /// Quebec.
    #[strum(serialize = "QC")]
    QC,
    /// Rhode Island.
    #[strum(serialize = "RI")]
    RI,
    /// Sacramento Valley.
    #[strum(serialize = "SV")]
    SV,
    /// San Diego.
    #[strum(serialize = "SDG")]
    SDG,
    /// San Francisco.
    #[strum(serialize = "SF")]
    SF,
    /// San Joaquin Valley.
    #[strum(serialize = "SJV")]
    SJV,
    /// Santa Barbara.
    #[strum(serialize = "SB")]
    SB,
    /// Santa Clara Valley.
    #[strum(serialize = "SCV")]
    SCV,
    /// Saskatchewan.
    #[strum(serialize = "SK")]
    SK,
    /// South Carolina.
    #[strum(serialize = "SC")]
    SC,
    /// South Dakota.
    #[strum(serialize = "SD")]
    SD,
    /// South Texas.
    #[strum(serialize = "STX")]
    STX,
    /// Southern Florida.
    #[strum(serialize = "SFL")]
    SFL,
    /// Southern New Jersey.
    #[strum(serialize = "SNJ")]
    SNJ,
    /// Tennessee.
    #[strum(serialize = "TN")]
    TN,
    /// Territories, replacing NT from 2023-01-01.
    #[strum(serialize = "TER")]
    TER,
    /// US Virgin Islands; covers multiple DXCC entities.
    #[strum(serialize = "VI")]
    VI,
    /// Utah.
    #[strum(serialize = "UT")]
    UT,
    /// Vermont.
    #[strum(serialize = "VT")]
    VT,
    /// Virginia.
    #[strum(serialize = "VA")]
    VA,
    /// West Central Florida.
    #[strum(serialize = "WCF")]
    WCF,
    /// West Texas.
    #[strum(serialize = "WTX")]
    WTX,
    /// West Virginia.
    #[strum(serialize = "WV")]
    WV,
    /// Western Massachusetts.
    #[strum(serialize = "WMA")]
    WMA,
    /// Western New York.
    #[strum(serialize = "WNY")]
    WNY,
    /// Western Pennsylvania.
    #[strum(serialize = "WPA")]
    WPA,
    /// Western Washington.
    #[strum(serialize = "WWA")]
    WWA,
    /// Wisconsin.
    #[strum(serialize = "WI")]
    WI,
    /// Wyoming.
    #[strum(serialize = "WY")]
    WY,
}

impl ArrlSection {
    /// Returns the canonical ADIF section abbreviation.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// Returns the section's human-readable name, independently of replacement notes.
    pub fn name(self) -> &'static str {
        match self {
            Self::AL => "Alabama",
            Self::AK => "Alaska",
            Self::AB => "Alberta",
            Self::AR => "Arkansas",
            Self::AZ => "Arizona",
            Self::BC => "British Columbia",
            Self::CO => "Colorado",
            Self::CT => "Connecticut",
            Self::DE => "Delaware",
            Self::EB => "East Bay",
            Self::EMA => "Eastern Massachusetts",
            Self::ENY => "Eastern New York",
            Self::EPA => "Eastern Pennsylvania",
            Self::EWA => "Eastern Washington",
            Self::GA => "Georgia",
            Self::GH => "Golden Horseshoe",
            Self::GTA => "Greater Toronto Area",
            Self::ID => "Idaho",
            Self::IL => "Illinois",
            Self::IN => "Indiana",
            Self::IA => "Iowa",
            Self::KS => "Kansas",
            Self::KY => "Kentucky",
            Self::LAX => "Los Angeles",
            Self::LA => "Louisiana",
            Self::ME => "Maine",
            Self::MB => "Manitoba",
            Self::MAR => "Maritime",
            Self::MDC => "Maryland-DC",
            Self::MI => "Michigan",
            Self::MN => "Minnesota",
            Self::MS => "Mississippi",
            Self::MO => "Missouri",
            Self::MT => "Montana",
            Self::NE => "Nebraska",
            Self::NV => "Nevada",
            Self::NB => "New Brunswick",
            Self::NH => "New Hampshire",
            Self::NM => "New Mexico",
            Self::NLI => "New York City-Long Island",
            Self::NL => "Newfoundland/Labrador",
            Self::NC => "North Carolina",
            Self::ND => "North Dakota",
            Self::NTX => "North Texas",
            Self::NFL => "Northern Florida",
            Self::NNJ => "Northern New Jersey",
            Self::NNY => "Northern New York",
            Self::NT | Self::NWT => "Northwest Territories/Yukon/Nunavut",
            Self::NS => "Nova Scotia",
            Self::OH => "Ohio",
            Self::OK => "Oklahoma",
            Self::ON => "Ontario",
            Self::ONE => "Ontario East",
            Self::ONN => "Ontario North",
            Self::ONS => "Ontario South",
            Self::ORG => "Orange",
            Self::OR => "Oregon",
            Self::PAC => "Pacific",
            Self::PE => "Prince Edward Island",
            Self::PR => "Puerto Rico",
            Self::QC => "Quebec",
            Self::RI => "Rhode Island",
            Self::SV => "Sacramento Valley",
            Self::SDG => "San Diego",
            Self::SF => "San Francisco",
            Self::SJV => "San Joaquin Valley",
            Self::SB => "Santa Barbara",
            Self::SCV => "Santa Clara Valley",
            Self::SK => "Saskatchewan",
            Self::SC => "South Carolina",
            Self::SD => "South Dakota",
            Self::STX => "South Texas",
            Self::SFL => "Southern Florida",
            Self::SNJ => "Southern New Jersey",
            Self::TN => "Tennessee",
            Self::TER => "Territories",
            Self::VI => "US Virgin Islands",
            Self::UT => "Utah",
            Self::VT => "Vermont",
            Self::VA => "Virginia",
            Self::WCF => "West Central Florida",
            Self::WTX => "West Texas",
            Self::WV => "West Virginia",
            Self::WMA => "Western Massachusetts",
            Self::WNY => "Western New York",
            Self::WPA => "Western Pennsylvania",
            Self::WWA => "Western Washington",
            Self::WI => "Wisconsin",
            Self::WY => "Wyoming",
        }
    }

    /// Returns all DXCC entity codes associated with the section in ADIF.
    pub fn dxcc_entity_codes(self) -> &'static [u16] {
        match self {
            Self::AK => &[6],
            Self::AB
            | Self::BC
            | Self::GH
            | Self::GTA
            | Self::MB
            | Self::MAR
            | Self::NB
            | Self::NL
            | Self::NT
            | Self::NWT
            | Self::NS
            | Self::ON
            | Self::ONE
            | Self::ONN
            | Self::ONS
            | Self::PE
            | Self::QC
            | Self::SK
            | Self::TER => &[1],
            Self::PAC => &[9, 20, 103, 110, 123, 138, 166, 174, 197, 297, 515],
            Self::PR => &[43, 202],
            Self::VI => &[105, 182, 285],
            _ => &[291],
        }
    }

    /// Returns ADIF's From Date, or None when the table leaves it unspecified.
    pub fn from_date(self) -> Option<NaiveDate> {
        match self {
            Self::GH | Self::NB | Self::NS | Self::TER => NaiveDate::from_ymd_opt(2023, 1, 1),
            Self::GTA | Self::ONE | Self::ONN | Self::ONS => NaiveDate::from_ymd_opt(2012, 9, 1),
            Self::NT => NaiveDate::from_ymd_opt(2003, 11, 1),
            Self::PE => NaiveDate::from_ymd_opt(2020, 4, 1),
            _ => None,
        }
    }

    /// Returns ADIF's Deleted Date, or None when the table leaves it unspecified.
    pub fn deleted_date(self) -> Option<NaiveDate> {
        match self {
            Self::GTA | Self::MAR | Self::NT => NaiveDate::from_ymd_opt(2023, 1, 1),
            Self::NWT => NaiveDate::from_ymd_opt(2003, 11, 1),
            Self::ON => NaiveDate::from_ymd_opt(2012, 9, 1),
            _ => None,
        }
    }
}
