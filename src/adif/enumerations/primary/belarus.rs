//! Belarus primary subdivisions, DXCC entity 27.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Belarus primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Minsk (Minskaya voblasts').
    #[strum(serialize = "MI")]
    MinskRegion,
    /// Brest (Brestskaya voblasts').
    #[strum(serialize = "BR")]
    Brest,
    /// Grodno (Hrodzenskaya voblasts').
    #[strum(serialize = "HR")]
    Grodno,
    /// Vitebsk (Vitsyebskaya voblasts').
    #[strum(serialize = "VI")]
    Vitebsk,
    /// Mogilev (Mahilyowskaya voblasts').
    #[strum(serialize = "MA")]
    Mogilev,
    /// Gomel (Homyel'skaya voblasts').
    #[strum(serialize = "HO")]
    Gomel,
    /// Horad Minsk.
    #[strum(serialize = "HM")]
    MinskCity,
}
