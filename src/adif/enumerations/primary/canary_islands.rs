//! Canary Islands primary subdivisions, DXCC entity 29.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Canary Islands primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Las Palmas.
    #[strum(serialize = "GC")]
    LasPalmas,
    /// Santa Cruz de Tenerife.
    #[strum(serialize = "TF")]
    SantaCruzDeTenerife,
}
