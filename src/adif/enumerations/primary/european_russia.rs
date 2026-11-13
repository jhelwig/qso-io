//! European Russia primary subdivisions, DXCC entity 54.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A European Russia primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// City of St. Petersburg.
    #[strum(serialize = "SP")]
    SaintPetersburg,
    /// Leningradskaya oblast.
    #[strum(serialize = "LO")]
    Leningrad,
    /// Republic of Karelia.
    #[strum(serialize = "KL")]
    Karelia,
    /// Arkhangelsk (Arkhangelskaya oblast).
    #[strum(serialize = "AR")]
    Arkhangelsk,
    /// Nenetsky Autonomous Okrug.
    #[strum(serialize = "NO")]
    Nenetsky,
    /// Vologda (Vologodskaya oblast).
    #[strum(serialize = "VO")]
    Vologda,
    /// Novgorodskaya oblast.
    #[strum(serialize = "NV")]
    Novgorod,
    /// Pskov (Pskovskaya oblast).
    #[strum(serialize = "PS")]
    Pskov,
    /// Murmansk (Murmanskaya oblast).
    #[strum(serialize = "MU")]
    Murmansk,
    /// City of Moscow.
    #[strum(serialize = "MA")]
    MoscowCity,
    /// Moscowskaya oblast.
    #[strum(serialize = "MO")]
    MoscowRegion,
    /// Oryel (Orlovskaya oblast).
    #[strum(serialize = "OR")]
    Oryel,
    /// Lipetsk (Lipetskaya oblast).
    #[strum(serialize = "LP")]
    Lipetsk,
    /// Tver' (Tverskaya oblast).
    #[strum(serialize = "TV")]
    Tver,
    /// Smolensk (Smolenskaya oblast).
    #[strum(serialize = "SM")]
    Smolensk,
    /// Yaroslavl (Yaroslavskaya oblast).
    #[strum(serialize = "YR")]
    Yaroslavl,
    /// Kostroma (Kostromskaya oblast).
    #[strum(serialize = "KS")]
    Kostroma,
    /// Tula (Tul'skaya oblast).
    #[strum(serialize = "TL")]
    Tula,
    /// Voronezh (Voronezhskaya oblast).
    #[strum(serialize = "VR")]
    Voronezh,
    /// Tambov (Tambovskaya oblast).
    #[strum(serialize = "TB")]
    Tambov,
    /// Ryazan' (Ryazanskaya oblast).
    #[strum(serialize = "RA")]
    Ryazan,
    /// Nizhni Novgorod (Nizhegorodskaya oblast).
    #[strum(serialize = "NN")]
    NizhniNovgorod,
    /// Ivanovo (Ivanovskaya oblast).
    #[strum(serialize = "IV")]
    Ivanovo,
    /// Vladimir (Vladimirskaya oblast).
    #[strum(serialize = "VL")]
    Vladimir,
    /// Kursk (Kurskaya oblast).
    #[strum(serialize = "KU")]
    Kursk,
    /// Kaluga (Kaluzhskaya oblast).
    #[strum(serialize = "KG")]
    Kaluga,
    /// Bryansk (Bryanskaya oblast).
    #[strum(serialize = "BR")]
    Bryansk,
    /// Belgorod (Belgorodskaya oblast).
    #[strum(serialize = "BO")]
    Belgorod,
    /// Volgograd (Volgogradskaya oblast).
    #[strum(serialize = "VG")]
    Volgograd,
    /// Saratov (Saratovskaya oblast).
    #[strum(serialize = "SA")]
    Saratov,
    /// Penza (Penzenskaya oblast).
    #[strum(serialize = "PE")]
    Penza,
    /// Samara (Samarskaya oblast).
    #[strum(serialize = "SR")]
    Samara,
    /// Ulyanovsk (Ulyanovskaya oblast).
    #[strum(serialize = "UL")]
    Ulyanovsk,
    /// Kirov (Kirovskaya oblast).
    #[strum(serialize = "KI")]
    Kirov,
    /// Republic of Tataria.
    #[strum(serialize = "TA")]
    Tataria,
    /// Republic of Marij-El.
    #[strum(serialize = "MR")]
    MarijEl,
    /// Republic of Mordovia.
    #[strum(serialize = "MD")]
    Mordovia,
    /// Republic of Udmurtia.
    #[strum(serialize = "UD")]
    Udmurtia,
    /// Republic of Chuvashia.
    #[strum(serialize = "CU")]
    Chuvashia,
    /// Krasnodar (Krasnodarsky Kraj).
    #[strum(serialize = "KR")]
    Krasnodar,
    /// Republic of Karachaevo-Cherkessia.
    #[strum(serialize = "KC")]
    KarachaevoCherkessia,
    /// Stavropol' (Stavropolsky Kraj).
    #[strum(serialize = "ST")]
    Stavropol,
    /// Republic of Kalmykia.
    #[strum(serialize = "KM")]
    Kalmykia,
    /// Republic of Northern Ossetia.
    #[strum(serialize = "SO")]
    NorthernOssetia,
    /// Rostov-on-Don (Rostovskaya oblast).
    #[strum(serialize = "RO")]
    RostovOnDon,
    /// Republic Chechnya.
    #[strum(serialize = "CN")]
    Chechnya,
    /// Republic of Ingushetia.
    #[strum(serialize = "IN")]
    Ingushetia,
    /// Astrakhan' (Astrakhanskaya oblast).
    #[strum(serialize = "AO")]
    Astrakhan,
    /// Republic of Daghestan.
    #[strum(serialize = "DA")]
    Daghestan,
    /// Republic of Kabardino-Balkaria.
    #[strum(serialize = "KB")]
    KabardinoBalkaria,
    /// Republic of Adygeya.
    #[strum(serialize = "AD")]
    Adygeya,
}
