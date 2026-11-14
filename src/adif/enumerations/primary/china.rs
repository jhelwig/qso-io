//! China subdivisions in ADIF III.B.12, DXCC entity 318.

use strum::{EnumIter, EnumString, IntoStaticStr};

/// A primary subdivision in ADIF's China group.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Anhui.
    #[strum(serialize = "AH")]
    Anhui,
    /// Beijing.
    #[strum(serialize = "BJ")]
    Beijing,
    /// Chongqing.
    #[strum(serialize = "CQ")]
    Chongqing,
    /// Fujian.
    #[strum(serialize = "FJ")]
    Fujian,
    /// Guangdong.
    #[strum(serialize = "GD")]
    Guangdong,
    /// Gansu.
    #[strum(serialize = "GS")]
    Gansu,
    /// Guangxi Zhuangzu.
    #[strum(serialize = "GX")]
    GuangxiZhuangzu,
    /// Guizhou.
    #[strum(serialize = "GZ")]
    Guizhou,
    /// Henan.
    #[strum(serialize = "HA")]
    Henan,
    /// Hubei.
    #[strum(serialize = "HB")]
    Hubei,
    /// Hebei.
    #[strum(serialize = "HE")]
    Hebei,
    /// Hainan.
    #[strum(serialize = "HI")]
    Hainan,
    /// Heilongjiang.
    #[strum(serialize = "HL")]
    Heilongjiang,
    /// Hunan.
    #[strum(serialize = "HN")]
    Hunan,
    /// Jilin.
    #[strum(serialize = "JL")]
    Jilin,
    /// Jiangsu.
    #[strum(serialize = "JS")]
    Jiangsu,
    /// Jiangxi.
    #[strum(serialize = "JX")]
    Jiangxi,
    /// Liaoning.
    #[strum(serialize = "LN")]
    Liaoning,
    /// Nei Mongol.
    #[strum(serialize = "NM")]
    NeiMongol,
    /// Ningxia Huizu.
    #[strum(serialize = "NX")]
    NingxiaHuizu,
    /// Qinghai.
    #[strum(serialize = "QH")]
    Qinghai,
    /// Sichuan.
    #[strum(serialize = "SC")]
    Sichuan,
    /// Shandong.
    #[strum(serialize = "SD")]
    Shandong,
    /// Shanghai.
    #[strum(serialize = "SH")]
    Shanghai,
    /// Shaanxi.
    #[strum(serialize = "SN")]
    Shaanxi,
    /// Shanxi.
    #[strum(serialize = "SX")]
    Shanxi,
    /// Tianjin.
    #[strum(serialize = "TJ")]
    Tianjin,
    /// Xinjiang Uygur.
    #[strum(serialize = "XJ")]
    XinjiangUygur,
    /// Xizang.
    #[strum(serialize = "XZ")]
    Xizang,
    /// Yunnan.
    #[strum(serialize = "YN")]
    Yunnan,
    /// Zhejiang.
    #[strum(serialize = "ZJ")]
    Zhejiang,
}
