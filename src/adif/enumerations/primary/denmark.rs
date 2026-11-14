//! Danish subdivisions, DXCC 221, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Danish administrative subdivisions listed by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Koebenhavns amt.
    #[strum(serialize = "015")]
    Koebenhavns,
    /// Frederiksborg amt.
    #[strum(serialize = "020")]
    Frederiksborg,
    /// Roskilde amt.
    #[strum(serialize = "025")]
    Roskilde,
    /// Vestsjaellands amt.
    #[strum(serialize = "030")]
    Vestsjaellands,
    /// Storstrøm amt (Storstroems).
    #[strum(serialize = "035")]
    Storstrom,
    /// Bornholms amt.
    #[strum(serialize = "040")]
    Bornholms,
    /// Fyns amt.
    #[strum(serialize = "042")]
    Fyns,
    /// Sínderjylland amt (Sydjyllands).
    #[strum(serialize = "050")]
    Sinderjylland,
    /// Ribe amt.
    #[strum(serialize = "055")]
    Ribe,
    /// Vejle amt.
    #[strum(serialize = "060")]
    Vejle,
    /// Ringkøbing amt (Ringkoebing).
    #[strum(serialize = "065")]
    Ringkobing,
    /// Århus amt (Aarhus).
    #[strum(serialize = "070")]
    Arhus,
    /// Viborg amt.
    #[strum(serialize = "076")]
    Viborg,
    /// Nordjyllands amt.
    #[strum(serialize = "080")]
    Nordjyllands,
    /// Copenhagen City.
    #[strum(serialize = "101")]
    CopenhagenCity,
    /// Frederiksberg.
    #[strum(serialize = "147")]
    Frederiksberg,
}
