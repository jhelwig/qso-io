//! Republic of Korea primary subdivisions, DXCC entity 137.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Republic of Korea primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Seoul (Seoul Teugbyeolsi).
    #[strum(serialize = "A")]
    Seoul,
    /// Inchon (Incheon Gwang'yeogsi).
    #[strum(serialize = "N")]
    Inchon,
    /// Kangwon-do (Gang 'weondo).
    #[strum(serialize = "D")]
    Kangwon,
    /// Kyunggi-do (Gyeonggido).
    #[strum(serialize = "C")]
    Kyunggi,
    /// Choongchungbuk-do (Chungcheongbugdo).
    #[strum(serialize = "E")]
    Choongchungbuk,
    /// Choongchungnam-do (Chungcheongnamdo).
    #[strum(serialize = "F")]
    Choongchungnam,
    /// Taejon (Daejeon Gwang'yeogsi).
    #[strum(serialize = "R")]
    Taejon,
    /// Cheju-do (Jejudo).
    #[strum(serialize = "M")]
    Cheju,
    /// Chollabuk-do (Jeonrabugdo).
    #[strum(serialize = "G")]
    Chollabuk,
    /// Chollanam-do (Jeonranamdo).
    #[strum(serialize = "H")]
    Chollanam,
    /// Kwangju (Gwangju Gwang'yeogsi).
    #[strum(serialize = "Q")]
    Kwangju,
    /// Kyungsangbuk-do (Gyeongsangbugdo).
    #[strum(serialize = "K")]
    Kyungsangbuk,
    /// Kyungsangnam-do (Gyeongsangnamdo).
    #[strum(serialize = "L")]
    Kyungsangnam,
    /// Pusan (Busan Gwang'yeogsi).
    #[strum(serialize = "B")]
    Pusan,
    /// Taegu (Daegu Gwang'yeogsi).
    #[strum(serialize = "P")]
    Taegu,
    /// Ulsan (Ulsan Gwanq'yeogsi).
    #[strum(serialize = "S")]
    Ulsan,
    /// Sejong.
    #[strum(serialize = "T")]
    Sejong,
    /// Special Island.
    #[strum(serialize = "IS")]
    SpecialIsland,
}
