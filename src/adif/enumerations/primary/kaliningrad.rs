//! Kaliningrad primary subdivision, DXCC entity 126.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// The Kaliningrad primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Kalingrad (Kaliningradskaya oblast), retaining ADIF's description spelling.
    #[strum(serialize = "KA")]
    Kaliningrad,
}
