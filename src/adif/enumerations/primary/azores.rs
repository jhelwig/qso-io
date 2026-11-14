//! Azores primary subdivision, DXCC entity 149.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Azores primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Açores.
    #[strum(serialize = "AC")]
    Acores,
}
