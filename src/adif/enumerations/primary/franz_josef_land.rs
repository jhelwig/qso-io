//! Franz Josef Land primary subdivisions, DXCC entity 61.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Franz Josef Land primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Arkhangelsk (Arkhangelskaya oblast).
    #[strum(serialize = "AR")]
    Arkhangelsk,
    /// Franz Josef Land; import-only subdivision spelling.
    #[strum(serialize = "FJL")]
    FranzJosefLand,
}
