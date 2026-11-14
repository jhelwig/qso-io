//! Balearic Islands primary subdivision, DXCC entity 21.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Balearic Islands primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Balears.
    #[strum(serialize = "IB")]
    Balears,
}
