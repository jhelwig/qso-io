use std::borrow::Cow;

use strum::{EnumIter, EnumString, IntoStaticStr};

/// A New Zealand region/district pair in ADIF 3.1.8 section III.B.23.a.
///
/// Codes contain the region and district separated by a slash.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum NzRegionDistrict {
    /// Far North district, Northland region, New Zealand.
    #[strum(serialize = "Northland/Far North")]
    NorthlandFarNorth,
    /// Whangarei district, Northland region, New Zealand.
    #[strum(serialize = "Northland/Whangarei")]
    NorthlandWhangarei,
    /// Kaipara district, Northland region, New Zealand.
    #[strum(serialize = "Northland/Kaipara")]
    NorthlandKaipara,
    /// Rodney district, Auckland region, New Zealand.
    #[strum(serialize = "Auckland/Rodney")]
    AucklandRodney,
    /// North Shore district, Auckland region, New Zealand.
    #[strum(serialize = "Auckland/North Shore")]
    AucklandNorthShore,
    /// Waitakere district, Auckland region, New Zealand.
    #[strum(serialize = "Auckland/Waitakere")]
    AucklandWaitakere,
    /// Auckland district, Auckland region, New Zealand.
    #[strum(serialize = "Auckland/Auckland")]
    AucklandAuckland,
    /// Manukau district, Auckland region, New Zealand.
    #[strum(serialize = "Auckland/Manukau")]
    AucklandManukau,
    /// Papakura district, Auckland region, New Zealand.
    #[strum(serialize = "Auckland/Papakura")]
    AucklandPapakura,
    /// Franklin district, Auckland region, New Zealand.
    #[strum(serialize = "Auckland/Franklin")]
    AucklandFranklin,
    /// Thames-Coromandel district, Waikato region, New Zealand.
    #[strum(serialize = "Waikato/Thames-Coromandel")]
    WaikatoThamesCoromandel,
    /// Hauraki district, Waikato region, New Zealand.
    #[strum(serialize = "Waikato/Hauraki")]
    WaikatoHauraki,
    /// Waikato district, Waikato region, New Zealand.
    #[strum(serialize = "Waikato/Waikato")]
    WaikatoWaikato,
    /// Matamata Piako district, Waikato region, New Zealand.
    #[strum(serialize = "Waikato/Matamata Piako")]
    WaikatoMatamataPiako,
    /// Hamilton district, Waikato region, New Zealand.
    #[strum(serialize = "Waikato/Hamilton")]
    WaikatoHamilton,
    /// Waipa district, Waikato region, New Zealand.
    #[strum(serialize = "Waikato/Waipa")]
    WaikatoWaipa,
    /// Otorohanga district, Waikato region, New Zealand.
    #[strum(serialize = "Waikato/Otorohanga")]
    WaikatoOtorohanga,
    /// Waitomo district, Waikato region, New Zealand.
    #[strum(serialize = "Waikato/Waitomo")]
    WaikatoWaitomo,
    /// South Waikato district, Waikato region, New Zealand.
    #[strum(serialize = "Waikato/South Waikato")]
    WaikatoSouthWaikato,
    /// Taupo district, Waikato region, New Zealand.
    #[strum(serialize = "Waikato/Taupo")]
    WaikatoTaupo,
    /// Western Bay of Plenty district, Bay of Plenty region, New Zealand.
    #[strum(serialize = "Bay of Plenty/Western Bay of Plenty")]
    BayOfPlentyWesternBayOfPlenty,
    /// Tauranga district, Bay of Plenty region, New Zealand.
    #[strum(serialize = "Bay of Plenty/Tauranga")]
    BayOfPlentyTauranga,
    /// Rotorua district, Bay of Plenty region, New Zealand.
    #[strum(serialize = "Bay of Plenty/Rotorua")]
    BayOfPlentyRotorua,
    /// Whakatane district, Bay of Plenty region, New Zealand.
    #[strum(serialize = "Bay of Plenty/Whakatane")]
    BayOfPlentyWhakatane,
    /// Kawerau district, Bay of Plenty region, New Zealand.
    #[strum(serialize = "Bay of Plenty/Kawerau")]
    BayOfPlentyKawerau,
    /// Opotiki district, Bay of Plenty region, New Zealand.
    #[strum(serialize = "Bay of Plenty/Opotiki")]
    BayOfPlentyOpotiki,
    /// Gisborne district, Gisborne region, New Zealand.
    #[strum(serialize = "Gisborne/Gisborne")]
    GisborneGisborne,
    /// Wairoa district, Hawkes Bay region, New Zealand.
    #[strum(serialize = "Hawkes Bay/Wairoa")]
    HawkesBayWairoa,
    /// Hastings district, Hawkes Bay region, New Zealand.
    #[strum(serialize = "Hawkes Bay/Hastings")]
    HawkesBayHastings,
    /// Napier district, Hawkes Bay region, New Zealand.
    #[strum(serialize = "Hawkes Bay/Napier")]
    HawkesBayNapier,
    /// Central Hawkes Bay district, Hawkes Bay region, New Zealand.
    #[strum(serialize = "Hawkes Bay/Central Hawkes Bay")]
    HawkesBayCentralHawkesBay,
    /// New Plymouth district, Taranaki region, New Zealand.
    #[strum(serialize = "Taranaki/New Plymouth")]
    TaranakiNewPlymouth,
    /// Stratford district, Taranaki region, New Zealand.
    #[strum(serialize = "Taranaki/Stratford")]
    TaranakiStratford,
    /// South Taranaki district, Taranaki region, New Zealand.
    #[strum(serialize = "Taranaki/South Taranaki")]
    TaranakiSouthTaranaki,
    /// Ruapehu district, Wanganui-Manawatu region, New Zealand.
    #[strum(serialize = "Wanganui-Manawatu/Ruapehu")]
    WanganuiManawatuRuapehu,
    /// Wanganui district, Wanganui-Manawatu region, New Zealand.
    #[strum(serialize = "Wanganui-Manawatu/Wanganui")]
    WanganuiManawatuWanganui,
    /// Rangitikei district, Wanganui-Manawatu region, New Zealand.
    #[strum(serialize = "Wanganui-Manawatu/Rangitikei")]
    WanganuiManawatuRangitikei,
    /// Palmerston North district, Wanganui-Manawatu region, New Zealand.
    #[strum(serialize = "Wanganui-Manawatu/Palmerston North")]
    WanganuiManawatuPalmerstonNorth,
    /// Manawatu district, Wanganui-Manawatu region, New Zealand.
    #[strum(serialize = "Wanganui-Manawatu/Manawatu")]
    WanganuiManawatuManawatu,
    /// Tararua district, Wanganui-Manawatu region, New Zealand.
    #[strum(serialize = "Wanganui-Manawatu/Tararua")]
    WanganuiManawatuTararua,
    /// Horowhenua district, Wanganui-Manawatu region, New Zealand.
    #[strum(serialize = "Wanganui-Manawatu/Horowhenua")]
    WanganuiManawatuHorowhenua,
    /// Masterton district, Wellington region, New Zealand.
    #[strum(serialize = "Wellington/Masterton")]
    WellingtonMasterton,
    /// Carterton district, Wellington region, New Zealand.
    #[strum(serialize = "Wellington/Carterton")]
    WellingtonCarterton,
    /// South Wairarapa district, Wellington region, New Zealand.
    #[strum(serialize = "Wellington/South Wairarapa")]
    WellingtonSouthWairarapa,
    /// Kapiti Coast district, Wellington region, New Zealand.
    #[strum(serialize = "Wellington/Kapiti Coast")]
    WellingtonKapitiCoast,
    /// Upper Hutt district, Wellington region, New Zealand.
    #[strum(serialize = "Wellington/Upper Hutt")]
    WellingtonUpperHutt,
    /// Lower Hutt district, Wellington region, New Zealand.
    #[strum(serialize = "Wellington/Lower Hutt")]
    WellingtonLowerHutt,
    /// Porirua district, Wellington region, New Zealand.
    #[strum(serialize = "Wellington/Porirua")]
    WellingtonPorirua,
    /// Wellington district, Wellington region, New Zealand.
    #[strum(serialize = "Wellington/Wellington")]
    WellingtonWellington,
    /// Nelson district, Nelson region, New Zealand.
    #[strum(serialize = "Nelson/Nelson")]
    NelsonNelson,
    /// Marlborough district, Marlborough region, New Zealand.
    #[strum(serialize = "Marlborough/Marlborough")]
    MarlboroughMarlborough,
    /// Tasman district, Tasman region, New Zealand.
    #[strum(serialize = "Tasman/Tasman")]
    TasmanTasman,
    /// Buller district, West Coast region, New Zealand.
    #[strum(serialize = "West Coast/Buller")]
    WestCoastBuller,
    /// Grey district, West Coast region, New Zealand.
    #[strum(serialize = "West Coast/Grey")]
    WestCoastGrey,
    /// Westland district, West Coast region, New Zealand.
    #[strum(serialize = "West Coast/Westland")]
    WestCoastWestland,
    /// Kaikoura district, Canterbury region, New Zealand.
    #[strum(serialize = "Canterbury/Kaikoura")]
    CanterburyKaikoura,
    /// Hurunui district, Canterbury region, New Zealand.
    #[strum(serialize = "Canterbury/Hurunui")]
    CanterburyHurunui,
    /// Selwyn district, Canterbury region, New Zealand.
    #[strum(serialize = "Canterbury/Selwyn")]
    CanterburySelwyn,
    /// Waimakariri district, Canterbury region, New Zealand.
    #[strum(serialize = "Canterbury/Waimakariri")]
    CanterburyWaimakariri,
    /// Christchurch district, Canterbury region, New Zealand.
    #[strum(serialize = "Canterbury/Christchurch")]
    CanterburyChristchurch,
    /// Banks Peninsula district, Canterbury region, New Zealand.
    #[strum(serialize = "Canterbury/Banks Peninsula")]
    CanterburyBanksPeninsula,
    /// Ashburton district, Canterbury region, New Zealand.
    #[strum(serialize = "Canterbury/Ashburton")]
    CanterburyAshburton,
    /// Mackenzie district, Canterbury region, New Zealand.
    #[strum(serialize = "Canterbury/Mackenzie")]
    CanterburyMackenzie,
    /// Timaru district, Canterbury region, New Zealand.
    #[strum(serialize = "Canterbury/Timaru")]
    CanterburyTimaru,
    /// Waimate district, Canterbury region, New Zealand.
    #[strum(serialize = "Canterbury/Waimate")]
    CanterburyWaimate,
    /// Waitaki district, Otago region, New Zealand.
    #[strum(serialize = "Otago/Waitaki")]
    OtagoWaitaki,
    /// Queenstown-Lakes district, Otago region, New Zealand.
    #[strum(serialize = "Otago/Queenstown-Lakes")]
    OtagoQueenstownLakes,
    /// Central Otago district, Otago region, New Zealand.
    #[strum(serialize = "Otago/Central Otago")]
    OtagoCentralOtago,
    /// Dunedin district, Otago region, New Zealand.
    #[strum(serialize = "Otago/Dunedin")]
    OtagoDunedin,
    /// Clutha district, Otago region, New Zealand.
    #[strum(serialize = "Otago/Clutha")]
    OtagoClutha,
    /// Gore district, Southland region, New Zealand.
    #[strum(serialize = "Southland/Gore")]
    SouthlandGore,
    /// Southland district, Southland region, New Zealand.
    #[strum(serialize = "Southland/Southland")]
    SouthlandSouthland,
    /// Invercargill district, Southland region, New Zealand.
    #[strum(serialize = "Southland/Invercargill")]
    SouthlandInvercargill,
}

impl NzRegionDistrict {
    /// Returns the ADIF region/district code.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// An alternate secondary-subdivision entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecondarySubdivisionAlt<'a> {
    /// A New Zealand region and district.
    NzRegions(NzRegionDistrict),
    /// An entry in another enumeration.
    Other {
        /// The enumeration name before the colon.
        enumeration: Cow<'a, str>,
        /// The code after the colon.
        code: Cow<'a, str>,
    },
}

impl<'a> SecondarySubdivisionAlt<'a> {
    /// Parses an enumeration name and its code, separated by a colon.
    ///
    /// NZ_Regions codes require membership in the supplied region/district table.
    /// Other enumeration codes are retained without external membership checks.
    /// Components must be nonempty. Name matching ignores ASCII case without
    /// trimming; text repertoire checks belong to the selected parsing encoding.
    pub fn parse_member(member: &'a str) -> Option<Self> {
        let (name, code) = member.split_once(':')?;
        if name.is_empty() || code.is_empty() {
            return None;
        }

        if name.eq_ignore_ascii_case("NZ_Regions") {
            code.parse().ok().map(Self::NzRegions)
        } else {
            Some(Self::Other {
                enumeration: Cow::Borrowed(name),
                code: Cow::Borrowed(code),
            })
        }
    }
}
