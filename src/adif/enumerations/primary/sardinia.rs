//! Sardinian subdivisions, DXCC 225, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Sardinian provinces, including import-only legacy codes.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Cagliari.
    #[strum(serialize = "CA")]
    Cagliari,
    /// Carbonia-Iglesias; import-only, replaced by SU.
    #[strum(serialize = "CI")]
    CarboniaIglesias,
    /// Sud Sardegna.
    #[strum(serialize = "SU")]
    SudSardegna,
    /// Medio Campidano; import-only.
    #[strum(serialize = "MD")]
    MedioCampidanoLegacy,
    /// Nuoro.
    #[strum(serialize = "NU")]
    Nuoro,
    /// Ogliastra.
    #[strum(serialize = "OG")]
    Ogliastra,
    /// Oristano.
    #[strum(serialize = "OR")]
    Oristano,
    /// Olbia-Tempio.
    #[strum(serialize = "OT")]
    OlbiaTempio,
    /// Sassari.
    #[strum(serialize = "SS")]
    Sassari,
    /// MedioCampidano.
    #[strum(serialize = "VS")]
    MedioCampidano,
}
