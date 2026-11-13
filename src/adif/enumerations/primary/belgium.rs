//! Belgian subdivisions, DXCC 209, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Belgian provinces and Brussels recognized by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Antwerpen.
    #[strum(serialize = "AN")]
    Antwerpen,
    /// Brussels.
    #[strum(serialize = "BR")]
    Brussels,
    /// Brabant Wallon.
    #[strum(serialize = "BW")]
    BrabantWallon,
    /// Hainaut.
    #[strum(serialize = "HT")]
    Hainaut,
    /// Limburg.
    #[strum(serialize = "LB")]
    Limburg,
    /// Liêge.
    #[strum(serialize = "LG")]
    Liege,
    /// Namur.
    #[strum(serialize = "NM")]
    Namur,
    /// Luxembourg.
    #[strum(serialize = "LU")]
    Luxembourg,
    /// Oost-Vlaanderen.
    #[strum(serialize = "OV")]
    OostVlaanderen,
    /// Vlaams Brabant.
    #[strum(serialize = "VB")]
    VlaamsBrabant,
    /// West-Vlaanderen.
    #[strum(serialize = "WV")]
    WestVlaanderen,
}
