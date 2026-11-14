//! Kure Island primary subdivision, DXCC entity 138.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Kure Island primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Kure Island.
    #[strum(serialize = "KI")]
    KureIsland,
}
