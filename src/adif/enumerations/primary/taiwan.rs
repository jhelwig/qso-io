//! Taiwan subdivisions in ADIF III.B.12, DXCC entity 386.

use strum::{EnumIter, EnumString, IntoStaticStr};

/// A county, city or special municipality in ADIF's Taiwan group.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Changhua county.
    #[strum(serialize = "CHA")]
    Changhua,
    /// Chiayi city.
    #[strum(serialize = "CYI")]
    ChiayiCity,
    /// Chiayi county.
    #[strum(serialize = "CYQ")]
    ChiayiCounty,
    /// Hsinchu city.
    #[strum(serialize = "HSZ")]
    HsinchuCity,
    /// Hsinchu county.
    #[strum(serialize = "HSQ")]
    HsinchuCounty,
    /// Hualien county.
    #[strum(serialize = "HUA")]
    Hualien,
    /// Kaohsiung special municipality.
    #[strum(serialize = "KHH")]
    Kaohsiung,
    /// Keelung city.
    #[strum(serialize = "KEE")]
    Keelung,
    /// Kinmen county.
    #[strum(serialize = "KIN")]
    Kinmen,
    /// Lienchiang county.
    #[strum(serialize = "LIE")]
    Lienchiang,
    /// Miaoli county.
    #[strum(serialize = "MIA")]
    Miaoli,
    /// Nantou county.
    #[strum(serialize = "NAN")]
    Nantou,
    /// New Taipei special municipality.
    #[strum(serialize = "NWT")]
    NewTaipei,
    /// Penghu county.
    #[strum(serialize = "PEN")]
    Penghu,
    /// Pingtung county.
    #[strum(serialize = "PIF")]
    Pingtung,
    /// Taichung special municipality.
    #[strum(serialize = "TXG")]
    Taichung,
    /// Tainan special municipality.
    #[strum(serialize = "TNN")]
    Tainan,
    /// Taipei special municipality.
    #[strum(serialize = "TPE")]
    Taipei,
    /// Taitung county.
    #[strum(serialize = "TTT")]
    Taitung,
    /// Taoyuan special municipality.
    #[strum(serialize = "TAO")]
    Taoyuan,
    /// Yilan county.
    #[strum(serialize = "ILA")]
    Yilan,
    /// Yunlin county.
    #[strum(serialize = "YUN")]
    Yunlin,
}
