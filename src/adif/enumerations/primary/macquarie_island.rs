//! Macquarie Island primary subdivision, DXCC entity 153.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Macquarie Island primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Macquarie Is.
    #[strum(serialize = "MA")]
    MacquarieIsland,
}
