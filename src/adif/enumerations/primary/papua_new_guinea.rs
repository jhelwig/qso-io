//! Papua New Guinea primary subdivisions, DXCC entity 163.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Papua New Guinea primary subdivision, including import-only historical identities.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// National Capital District (Port Moresby).
    #[strum(serialize = "NCD")]
    NationalCapitalDistrict,
    /// Central.
    #[strum(serialize = "CPM")]
    Central,
    /// Chimbu.
    #[strum(serialize = "CPK")]
    Chimbu,
    /// Eastern Highlands.
    #[strum(serialize = "EHG")]
    EasternHighlands,
    /// East New Britain.
    #[strum(serialize = "EBR")]
    EastNewBritain,
    /// East Sepik.
    #[strum(serialize = "ESW")]
    EastSepik,
    /// Enga.
    #[strum(serialize = "EPW")]
    Enga,
    /// Gulf.
    #[strum(serialize = "GPK")]
    Gulf,
    /// Madang.
    #[strum(serialize = "MPM")]
    Madang,
    /// Manus.
    #[strum(serialize = "MRL")]
    Manus,
    /// Milne Bay.
    #[strum(serialize = "MBA")]
    MilneBay,
    /// Morobe.
    #[strum(serialize = "MPL")]
    Morobe,
    /// New Ireland.
    #[strum(serialize = "NIK")]
    NewIreland,
    /// Northern.
    #[strum(serialize = "NPP")]
    Northern,
    /// North Solomons; import-only historical spelling.
    #[strum(serialize = "NSA")]
    NorthSolomons,
    /// Bougainville.
    #[strum(serialize = "NSB")]
    Bougainville,
    /// West Sepik.
    #[strum(serialize = "SAN")]
    WestSepik,
    /// Southern Highlands.
    #[strum(serialize = "SHM")]
    SouthernHighlands,
    /// Western.
    #[strum(serialize = "WPD")]
    Western,
    /// Western Highlands.
    #[strum(serialize = "WHM")]
    WesternHighlands,
    /// West New Britain; import-only historical spelling WBR.
    #[strum(serialize = "WBR")]
    WestNewBritainLegacy,
    /// West New Britain.
    #[strum(serialize = "WBK")]
    WestNewBritain,
    /// Hela.
    #[strum(serialize = "HLA")]
    Hela,
    /// Jiwaka.
    #[strum(serialize = "JWK")]
    Jiwaka,
}
