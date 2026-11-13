//! Polish subdivisions, DXCC 269, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Polish administrative subdivisions recognized by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Zachodnio-Pomorskie.
    #[strum(serialize = "Z")]
    ZachodnioPomorskie,
    /// Pomorskie.
    #[strum(serialize = "F")]
    Pomorskie,
    /// Kujawsko-Pomorskie.
    #[strum(serialize = "P")]
    KujawskoPomorskie,
    /// Lubuskie.
    #[strum(serialize = "B")]
    Lubuskie,
    /// Wielkopolskie.
    #[strum(serialize = "W")]
    Wielkopolskie,
    /// Warminsko-Mazurskie.
    #[strum(serialize = "J")]
    WarminskoMazurskie,
    /// Podlaskie.
    #[strum(serialize = "O")]
    Podlaskie,
    /// Mazowieckie.
    #[strum(serialize = "R")]
    Mazowieckie,
    /// Dolnoslaskie.
    #[strum(serialize = "D")]
    Dolnoslaskie,
    /// Opolskie.
    #[strum(serialize = "U")]
    Opolskie,
    /// Lodzkie.
    #[strum(serialize = "C")]
    Lodzkie,
    /// Swietokrzyskie.
    #[strum(serialize = "S")]
    Swietokrzyskie,
    /// Podkarpackie.
    #[strum(serialize = "K")]
    Podkarpackie,
    /// Lubelskie.
    #[strum(serialize = "L")]
    Lubelskie,
    /// Slaskie.
    #[strum(serialize = "G")]
    Slaskie,
    /// Malopolskie.
    #[strum(serialize = "M")]
    Malopolskie,
}
