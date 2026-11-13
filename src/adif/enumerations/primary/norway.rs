//! Norwegian subdivisions, DXCC 266, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Norwegian counties listed in ADIF 3.1.8.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Agder.
    #[strum(serialize = "42")]
    Agder,
    /// Innlandet.
    #[strum(serialize = "34")]
    Innlandet,
    /// Møre og Romsdal.
    #[strum(serialize = "15")]
    MoreOgRomsdal,
    /// Nordland.
    #[strum(serialize = "18")]
    Nordland,
    /// Oslo.
    #[strum(serialize = "03")]
    Oslo,
    /// Rogaland.
    #[strum(serialize = "11")]
    Rogaland,
    /// Troms og Finnmark.
    #[strum(serialize = "54")]
    TromsOgFinnmark,
    /// Trøndelag.
    #[strum(serialize = "50")]
    Trondelag,
    /// Vestfold og Telemark.
    #[strum(serialize = "38")]
    VestfoldOgTelemark,
    /// Vestland.
    #[strum(serialize = "46")]
    Vestland,
    /// Viken.
    #[strum(serialize = "30")]
    Viken,
}
