//! Jan Mayen primary subdivision, DXCC entity 118.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Jan Mayen primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Jan Mayen.
    #[strum(serialize = "22")]
    JanMayen,
}
