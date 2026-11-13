//! Asiatic Russia primary subdivisions, DXCC entity 15.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// An Asiatic Russia primary subdivision, retaining ADIF's historical identities.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Ust’-Ordynsky Autonomous Okrug, for contacts before 2008-01-01.
    #[strum(serialize = "UO")]
    UstOrdynsky,
    /// Aginsky Buryatsky Autonomous Okrug, for contacts before 2008-03-01.
    #[strum(serialize = "AB")]
    AginskyBuryatsky,
    /// Chelyabinsk (Chelyabinskaya oblast).
    #[strum(serialize = "CB")]
    Chelyabinsk,
    /// Sverdlovskaya oblast.
    #[strum(serialize = "SV")]
    Sverdlovsk,
    /// Perm: ADIF associates PM with Perm` (Permskaya oblast) on or after
    /// 2005-12-01 and with Permskaya Kraj before 2005-12-01.
    #[strum(serialize = "PM")]
    Perm,
    /// Komi-Permyatsky Autonomous Okrug, for contacts before 2005-12-01.
    #[strum(serialize = "KP")]
    KomiPermyatsky,
    /// Tomsk (Tomskaya oblast).
    #[strum(serialize = "TO")]
    Tomsk,
    /// Khanty-Mansyisky Autonomous Okrug.
    #[strum(serialize = "HM")]
    KhantyMansyisky,
    /// Yamalo-Nenetsky Autonomous Okrug.
    #[strum(serialize = "YN")]
    YamaloNenetsky,
    /// Tyumen' (Tyumenskaya oblast).
    #[strum(serialize = "TN")]
    Tyumen,
    /// Omsk (Omskaya oblast).
    #[strum(serialize = "OM")]
    Omsk,
    /// Novosibirsk (Novosibirskaya oblast).
    #[strum(serialize = "NS")]
    Novosibirsk,
    /// Kurgan (Kurganskaya oblast).
    #[strum(serialize = "KN")]
    Kurgan,
    /// Orenburg (Orenburgskaya oblast).
    #[strum(serialize = "OB")]
    Orenburg,
    /// Kemerovo (Kemerovskaya oblast).
    #[strum(serialize = "KE")]
    Kemerovo,
    /// Republic of Bashkortostan.
    #[strum(serialize = "BA")]
    Bashkortostan,
    /// Republic of Komi.
    #[strum(serialize = "KO")]
    Komi,
    /// Altaysky Kraj.
    #[strum(serialize = "AL")]
    Altaysky,
    /// Republic Gorny Altay.
    #[strum(serialize = "GA")]
    GornyAltay,
    /// Krasnoyarsk (Krasnoyarsk Kraj).
    #[strum(serialize = "KK")]
    Krasnoyarsk,
    /// Taymyr Autonomous Okrug, for contacts before 2007-01-01.
    #[strum(serialize = "TM")]
    Taymyr,
    /// Khabarovsk (Khabarovsky Kraj).
    #[strum(serialize = "HK")]
    Khabarovsk,
    /// Yevreyskaya Autonomous Oblast.
    #[strum(serialize = "EA")]
    Yevreyskaya,
    /// Sakhalin (Sakhalinskaya oblast).
    #[strum(serialize = "SL")]
    Sakhalin,
    /// Evenkiysky Autonomous Okrug, for contacts before 2007-01-01.
    #[strum(serialize = "EV")]
    Evenkiysky,
    /// Magadan (Magadanskaya oblast).
    #[strum(serialize = "MG")]
    Magadan,
    /// Amurskaya oblast.
    #[strum(serialize = "AM")]
    Amur,
    /// Chukotka Autonomous Okrug.
    #[strum(serialize = "CK")]
    Chukotka,
    /// Primorsky Kraj.
    #[strum(serialize = "PK")]
    Primorsky,
    /// Republic of Buryatia.
    #[strum(serialize = "BU")]
    Buryatia,
    /// Sakha (Yakut) Republic.
    #[strum(serialize = "YA")]
    Sakha,
    /// Irkutsk (Irkutskaya oblast).
    #[strum(serialize = "IR")]
    Irkutsk,
    /// Zabaykalsky Kraj; called Chita (Chitinskaya oblast) before 2008-03-01.
    #[strum(serialize = "CT")]
    Zabaykalsky,
    /// Republic of Khakassia.
    #[strum(serialize = "HA")]
    Khakassia,
    /// Koryaksky Autonomous Okrug, for contacts before 2007-01-01.
    #[strum(serialize = "KY")]
    Koryaksky,
    /// Republic of Tuva.
    #[strum(serialize = "TU")]
    Tuva,
    /// Kamchatka (Kamchatskaya oblast).
    #[strum(serialize = "KT")]
    Kamchatka,
}
