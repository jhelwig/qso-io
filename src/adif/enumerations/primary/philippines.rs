//! Philippines subdivisions in ADIF III.B.12, DXCC entity 375.

use strum::{EnumIter, EnumString, IntoStaticStr};

/// A province in ADIF's Philippines group.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Aurora, Southern Tagalog.
    #[strum(serialize = "AUR")]
    Aurora,
    /// Batangas, Southern Tagalog.
    #[strum(serialize = "BTG")]
    Batangas,
    /// Cavite, Southern Tagalog.
    #[strum(serialize = "CAV")]
    Cavite,
    /// Laguna, Southern Tagalog.
    #[strum(serialize = "LAG")]
    Laguna,
    /// Marinduque, Southern Tagalog.
    #[strum(serialize = "MAD")]
    Marinduque,
    /// Mindoro Occidental, Southern Tagalog.
    #[strum(serialize = "MDC")]
    MindoroOccidental,
    /// Mindoro Oriental, Southern Tagalog.
    #[strum(serialize = "MDR")]
    MindoroOriental,
    /// Palawan, Southern Tagalog.
    #[strum(serialize = "PLW")]
    Palawan,
    /// Quezon, Southern Tagalog.
    #[strum(serialize = "QUE")]
    Quezon,
    /// Rizal, Southern Tagalog.
    #[strum(serialize = "RIZ")]
    Rizal,
    /// Romblon, Southern Tagalog.
    #[strum(serialize = "ROM")]
    Romblon,
    /// Ilocos Norte, Ilocos.
    #[strum(serialize = "ILN")]
    IlocosNorte,
    /// Ilocos Sur, Ilocos.
    #[strum(serialize = "ILS")]
    IlocosSur,
    /// La Union, Ilocos.
    #[strum(serialize = "LUN")]
    LaUnion,
    /// Pangasinan, Ilocos.
    #[strum(serialize = "PAN")]
    Pangasinan,
    /// Batanes, Cagayan Valley.
    #[strum(serialize = "BTN")]
    Batanes,
    /// Cagayan, Cagayan Valley.
    #[strum(serialize = "CAG")]
    Cagayan,
    /// Isabela, Cagayan Valley.
    #[strum(serialize = "ISA")]
    Isabela,
    /// Nueva Vizcaya, Cagayan Valley.
    #[strum(serialize = "NUV")]
    NuevaVizcaya,
    /// Quirino, Cagayan Valley.
    #[strum(serialize = "QUI")]
    Quirino,
    /// Abra, Cordillera Administrative Region.
    #[strum(serialize = "ABR")]
    Abra,
    /// Apayao, Cordillera Administrative Region.
    #[strum(serialize = "APA")]
    Apayao,
    /// Benguet, Cordillera Administrative Region.
    #[strum(serialize = "BEN")]
    Benguet,
    /// Ifugao, Cordillera Administrative Region.
    #[strum(serialize = "IFU")]
    Ifugao,
    /// Kalinga, Cordillera Administrative Region.
    #[strum(serialize = "KAL")]
    Kalinga,
    /// Mountain Province, Cordillera Administrative Region.
    #[strum(serialize = "MOU")]
    MountainProvince,
    /// Bataan, Central Luzon.
    #[strum(serialize = "BAN")]
    Bataan,
    /// Bulacan, Central Luzon.
    #[strum(serialize = "BUL")]
    Bulacan,
    /// Nueva Ecija, Central Luzon.
    #[strum(serialize = "NUE")]
    NuevaEcija,
    /// Pampanga, Central Luzon.
    #[strum(serialize = "PAM")]
    Pampanga,
    /// Tarlac, Central Luzon.
    #[strum(serialize = "TAR")]
    Tarlac,
    /// Zambales, Central Luzon.
    #[strum(serialize = "ZMB")]
    Zambales,
    /// Albay, Bicol.
    #[strum(serialize = "ALB")]
    Albay,
    /// Camarines Norte, Bicol.
    #[strum(serialize = "CAN")]
    CamarinesNorte,
    /// Camarines Sur, Bicol.
    #[strum(serialize = "CAS")]
    CamarinesSur,
    /// Catanduanes, Bicol.
    #[strum(serialize = "CAT")]
    Catanduanes,
    /// Masbate, Bicol.
    #[strum(serialize = "MAS")]
    Masbate,
    /// Sorsogon, Bicol.
    #[strum(serialize = "SOR")]
    Sorsogon,
    /// Biliran, Eastern Visayas.
    #[strum(serialize = "BIL")]
    Biliran,
    /// Eastern Samar, Eastern Visayas.
    #[strum(serialize = "EAS")]
    EasternSamar,
    /// Leyte, Eastern Visayas.
    #[strum(serialize = "LEY")]
    Leyte,
    /// Northern Samar, Eastern Visayas.
    #[strum(serialize = "NSA")]
    NorthernSamar,
    /// Southern Leyte, Eastern Visayas.
    #[strum(serialize = "SLE")]
    SouthernLeyte,
    /// Western Samar, Eastern Visayas.
    #[strum(serialize = "WSA")]
    WesternSamar,
    /// Aklan, Western Visayas.
    #[strum(serialize = "AKL")]
    Aklan,
    /// Antique, Western Visayas.
    #[strum(serialize = "ANT")]
    Antique,
    /// Capiz, Western Visayas.
    #[strum(serialize = "CAP")]
    Capiz,
    /// Guimaras, Western Visayas.
    #[strum(serialize = "GUI")]
    Guimaras,
    /// Iloilo, Western Visayas.
    #[strum(serialize = "ILI")]
    Iloilo,
    /// Negros Occidental, Western Visayas.
    #[strum(serialize = "NEC")]
    NegrosOccidental,
    /// Bohol, Central Visayas.
    #[strum(serialize = "BOH")]
    Bohol,
    /// Cebu, Central Visayas.
    #[strum(serialize = "CEB")]
    Cebu,
    /// Negros Oriental, Central Visayas.
    #[strum(serialize = "NER")]
    NegrosOriental,
    /// Siquijor, Central Visayas.
    #[strum(serialize = "SIG")]
    Siquijor,
    /// Zamboanga del Norte, Zamboanga Peninsular.
    #[strum(serialize = "ZAN")]
    ZamboangaDelNorte,
    /// Zamboanga del Sur, Zamboanga Peninsular.
    #[strum(serialize = "ZAS")]
    ZamboangaDelSur,
    /// Zamboanga Sibugay, Zamboanga Peninsular.
    #[strum(serialize = "ZSI")]
    ZamboangaSibugay,
    /// Cotabato, Soccsksargen.
    #[strum(serialize = "NCO")]
    Cotabato,
    /// Sultan Kudarat, Soccsksargen.
    #[strum(serialize = "SUK")]
    SultanKudarat,
    /// Sarangani, Soccsksargen.
    #[strum(serialize = "SAR")]
    Sarangani,
    /// South Cotabato, Soccsksargen.
    #[strum(serialize = "SCO")]
    SouthCotabato,
    /// Basilan, Autonomous Region in Muslim Mindanao.
    #[strum(serialize = "BAS")]
    Basilan,
    /// Lanao del Sur, Autonomous Region in Muslim Mindanao.
    #[strum(serialize = "LAS")]
    LanaoDelSur,
    /// Maguindanao, Autonomous Region in Muslim Mindanao.
    #[strum(serialize = "MAG")]
    Maguindanao,
    /// Sulu, Autonomous Region in Muslim Mindanao.
    #[strum(serialize = "SLU")]
    Sulu,
    /// Tawi-Tawi, Autonomous Region in Muslim Mindanao.
    #[strum(serialize = "TAW")]
    TawiTawi,
    /// Lanao del Norte, Northern Mindanao.
    #[strum(serialize = "LAN")]
    LanaoDelNorte,
    /// Bukidnon, Northern Mindanao.
    #[strum(serialize = "BUK")]
    Bukidnon,
    /// Camiguin, Northern Mindanao.
    #[strum(serialize = "CAM")]
    Camiguin,
    /// Misamis Occidental, Northern Mindanao.
    #[strum(serialize = "MSC")]
    MisamisOccidental,
    /// Misamis Oriental, Northern Mindanao.
    #[strum(serialize = "MSR")]
    MisamisOriental,
    /// Davao de Oro, Davao.
    #[strum(serialize = "COM")]
    DavaoDeOro,
    /// Davao del Norte, Davao.
    #[strum(serialize = "DAV")]
    DavaoDelNorte,
    /// Davao del Sur, Davao.
    #[strum(serialize = "DAS")]
    DavaoDelSur,
    /// Davao Oriental, Davao.
    #[strum(serialize = "DAO")]
    DavaoOriental,
    /// Agusan del Norte, CARAGA.
    #[strum(serialize = "AGN")]
    AgusanDelNorte,
    /// Agusan del Sur, CARAGA.
    #[strum(serialize = "AGS")]
    AgusanDelSur,
    /// Surigao del Norte, CARAGA.
    #[strum(serialize = "SUN")]
    SurigaoDelNorte,
    /// Surigao del Sur, CARAGA.
    #[strum(serialize = "SUR")]
    SurigaoDelSur,
}
