use bigdecimal::BigDecimal;
use strum::{EnumIter, EnumString, IntoStaticStr};

/// An ADIF wavelength band, including the specification's frequency bounds.
///
/// The bounds describe ADIF classification, not permission to transmit. They
/// need not match the allocations applicable to a particular station.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum Band {
    /// 2190 metre band: 0.1357–0.1378 MHz.
    #[strum(serialize = "2190m")]
    Metres2190,
    /// 630 metre band: 0.472–0.479 MHz.
    #[strum(serialize = "630m")]
    Metres630,
    /// 560 metre band: 0.501–0.504 MHz.
    #[strum(serialize = "560m")]
    Metres560,
    /// 160 metre band: 1.8–2 MHz.
    #[strum(serialize = "160m")]
    Metres160,
    /// 80 metre band: 3.5–4 MHz.
    #[strum(serialize = "80m")]
    Metres80,
    /// 60 metre band: 5.06–5.45 MHz.
    #[strum(serialize = "60m")]
    Metres60,
    /// 40 metre band: 7–7.3 MHz.
    #[strum(serialize = "40m")]
    Metres40,
    /// 30 metre band: 10.1–10.15 MHz.
    #[strum(serialize = "30m")]
    Metres30,
    /// 20 metre band: 14–14.35 MHz.
    #[strum(serialize = "20m")]
    Metres20,
    /// 17 metre band: 18.068–18.168 MHz.
    #[strum(serialize = "17m")]
    Metres17,
    /// 15 metre band: 21–21.45 MHz.
    #[strum(serialize = "15m")]
    Metres15,
    /// 12 metre band: 24.890–24.99 MHz.
    #[strum(serialize = "12m")]
    Metres12,
    /// 10 metre band: 28–29.7 MHz.
    #[strum(serialize = "10m")]
    Metres10,
    /// 8 metre band: 40–45 MHz.
    #[strum(serialize = "8m")]
    Metres8,
    /// 6 metre band: 50–54 MHz.
    #[strum(serialize = "6m")]
    Metres6,
    /// 5 metre band: 54.000001–69.9 MHz.
    #[strum(serialize = "5m")]
    Metres5,
    /// 4 metre band: 70–71 MHz.
    #[strum(serialize = "4m")]
    Metres4,
    /// 2 metre band: 144–148 MHz.
    #[strum(serialize = "2m")]
    Metres2,
    /// 1.25 metre band: 222–225 MHz.
    #[strum(serialize = "1.25m")]
    Metres1_25,
    /// 70 centimetre band: 420–450 MHz.
    #[strum(serialize = "70cm")]
    Centimetres70,
    /// 33 centimetre band: 902–928 MHz.
    #[strum(serialize = "33cm")]
    Centimetres33,
    /// 23 centimetre band: 1240–1300 MHz.
    #[strum(serialize = "23cm")]
    Centimetres23,
    /// 13 centimetre band: 2300–2450 MHz.
    #[strum(serialize = "13cm")]
    Centimetres13,
    /// 9 centimetre band: 3300–3500 MHz.
    #[strum(serialize = "9cm")]
    Centimetres9,
    /// 6 centimetre band: 5650–5925 MHz.
    #[strum(serialize = "6cm")]
    Centimetres6,
    /// 3 centimetre band: 10000–10500 MHz.
    #[strum(serialize = "3cm")]
    Centimetres3,
    /// 1.25 centimetre band: 24000–24250 MHz.
    #[strum(serialize = "1.25cm")]
    Centimetres1_25,
    /// 6 millimetre band: 47000–47200 MHz.
    #[strum(serialize = "6mm")]
    Millimetres6,
    /// 4 millimetre band: 75500–81000 MHz.
    #[strum(serialize = "4mm")]
    Millimetres4,
    /// 2.5 millimetre band: 119980–123000 MHz.
    #[strum(serialize = "2.5mm")]
    Millimetres2_5,
    /// 2 millimetre band: 134000–149000 MHz.
    #[strum(serialize = "2mm")]
    Millimetres2,
    /// 1 millimetre band: 241000–250000 MHz.
    #[strum(serialize = "1mm")]
    Millimetres1,
    /// Submillimetre band: 300000–7500000 MHz.
    #[strum(serialize = "submm")]
    Submillimetre,
}

impl Band {
    /// The canonical ADIF spelling.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// ADIF's lower and upper band frequencies, in MHz.
    pub fn frequency_bounds_mhz(self) -> (BigDecimal, BigDecimal) {
        let (lower, lower_scale, upper, upper_scale) = match self {
            Self::Metres2190 => (1357, 4, 1378, 4),
            Self::Metres630 => (472, 3, 479, 3),
            Self::Metres560 => (501, 3, 504, 3),
            Self::Metres160 => (18, 1, 20, 1),
            Self::Metres80 => (35, 1, 40, 1),
            Self::Metres60 => (506, 2, 545, 2),
            Self::Metres40 => (70, 1, 73, 1),
            Self::Metres30 => (101, 1, 1015, 2),
            Self::Metres20 => (140, 1, 1435, 2),
            Self::Metres17 => (18068, 3, 18168, 3),
            Self::Metres15 => (210, 1, 2145, 2),
            Self::Metres12 => (24890, 3, 2499, 2),
            Self::Metres10 => (280, 1, 297, 1),
            Self::Metres8 => (40, 0, 45, 0),
            Self::Metres6 => (50, 0, 54, 0),
            Self::Metres5 => (54000001, 6, 699, 1),
            Self::Metres4 => (70, 0, 71, 0),
            Self::Metres2 => (144, 0, 148, 0),
            Self::Metres1_25 => (222, 0, 225, 0),
            Self::Centimetres70 => (420, 0, 450, 0),
            Self::Centimetres33 => (902, 0, 928, 0),
            Self::Centimetres23 => (1240, 0, 1300, 0),
            Self::Centimetres13 => (2300, 0, 2450, 0),
            Self::Centimetres9 => (3300, 0, 3500, 0),
            Self::Centimetres6 => (5650, 0, 5925, 0),
            Self::Centimetres3 => (10000, 0, 10500, 0),
            Self::Centimetres1_25 => (24000, 0, 24250, 0),
            Self::Millimetres6 => (47000, 0, 47200, 0),
            Self::Millimetres4 => (75500, 0, 81000, 0),
            Self::Millimetres2_5 => (119980, 0, 123000, 0),
            Self::Millimetres2 => (134000, 0, 149000, 0),
            Self::Millimetres1 => (241000, 0, 250000, 0),
            Self::Submillimetre => (300000, 0, 7500000, 0),
        };

        (
            BigDecimal::new(lower.into(), lower_scale),
            BigDecimal::new(upper.into(), upper_scale),
        )
    }
}
