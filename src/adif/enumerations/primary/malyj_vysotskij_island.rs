//! Malyj Vysotskij Island primary subdivisions, DXCC entity 151.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Malyj Vysotskij Island primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Leningradskaya Oblast.
    #[strum(serialize = "LO")]
    Leningrad,
    /// Malyj Vysotskij; import-only subdivision spelling.
    #[strum(serialize = "MV")]
    MalyjVysotskij,
}
