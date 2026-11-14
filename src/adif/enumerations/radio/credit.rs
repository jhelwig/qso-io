use strum::{EnumIter, EnumString, IntoStaticStr};

/// An award sponsor identified by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum AwardSponsor {
    /// ADIF Development Group.
    #[strum(serialize = "ADIF")]
    Adif,
    /// Associazione Radioamatori Italiani.
    #[strum(serialize = "ARI")]
    Ari,
    /// American Radio Relay League.
    #[strum(serialize = "ARRL")]
    Arrl,
    /// CQ Magazine.
    #[strum(serialize = "CQ")]
    Cq,
    /// Deutscher Amateur-Radio-Club e.V.
    #[strum(serialize = "DARC")]
    Darc,
    /// eQSL.
    #[strum(serialize = "EQSL")]
    Eqsl,
    /// International Amateur Radio Union.
    #[strum(serialize = "IARU")]
    Iaru,
    /// Japan Amateur Radio League.
    #[strum(serialize = "JARL")]
    Jarl,
    /// Radio Society of Great Britain.
    #[strum(serialize = "RSGB")]
    Rsgb,
    /// Tambov award group.
    #[strum(serialize = "TAG")]
    Tag,
    /// Worked All Britain award group.
    #[strum(serialize = "WABAG")]
    WabAg,
}

impl AwardSponsor {
    /// Returns the canonical sponsor identifier.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// An award and facet for which a contact may receive ADIF credit.
///
/// Credit values distinguish, for example, mixed DXCC from its band, mode and
/// satellite facets. They are not interchangeable with award sponsors
/// or the legacy `AWARD` enumeration.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum Credit {
    /// CQ Magazine DX award, mixed facet.
    #[strum(serialize = "CQDX")]
    CqDx,
    /// CQ Magazine DX award, band facet.
    #[strum(serialize = "CQDX_BAND")]
    CqDxBand,
    /// CQ Magazine DX award, mode facet.
    #[strum(serialize = "CQDX_MODE")]
    CqDxMode,
    /// CQ Magazine DX award, mobile facet.
    #[strum(serialize = "CQDX_MOBILE")]
    CqDxMobile,
    /// CQ Magazine DX award, QRP facet.
    #[strum(serialize = "CQDX_QRP")]
    CqDxQrp,
    /// CQ Magazine DX award, satellite facet.
    #[strum(serialize = "CQDX_SATELLITE")]
    CqDxSatellite,
    /// CQ Magazine DX Field award, mixed facet.
    #[strum(serialize = "CQDXFIELD")]
    CqDxField,
    /// CQ Magazine DX Field award, band facet.
    #[strum(serialize = "CQDXFIELD_BAND")]
    CqDxFieldBand,
    /// CQ Magazine DX Field award, mode facet.
    #[strum(serialize = "CQDXFIELD_MODE")]
    CqDxFieldMode,
    /// CQ Magazine DX Field award, mobile facet.
    #[strum(serialize = "CQDXFIELD_MOBILE")]
    CqDxFieldMobile,
    /// CQ Magazine DX Field award, QRP facet.
    #[strum(serialize = "CQDXFIELD_QRP")]
    CqDxFieldQrp,
    /// CQ Magazine DX Field award, satellite facet.
    #[strum(serialize = "CQDXFIELD_SATELLITE")]
    CqDxFieldSatellite,
    /// CQ Magazine Worked All Zones award, mixed facet.
    #[strum(serialize = "CQWAZ_MIXED")]
    CqWazMixed,
    /// CQ Magazine Worked All Zones award, band facet.
    #[strum(serialize = "CQWAZ_BAND")]
    CqWazBand,
    /// CQ Magazine Worked All Zones award, mode facet.
    #[strum(serialize = "CQWAZ_MODE")]
    CqWazMode,
    /// CQ Magazine Worked All Zones award, satellite facet.
    #[strum(serialize = "CQWAZ_SATELLITE")]
    CqWazSatellite,
    /// CQ Magazine Worked All Zones award, Earth–Moon–Earth facet.
    #[strum(serialize = "CQWAZ_EME")]
    CqWazEme,
    /// CQ Magazine Worked All Zones award, mobile facet.
    #[strum(serialize = "CQWAZ_MOBILE")]
    CqWazMobile,
    /// CQ Magazine Worked All Zones award, QRP facet.
    #[strum(serialize = "CQWAZ_QRP")]
    CqWazQrp,
    /// CQ Magazine WPX award, mixed facet.
    #[strum(serialize = "CQWPX")]
    CqWpx,
    /// CQ Magazine WPX award, band facet.
    #[strum(serialize = "CQWPX_BAND")]
    CqWpxBand,
    /// CQ Magazine WPX award, mode facet.
    #[strum(serialize = "CQWPX_MODE")]
    CqWpxMode,
    /// ARRL DX Century Club award, mixed facet.
    #[strum(serialize = "DXCC")]
    Dxcc,
    /// ARRL DX Century Club award, band facet.
    #[strum(serialize = "DXCC_BAND")]
    DxccBand,
    /// ARRL DX Century Club award, mode facet.
    #[strum(serialize = "DXCC_MODE")]
    DxccMode,
    /// ARRL DX Century Club award, satellite facet.
    #[strum(serialize = "DXCC_SATELLITE")]
    DxccSatellite,
    /// eQSL eAustralia award, mixed facet.
    #[strum(serialize = "EAUSTRALIA")]
    EAustralia,
    /// eQSL eCanada award, mixed facet.
    #[strum(serialize = "ECANADA")]
    ECanada,
    /// eQSL eCounty award, state facet.
    #[strum(serialize = "ECOUNTY_STATE")]
    ECountyState,
    /// eQSL eDX award, mixed facet.
    #[strum(serialize = "EDX")]
    EDx,
    /// eQSL eDX100 award, mixed facet.
    #[strum(serialize = "EDX100")]
    EDx100,
    /// eQSL eDX100 award, band facet.
    #[strum(serialize = "EDX100_BAND")]
    EDx100Band,
    /// eQSL eDX100 award, mode facet.
    #[strum(serialize = "EDX100_MODE")]
    EDx100Mode,
    /// eQSL eEcholink50 award, Echolink facet.
    #[strum(serialize = "EECHOLINK50")]
    EEcholink50,
    /// eQSL eGrid award, band facet.
    #[strum(serialize = "EGRID_BAND")]
    EGridBand,
    /// eQSL eGrid award, satellite facet.
    #[strum(serialize = "EGRID_SATELLITE")]
    EGridSatellite,
    /// eQSL ePfx300 award, mixed facet.
    #[strum(serialize = "EPFX300")]
    EPfx300,
    /// eQSL ePfx300 award, mode facet.
    #[strum(serialize = "EPFX300_MODE")]
    EPfx300Mode,
    /// eQSL eWAS award, mixed facet.
    #[strum(serialize = "EWAS")]
    EWas,
    /// eQSL eWAS award, band facet.
    #[strum(serialize = "EWAS_BAND")]
    EWasBand,
    /// eQSL eWAS award, mode facet.
    #[strum(serialize = "EWAS_MODE")]
    EWasMode,
    /// eQSL eWAS award, satellite facet.
    #[strum(serialize = "EWAS_SATELLITE")]
    EWasSatellite,
    /// eQSL eZ40 award, mixed facet.
    #[strum(serialize = "EZ40")]
    EZ40,
    /// eQSL eZ40 award, mode facet.
    #[strum(serialize = "EZ40_MODE")]
    EZ40Mode,
    /// ARRL Fred Fish Memorial Award, mixed facet.
    #[strum(serialize = "FFMA")]
    Ffma,
    /// RSGB Islands on the Air award, mixed facet.
    #[strum(serialize = "IOTA")]
    Iota,
    /// RSGB Islands on the Air basic credit, mixed facet.
    #[strum(serialize = "IOTA_BASIC")]
    IotaBasic,
    /// RSGB Islands on the Air award, continent facet.
    #[strum(serialize = "IOTA_CONT")]
    IotaContinent,
    /// RSGB Islands on the Air award, group facet.
    #[strum(serialize = "IOTA_GROUP")]
    IotaGroup,
    /// Tambov award group's Russian Districts Award, mixed facet.
    #[strum(serialize = "RDA")]
    Rda,
    /// CQ Magazine United States of America Counties award, mixed facet.
    #[strum(serialize = "USACA")]
    Usaca,
    /// ARRL VHF/UHF Century Club Program, band facet.
    #[strum(serialize = "VUCC_BAND")]
    VuccBand,
    /// ARRL VHF/UHF Century Club Program, satellite facet.
    #[strum(serialize = "VUCC_SATELLITE")]
    VuccSatellite,
    /// Worked All Britain award group award, mixed facet.
    #[strum(serialize = "WAB")]
    Wab,
    /// IARU Worked All Continents award, mixed facet.
    #[strum(serialize = "WAC")]
    Wac,
    /// IARU Worked All Continents award, band facet.
    #[strum(serialize = "WAC_BAND")]
    WacBand,
    /// DARC Worked All Europe award, mixed facet.
    #[strum(serialize = "WAE")]
    Wae,
    /// DARC Worked All Europe award, band facet.
    #[strum(serialize = "WAE_BAND")]
    WaeBand,
    /// DARC Worked All Europe award, mode facet.
    #[strum(serialize = "WAE_MODE")]
    WaeMode,
    /// ARI Worked All Italian Provinces award, mixed facet.
    #[strum(serialize = "WAIP")]
    Waip,
    /// ARI Worked All Italian Provinces award, band facet.
    #[strum(serialize = "WAIP_BAND")]
    WaipBand,
    /// ARI Worked All Italian Provinces award, mode facet.
    #[strum(serialize = "WAIP_MODE")]
    WaipMode,
    /// ARRL Worked All States award, mixed facet.
    #[strum(serialize = "WAS")]
    Was,
    /// ARRL Worked All States award, band facet.
    #[strum(serialize = "WAS_BAND")]
    WasBand,
    /// ARRL Worked All States award, Earth–Moon–Earth facet.
    #[strum(serialize = "WAS_EME")]
    WasEme,
    /// ARRL Worked All States award, mode facet.
    #[strum(serialize = "WAS_MODE")]
    WasMode,
    /// ARRL Worked All States award, novice facet.
    #[strum(serialize = "WAS_NOVICE")]
    WasNovice,
    /// ARRL Worked All States award, QRP facet.
    #[strum(serialize = "WAS_QRP")]
    WasQrp,
    /// ARRL Worked All States award, satellite facet.
    #[strum(serialize = "WAS_SATELLITE")]
    WasSatellite,
    /// RSGB Worked ITU Zones award, mixed facet.
    #[strum(serialize = "WITUZ")]
    Wituz,
    /// RSGB Worked ITU Zones award, band facet.
    #[strum(serialize = "WITUZ_BAND")]
    WituzBand,
}

impl Credit {
    /// The canonical ADIF spelling.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// The sponsoring organization specified by ADIF for this credit.
    pub const fn sponsor(self) -> AwardSponsor {
        match self {
            Self::CqDx
            | Self::CqDxBand
            | Self::CqDxMode
            | Self::CqDxMobile
            | Self::CqDxQrp
            | Self::CqDxSatellite
            | Self::CqDxField
            | Self::CqDxFieldBand
            | Self::CqDxFieldMode
            | Self::CqDxFieldMobile
            | Self::CqDxFieldQrp
            | Self::CqDxFieldSatellite
            | Self::CqWazMixed
            | Self::CqWazBand
            | Self::CqWazMode
            | Self::CqWazSatellite
            | Self::CqWazEme
            | Self::CqWazMobile
            | Self::CqWazQrp
            | Self::CqWpx
            | Self::CqWpxBand
            | Self::CqWpxMode
            | Self::Usaca => AwardSponsor::Cq,
            Self::Dxcc
            | Self::DxccBand
            | Self::DxccMode
            | Self::DxccSatellite
            | Self::Ffma
            | Self::VuccBand
            | Self::VuccSatellite
            | Self::Was
            | Self::WasBand
            | Self::WasEme
            | Self::WasMode
            | Self::WasNovice
            | Self::WasQrp
            | Self::WasSatellite => AwardSponsor::Arrl,
            Self::EAustralia
            | Self::ECanada
            | Self::ECountyState
            | Self::EDx
            | Self::EDx100
            | Self::EDx100Band
            | Self::EDx100Mode
            | Self::EEcholink50
            | Self::EGridBand
            | Self::EGridSatellite
            | Self::EPfx300
            | Self::EPfx300Mode
            | Self::EWas
            | Self::EWasBand
            | Self::EWasMode
            | Self::EWasSatellite
            | Self::EZ40
            | Self::EZ40Mode => AwardSponsor::Eqsl,
            Self::Iota
            | Self::IotaBasic
            | Self::IotaContinent
            | Self::IotaGroup
            | Self::Wituz
            | Self::WituzBand => AwardSponsor::Rsgb,
            Self::Rda => AwardSponsor::Tag,
            Self::Wab => AwardSponsor::WabAg,
            Self::Wac | Self::WacBand => AwardSponsor::Iaru,
            Self::Wae | Self::WaeBand | Self::WaeMode => AwardSponsor::Darc,
            Self::Waip | Self::WaipBand | Self::WaipMode => AwardSponsor::Ari,
        }
    }

    /// The award facet spelled as in ADIF's Credit table.
    pub const fn facet(self) -> &'static str {
        match self {
            Self::CqDx
            | Self::CqDxField
            | Self::CqWazMixed
            | Self::CqWpx
            | Self::Dxcc
            | Self::EAustralia
            | Self::ECanada
            | Self::EDx
            | Self::EDx100
            | Self::EPfx300
            | Self::EWas
            | Self::EZ40
            | Self::Ffma
            | Self::Iota
            | Self::IotaBasic
            | Self::Rda
            | Self::Usaca
            | Self::Wab
            | Self::Wac
            | Self::Wae
            | Self::Waip
            | Self::Was
            | Self::Wituz => "Mixed",
            Self::CqDxBand
            | Self::CqDxFieldBand
            | Self::CqWazBand
            | Self::CqWpxBand
            | Self::DxccBand
            | Self::EDx100Band
            | Self::EGridBand
            | Self::EWasBand
            | Self::VuccBand
            | Self::WacBand
            | Self::WaeBand
            | Self::WaipBand
            | Self::WasBand
            | Self::WituzBand => "Band",
            Self::CqDxMode
            | Self::CqDxFieldMode
            | Self::CqWazMode
            | Self::CqWpxMode
            | Self::DxccMode
            | Self::EDx100Mode
            | Self::EPfx300Mode
            | Self::EWasMode
            | Self::EZ40Mode
            | Self::WaeMode
            | Self::WaipMode
            | Self::WasMode => "Mode",
            Self::CqDxMobile | Self::CqDxFieldMobile | Self::CqWazMobile => "Mobile",
            Self::CqDxQrp | Self::CqDxFieldQrp | Self::CqWazQrp | Self::WasQrp => "QRP",
            Self::CqDxSatellite
            | Self::CqDxFieldSatellite
            | Self::CqWazSatellite
            | Self::DxccSatellite
            | Self::EGridSatellite
            | Self::EWasSatellite
            | Self::VuccSatellite
            | Self::WasSatellite => "Satellite",
            Self::CqWazEme | Self::WasEme => "EME",
            Self::ECountyState => "State",
            Self::EEcholink50 => "Echolink",
            Self::IotaContinent => "Continent",
            Self::IotaGroup => "Group",
            Self::WasNovice => "Novice",
        }
    }
}
