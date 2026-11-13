use strum::{EnumIter, EnumString, IntoStaticStr};

/// Award identifiers from ADIF's import-only Award enumeration.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum Award {
    /// ADIF `AJA` identifier.
    #[strum(serialize = "AJA")]
    Aja,
    /// ADIF `CQDX` identifier.
    #[strum(serialize = "CQDX")]
    Cqdx,
    /// ADIF `CQDXFIELD` identifier.
    #[strum(serialize = "CQDXFIELD")]
    Cqdxfield,
    /// ADIF `CQWAZ_MIXED` identifier.
    #[strum(serialize = "CQWAZ_MIXED")]
    CqwazMixed,
    /// ADIF `CQWAZ_CW` identifier.
    #[strum(serialize = "CQWAZ_CW")]
    CqwazCw,
    /// ADIF `CQWAZ_PHONE` identifier.
    #[strum(serialize = "CQWAZ_PHONE")]
    CqwazPhone,
    /// ADIF `CQWAZ_RTTY` identifier.
    #[strum(serialize = "CQWAZ_RTTY")]
    CqwazRtty,
    /// ADIF `CQWAZ_160m` identifier.
    #[strum(serialize = "CQWAZ_160m")]
    Cqwaz160m,
    /// ADIF `CQWPX` identifier.
    #[strum(serialize = "CQWPX")]
    Cqwpx,
    /// ADIF `DARC_DOK` identifier.
    #[strum(serialize = "DARC_DOK")]
    DarcDok,
    /// ADIF `DXCC` identifier.
    #[strum(serialize = "DXCC")]
    Dxcc,
    /// ADIF `DXCC_MIXED` identifier.
    #[strum(serialize = "DXCC_MIXED")]
    DxccMixed,
    /// ADIF `DXCC_CW` identifier.
    #[strum(serialize = "DXCC_CW")]
    DxccCw,
    /// ADIF `DXCC_PHONE` identifier.
    #[strum(serialize = "DXCC_PHONE")]
    DxccPhone,
    /// ADIF `DXCC_RTTY` identifier.
    #[strum(serialize = "DXCC_RTTY")]
    DxccRtty,
    /// ADIF `IOTA` identifier.
    #[strum(serialize = "IOTA")]
    Iota,
    /// ADIF `JCC` identifier.
    #[strum(serialize = "JCC")]
    Jcc,
    /// ADIF `JCG` identifier.
    #[strum(serialize = "JCG")]
    Jcg,
    /// ADIF `MARATHON` identifier.
    #[strum(serialize = "MARATHON")]
    Marathon,
    /// ADIF `RDA` identifier.
    #[strum(serialize = "RDA")]
    Rda,
    /// ADIF `WAB` identifier.
    #[strum(serialize = "WAB")]
    Wab,
    /// ADIF `WAC` identifier.
    #[strum(serialize = "WAC")]
    Wac,
    /// ADIF `WAE` identifier.
    #[strum(serialize = "WAE")]
    Wae,
    /// ADIF `WAIP` identifier.
    #[strum(serialize = "WAIP")]
    Waip,
    /// ADIF `WAJA` identifier.
    #[strum(serialize = "WAJA")]
    Waja,
    /// ADIF `WAS` identifier.
    #[strum(serialize = "WAS")]
    Was,
    /// ADIF `WAZ` identifier.
    #[strum(serialize = "WAZ")]
    Waz,
    /// ADIF `USACA` identifier.
    #[strum(serialize = "USACA")]
    Usaca,
    /// ADIF `VUCC` identifier.
    #[strum(serialize = "VUCC")]
    Vucc,
}

impl Award {
    /// Returns the ADIF Award identifier, retaining its specified spelling.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}
