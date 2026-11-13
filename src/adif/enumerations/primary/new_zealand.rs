//! New Zealand primary subdivisions, DXCC entity 170.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A New Zealand primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Auckland.
    #[strum(serialize = "AUK")]
    Auckland,
    /// Bay of Plenty.
    #[strum(serialize = "BOP")]
    BayOfPlenty,
    /// Northland.
    #[strum(serialize = "NTL")]
    Northland,
    /// Waikato.
    #[strum(serialize = "WKO")]
    Waikato,
    /// Gisborne.
    #[strum(serialize = "GIS")]
    Gisborne,
    /// Hawke's Bay.
    #[strum(serialize = "HKB")]
    HawkesBay,
    /// Manawatū-Whanganui.
    #[strum(serialize = "MWT")]
    ManawatuWhanganui,
    /// Taranaki.
    #[strum(serialize = "TKI")]
    Taranaki,
    /// Greater Wellington.
    #[strum(serialize = "WGN")]
    GreaterWellington,
    /// Canterbury.
    #[strum(serialize = "CAN")]
    Canterbury,
    /// Marlborough.
    #[strum(serialize = "MBH")]
    Marlborough,
    /// Nelson.
    #[strum(serialize = "NSN")]
    Nelson,
    /// Tasman.
    #[strum(serialize = "TAS")]
    Tasman,
    /// West Coast.
    #[strum(serialize = "WTC")]
    WestCoast,
    /// Otago.
    #[strum(serialize = "OTA")]
    Otago,
    /// Southland.
    #[strum(serialize = "STL")]
    Southland,
}
