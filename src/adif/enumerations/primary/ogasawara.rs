//! Ogasawara primary subdivision, DXCC entity 192.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Ogasawara primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Ogasawara.
    #[strum(serialize = "O")]
    Ogasawara,
}
