//! Cuban primary subdivisions, DXCC entity 70.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Cuban province or special municipality supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Artemisa province.
    #[strum(serialize = "15")]
    Artemisa,
    /// Camagüey province.
    #[strum(serialize = "09")]
    Camaguey,
    /// Ciego de Ávila province.
    #[strum(serialize = "08")]
    CiegoDeAvila,
    /// Cienfuegos province.
    #[strum(serialize = "06")]
    Cienfuegos,
    /// Granma province.
    #[strum(serialize = "12")]
    Granma,
    /// Guantánamo province.
    #[strum(serialize = "14")]
    Guantanamo,
    /// Holguín province.
    #[strum(serialize = "11")]
    Holguin,
    /// Isla de la Juventud, a special municipality.
    #[strum(serialize = "99")]
    IslaDeLaJuventud,
    /// La Habana province.
    #[strum(serialize = "03")]
    LaHabana,
    /// Las Tunas province.
    #[strum(serialize = "10")]
    LasTunas,
    /// Matanzas province.
    #[strum(serialize = "04")]
    Matanzas,
    /// Mayabeque province.
    #[strum(serialize = "16")]
    Mayabeque,
    /// Pinar del Río province.
    #[strum(serialize = "01")]
    PinarDelRio,
    /// Sancti Spíritus province.
    #[strum(serialize = "07")]
    SanctiSpiritus,
    /// Santiago de Cuba province.
    #[strum(serialize = "13")]
    SantiagoDeCuba,
    /// Villa Clara province.
    #[strum(serialize = "05")]
    VillaClara,
}
