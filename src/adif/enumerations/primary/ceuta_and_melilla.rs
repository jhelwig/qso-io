//! Ceuta and Melilla primary subdivisions, DXCC entity 32.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Ceuta and Melilla primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Ceuta.
    #[strum(serialize = "CE")]
    Ceuta,
    /// Melilla.
    #[strum(serialize = "ML")]
    Melilla,
}
