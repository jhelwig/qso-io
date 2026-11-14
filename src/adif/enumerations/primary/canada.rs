//! Canadian provinces and territories, DXCC entity 1.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Canadian province or territory in the ADIF primary subdivision table.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Nova Scotia.
    #[strum(serialize = "NS")]
    NovaScotia,
    /// Québec.
    #[strum(serialize = "QC")]
    Quebec,
    /// Ontario.
    #[strum(serialize = "ON")]
    Ontario,
    /// Manitoba.
    #[strum(serialize = "MB")]
    Manitoba,
    /// Saskatchewan.
    #[strum(serialize = "SK")]
    Saskatchewan,
    /// Alberta.
    #[strum(serialize = "AB")]
    Alberta,
    /// British Columbia.
    #[strum(serialize = "BC")]
    BritishColumbia,
    /// Northwest Territories.
    #[strum(serialize = "NT")]
    NorthwestTerritories,
    /// New Brunswick.
    #[strum(serialize = "NB")]
    NewBrunswick,
    /// Newfoundland and Labrador.
    #[strum(serialize = "NL")]
    NewfoundlandAndLabrador,
    /// Yukon.
    #[strum(serialize = "YT")]
    Yukon,
    /// Prince Edward Island.
    #[strum(serialize = "PE")]
    PrinceEdwardIsland,
    /// Nunavut.
    #[strum(serialize = "NU")]
    Nunavut,
}
