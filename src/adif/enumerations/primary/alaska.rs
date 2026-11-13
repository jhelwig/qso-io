//! Alaska primary subdivision, DXCC entity 6.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Alaska primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Alaska.
    #[strum(serialize = "AK")]
    Alaska,
}
