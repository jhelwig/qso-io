//! Lakshadweep primary subdivision, DXCC entity 142.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Lakshadweep primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Lakshadweep, a union territory.
    #[strum(serialize = "LD")]
    Lakshadweep,
}
