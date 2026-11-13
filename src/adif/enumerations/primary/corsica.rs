//! Corsican subdivisions, DXCC 214, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Corsican departments recognized by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Corse-du-Sud.
    #[strum(serialize = "2A")]
    CorseDuSud,
    /// Haute-Corse.
    #[strum(serialize = "2B")]
    HauteCorse,
}
