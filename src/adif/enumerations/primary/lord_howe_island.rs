//! Lord Howe Island primary subdivision, DXCC entity 147.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Lord Howe Island primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Lord Howe Is.
    #[strum(serialize = "LH")]
    LordHoweIsland,
}
