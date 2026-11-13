//! ADIF 3.1.8 reference identities used by the parser and available to callers.
//!
//! Parsing uses ASCII case-insensitive code identity without trimming or Unicode
//! normalization. Historical records remain available independently of whether
//! their dates apply to a particular QSO.

use strum::{EnumIter, IntoEnumIterator};

mod ant_path;
mod arrl_section;
mod award;
mod geography;
/// Country-specific primary subdivision enums following ADIF III.B.12.
pub mod primary;
mod radio;
mod status;

pub use ant_path::AntPath;
pub use arrl_section::ArrlSection;
pub use award::Award;
pub use geography::{
    AlaskaCounty, DxccEntity, NzRegionDistrict, Region, SecondarySubdivision,
    SecondarySubdivisionAlt,
};
pub use radio::{AwardSponsor, Band, ContestId, Credit, Mode, Submode};
pub use status::{
    Continent, EqslAg, MorseKeyType, PropagationMode, QslMedium, QslRcvd, QslSent, QslVia,
    QsoComplete, QsoDownloadStatus, QsoUploadStatus,
};

/// An ADIF enumeration associated with a standard field or compound value.
///
/// This identifies the reference set; individual members are represented by the
/// corresponding Rust reference type, such as [`AntPath`] or [`ArrlSection`].
/// DOK and externally delegated award assignments are not supplied reference sets.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
pub enum Enumeration {
    /// Signal propagation path relative to the antenna direction.
    AntPath,
    /// Current and historical ARRL sections.
    ArrlSection,
    /// Import-only award identities.
    Award,
    /// Organizations assigning sponsored award identities.
    AwardSponsor,
    /// Amateur radio bands identified by wavelength.
    Band,
    /// Contest identifiers recommended for CONTEST_ID.
    ContestId,
    /// Continents, including IOTA continent prefixes.
    Continent,
    /// Award credits and facets used in CreditList.
    Credit,
    /// Current and deleted DXCC entity codes.
    DxccEntityCode,
    /// eQSL Authenticity Guaranteed status.
    EqslAg,
    /// Radio modes, including import-only historical identifiers.
    Mode,
    /// Morse key mechanisms and character composition.
    MorseKeyType,
    /// Primary administrative subdivisions within DXCC entities.
    PrimarySubdivision,
    /// Radio propagation mechanisms.
    PropagationMode,
    /// QSO confirmation media used in credit annotations.
    QslMedium,
    /// Receipt and verification state of a QSL.
    QslRcvd,
    /// Sending and queueing state of a QSL.
    QslSent,
    /// Routes by which QSLs are sent and received.
    QslVia,
    /// Completion state of a QSO.
    QsoComplete,
    /// Online service download status.
    QsoDownloadStatus,
    /// Online service upload status.
    QsoUploadStatus,
    /// Regions used by WAE and CQ awards, with historical applicability.
    Region,
    /// The supplied secondary-subdivision records, covering Alaska.
    SecondarySubdivision,
    /// Alternate secondary subdivisions in the supplied NZ_Regions namespace.
    SecondarySubdivisionAlt,
    /// Submodes and their associated modes.
    Submode,
}

impl Enumeration {
    /// Returns the enumeration's name in the ADIF specification.
    pub fn name(self) -> &'static str {
        match self {
            Self::AntPath => "Ant_Path",
            Self::ArrlSection => "ARRL_Section",
            Self::Award => "Award",
            Self::AwardSponsor => "Award_Sponsor",
            Self::Band => "Band",
            Self::ContestId => "Contest_ID",
            Self::Continent => "Continent",
            Self::Credit => "Credit",
            Self::DxccEntityCode => "DXCC_Entity_Code",
            Self::EqslAg => "EQSL_AG",
            Self::Mode => "Mode",
            Self::MorseKeyType => "Morse_Key_Type",
            Self::PrimarySubdivision => "Primary_Administrative_Subdivision",
            Self::PropagationMode => "Propagation_Mode",
            Self::QslMedium => "QSL_Medium",
            Self::QslRcvd => "QSL_Rcvd",
            Self::QslSent => "QSL_Sent",
            Self::QslVia => "QSL_Via",
            Self::QsoComplete => "QSO_Complete",
            Self::QsoDownloadStatus => "QSO_Download_Status",
            Self::QsoUploadStatus => "QSO_Upload_Status",
            Self::Region => "Region",
            Self::SecondarySubdivision => "Secondary_Administrative_Subdivision",
            Self::SecondarySubdivisionAlt => "Secondary_Administrative_Subdivision_Alt",
            Self::Submode => "Submode",
        }
    }

    /// Reports membership in the reference records supplied by ADIF 3.1.8.
    ///
    /// Comparison ignores ASCII case without trimming. An absent member in a
    /// partial or externally delegated set does not establish that the value is
    /// invalid. Primary subdivision codes are recognized across the country-specific
    /// enums without requiring or identifying a country.
    pub fn contains(self, value: &str) -> bool {
        match self {
            Self::AntPath => value.parse::<AntPath>().is_ok(),
            Self::ArrlSection => value.parse::<ArrlSection>().is_ok(),
            Self::Award => value.parse::<Award>().is_ok(),
            Self::AwardSponsor => value.parse::<AwardSponsor>().is_ok(),
            Self::Band => value.parse::<Band>().is_ok(),
            Self::ContestId => value.parse::<ContestId>().is_ok(),
            Self::Continent => value.parse::<Continent>().is_ok(),
            Self::Credit => value.parse::<Credit>().is_ok(),
            Self::EqslAg => value.parse::<EqslAg>().is_ok(),
            Self::Mode => value.parse::<Mode>().is_ok(),
            Self::MorseKeyType => value.parse::<MorseKeyType>().is_ok(),
            Self::PropagationMode => value.parse::<PropagationMode>().is_ok(),
            Self::QslMedium => value.parse::<QslMedium>().is_ok(),
            Self::QslRcvd => value.parse::<QslRcvd>().is_ok(),
            Self::QslSent => value.parse::<QslSent>().is_ok(),
            Self::QslVia => value.parse::<QslVia>().is_ok(),
            Self::QsoComplete => value.parse::<QsoComplete>().is_ok(),
            Self::QsoDownloadStatus => value.parse::<QsoDownloadStatus>().is_ok(),
            Self::QsoUploadStatus => value.parse::<QsoUploadStatus>().is_ok(),
            Self::Submode => value.parse::<Submode>().is_ok(),
            Self::DxccEntityCode => value.parse::<DxccEntity>().is_ok(),
            Self::PrimarySubdivision => {
                DxccEntity::iter().any(|entity| entity.recognizes_primary_subdivision(value))
            }
            Self::Region => value.parse::<Region>().is_ok(),
            Self::SecondarySubdivision => SecondarySubdivision::candidates(value)
                .any(|candidate| matches!(candidate, SecondarySubdivision::Alaska(_))),
            Self::SecondarySubdivisionAlt => matches!(
                SecondarySubdivisionAlt::parse_member(value),
                Some(SecondarySubdivisionAlt::NzRegions(_))
            ),
        }
    }
}
