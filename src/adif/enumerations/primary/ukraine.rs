//! Ukrainian subdivisions, DXCC 288, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Ukrainian administrative subdivisions as named by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Sums'ka Oblast'.
    #[strum(serialize = "SU")]
    Sumska,
    /// Ternopil's'ka Oblast'.
    #[strum(serialize = "TE")]
    Ternopilska,
    /// Cherkas'ka Oblast'.
    #[strum(serialize = "CH")]
    Cherkaska,
    /// Zakarpats'ka Oblast'.
    #[strum(serialize = "ZA")]
    Zakarpatska,
    /// Dnipropetrovs'ka Oblast'.
    #[strum(serialize = "DN")]
    Dnipropetrovska,
    /// Odes'ka Oblast'.
    #[strum(serialize = "OD")]
    Odeska,
    /// Khersons'ka Oblast'.
    #[strum(serialize = "HE")]
    Khersonska,
    /// Poltavs'ka Oblast'.
    #[strum(serialize = "PO")]
    Poltavska,
    /// Donets'ka Oblast'.
    #[strum(serialize = "DO")]
    Donetska,
    /// Rivnens'ka Oblast'.
    #[strum(serialize = "RI")]
    Rivnenska,
    /// Kharkivs'ka Oblast'.
    #[strum(serialize = "HA")]
    Kharkivska,
    /// Luhans'ka Oblast'.
    #[strum(serialize = "LU")]
    Luhanska,
    /// Vinnyts'ka Oblast'.
    #[strum(serialize = "VI")]
    Vinnytska,
    /// Volyos'ka Oblast'.
    #[strum(serialize = "VO")]
    Volyoska,
    /// Zaporiz'ka Oblast'.
    #[strum(serialize = "ZP")]
    Zaporizka,
    /// Chernihivs'ka Oblast'.
    #[strum(serialize = "CR")]
    Chernihivska,
    /// Ivano-Frankivs'ka Oblast'.
    #[strum(serialize = "IF")]
    IvanoFrankivska,
    /// Khmel'nyts'ka Oblast'.
    #[strum(serialize = "HM")]
    Khmelnytska,
    /// Kyïv.
    #[strum(serialize = "KV")]
    Kyiv,
    /// Kyivs'ka Oblast'.
    #[strum(serialize = "KO")]
    Kyivska,
    /// Kirovohrads'ka Oblast'.
    #[strum(serialize = "KI")]
    Kirovohradska,
    /// L'vivs'ka Oblast'.
    #[strum(serialize = "LV")]
    Lvivska,
    /// Zhytomyrs'ka Oblast'.
    #[strum(serialize = "ZH")]
    Zhytomyrska,
    /// Chernivets'ka Oblast'.
    #[strum(serialize = "CN")]
    Chernivetska,
    /// Mykolaivs'ka Oblast'.
    #[strum(serialize = "NI")]
    Mykolaivska,
    /// Respublika Krym.
    #[strum(serialize = "KR")]
    RespublikaKrym,
    /// Sevastopol'.
    #[strum(serialize = "SL")]
    Sevastopol,
}
