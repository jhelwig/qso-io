//! Andaman and Nicobar Islands primary subdivision, DXCC entity 11.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Andaman and Nicobar Islands primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Andaman and Nicobar Islands, a union territory.
    #[strum(serialize = "AN")]
    AndamanAndNicobarIslands,
}
