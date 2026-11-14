//! Australian primary subdivisions, DXCC entity 150.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// An Australian state or territory supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Australian Capital Territory.
    #[strum(serialize = "ACT")]
    AustralianCapitalTerritory,
    /// New South Wales.
    #[strum(serialize = "NSW")]
    NewSouthWales,
    /// Victoria.
    #[strum(serialize = "VIC")]
    Victoria,
    /// Queensland.
    #[strum(serialize = "QLD")]
    Queensland,
    /// South Australia.
    #[strum(serialize = "SA")]
    SouthAustralia,
    /// Western Australia.
    #[strum(serialize = "WA")]
    WesternAustralia,
    /// Tasmania.
    #[strum(serialize = "TAS")]
    Tasmania,
    /// Northern Territory.
    #[strum(serialize = "NT")]
    NorthernTerritory,
}
