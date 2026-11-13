//! Netherlands subdivisions, DXCC 263, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Netherlands provinces with separate import-only legacy codes.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Drenthe.
    #[strum(serialize = "DR")]
    Drenthe,
    /// Fryslân.
    #[strum(serialize = "FR")]
    Fryslan,
    /// Groningen.
    #[strum(serialize = "GR")]
    Groningen,
    /// Noord-Brabant.
    #[strum(serialize = "NB")]
    NoordBrabant,
    /// Overijssel.
    #[strum(serialize = "OV")]
    Overijssel,
    /// Zuid-Holland.
    #[strum(serialize = "ZH")]
    ZuidHolland,
    /// Flevoland.
    #[strum(serialize = "FL")]
    Flevoland,
    /// Gelderland; import-only legacy code GD.
    #[strum(serialize = "GD")]
    GelderlandLegacy,
    /// Gelderland.
    #[strum(serialize = "GE")]
    Gelderland,
    /// Limburg; import-only legacy code LB.
    #[strum(serialize = "LB")]
    LimburgLegacy,
    /// Limburg.
    #[strum(serialize = "LI")]
    Limburg,
    /// Noord-Holland.
    #[strum(serialize = "NH")]
    NoordHolland,
    /// Utrecht.
    #[strum(serialize = "UT")]
    Utrecht,
    /// Zeeland; import-only legacy code ZL.
    #[strum(serialize = "ZL")]
    ZeelandLegacy,
    /// Zeeland.
    #[strum(serialize = "ZE")]
    Zeeland,
}
