//! Minami Torishima primary subdivision, DXCC entity 177.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Minami Torishima primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Minami Torishima.
    #[strum(serialize = "MT")]
    MinamiTorishima,
}
