//! Hawaii primary subdivision, DXCC entity 110.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Hawaii primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Hawaii.
    #[strum(serialize = "HI")]
    Hawaii,
}
