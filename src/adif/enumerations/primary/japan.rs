//! Japan subdivisions in ADIF III.B.12, DXCC entity 339.

use strum::{EnumIter, EnumString, IntoStaticStr};

/// A prefecture in ADIF's Japan group.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Chiba, Kanto.
    #[strum(serialize = "12")]
    Chiba,
    /// Gunma, Kanto.
    #[strum(serialize = "16")]
    Gunma,
    /// Ibaraki, Kanto.
    #[strum(serialize = "14")]
    Ibaraki,
    /// Kanagawa, Kanto.
    #[strum(serialize = "11")]
    Kanagawa,
    /// Saitama, Kanto.
    #[strum(serialize = "13")]
    Saitama,
    /// Tochigi, Kanto.
    #[strum(serialize = "15")]
    Tochigi,
    /// Tokyo, Kanto.
    #[strum(serialize = "10")]
    Tokyo,
    /// Yamanashi, Kanto.
    #[strum(serialize = "17")]
    Yamanashi,
    /// Aichi, Tokai.
    #[strum(serialize = "20")]
    Aichi,
    /// Gifu, Tokai.
    #[strum(serialize = "19")]
    Gifu,
    /// Mie, Tokai.
    #[strum(serialize = "21")]
    Mie,
    /// Shizuoka, Tokai.
    #[strum(serialize = "18")]
    Shizuoka,
    /// Hyogo, Kansai.
    #[strum(serialize = "27")]
    Hyogo,
    /// Kyoto, Kansai.
    #[strum(serialize = "22")]
    Kyoto,
    /// Nara, Kansai.
    #[strum(serialize = "24")]
    Nara,
    /// Osaka, Kansai.
    #[strum(serialize = "25")]
    Osaka,
    /// Shiga, Kansai.
    #[strum(serialize = "23")]
    Shiga,
    /// Wakayama, Kansai.
    #[strum(serialize = "26")]
    Wakayama,
    /// Hiroshima, Chugoku.
    #[strum(serialize = "35")]
    Hiroshima,
    /// Okayama, Chugoku.
    #[strum(serialize = "31")]
    Okayama,
    /// Shimane, Chugoku.
    #[strum(serialize = "32")]
    Shimane,
    /// Tottori, Chugoku.
    #[strum(serialize = "34")]
    Tottori,
    /// Yamaguchi, Chugoku.
    #[strum(serialize = "33")]
    Yamaguchi,
    /// Ehime, Shikoku.
    #[strum(serialize = "38")]
    Ehime,
    /// Kagawa, Shikoku.
    #[strum(serialize = "36")]
    Kagawa,
    /// Kochi, Shikoku.
    #[strum(serialize = "39")]
    Kochi,
    /// Tokushima, Shikoku.
    #[strum(serialize = "37")]
    Tokushima,
    /// Fukuoka, Kyushu.
    #[strum(serialize = "40")]
    Fukuoka,
    /// Kagoshima, Kyushu.
    #[strum(serialize = "46")]
    Kagoshima,
    /// Kumamoto, Kyushu.
    #[strum(serialize = "43")]
    Kumamoto,
    /// Miyazaki, Kyushu.
    #[strum(serialize = "45")]
    Miyazaki,
    /// Nagasaki, Kyushu.
    #[strum(serialize = "42")]
    Nagasaki,
    /// Oita, Kyushu.
    #[strum(serialize = "44")]
    Oita,
    /// Okinawa, Kyushu.
    #[strum(serialize = "47")]
    Okinawa,
    /// Saga, Kyushu.
    #[strum(serialize = "41")]
    Saga,
    /// Akita, Tohoku.
    #[strum(serialize = "04")]
    Akita,
    /// Aomori, Tohoku.
    #[strum(serialize = "02")]
    Aomori,
    /// Fukushima, Tohoku.
    #[strum(serialize = "07")]
    Fukushima,
    /// Iwate, Tohoku.
    #[strum(serialize = "03")]
    Iwate,
    /// Miyagi, Tohoku.
    #[strum(serialize = "06")]
    Miyagi,
    /// Yamagata, Tohoku.
    #[strum(serialize = "05")]
    Yamagata,
    /// Hokkaido.
    #[strum(serialize = "01")]
    Hokkaido,
    /// Fukui, Hokuriku.
    #[strum(serialize = "29")]
    Fukui,
    /// Ishikawa, Hokuriku.
    #[strum(serialize = "30")]
    Ishikawa,
    /// Toyama, Hokuriku.
    #[strum(serialize = "28")]
    Toyama,
    /// Nagano, Shin'estu.
    #[strum(serialize = "09")]
    Nagano,
    /// Niigata, Shin'estu.
    #[strum(serialize = "08")]
    Niigata,
}
