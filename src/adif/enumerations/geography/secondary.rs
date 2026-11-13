use std::borrow::Cow;

use strum::{EnumDiscriminants, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

use crate::adif::primary_subdivisions::united_states::PrimarySubdivision as UsState;

/// An Alaska county name from ADIF 3.1.8 section III.B.22.a.
///
/// Includes current and deleted entries.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum AlaskaCounty {
    /// Aleutians East, Alaska; Third Judicial District.
    #[strum(serialize = "Aleutians East")]
    AleutiansEast,
    /// Aleutians Islands, Alaska; deleted; Third Judicial District.
    #[strum(serialize = "Aleutians Islands")]
    AleutiansIslands,
    /// Aleutians West, Alaska; Third Judicial District.
    #[strum(serialize = "Aleutians West")]
    AleutiansWest,
    /// Anchorage, Alaska; Third Judicial District.
    #[strum(serialize = "Anchorage")]
    Anchorage,
    /// Angoon, Alaska; deleted; First Judicial District.
    #[strum(serialize = "Angoon")]
    Angoon,
    /// Barrow, Alaska; deleted; Second Judicial District.
    #[strum(serialize = "Barrow")]
    Barrow,
    /// Bethel, Alaska; Fourth Judicial District.
    #[strum(serialize = "Bethel")]
    Bethel,
    /// Bristol Bay, Alaska; Third Judicial District.
    #[strum(serialize = "Bristol Bay")]
    BristolBay,
    /// Cordova-McCarthy, Alaska; deleted; Third Judicial District.
    #[strum(serialize = "Cordova-McCarthy")]
    CordovaMcCarthy,
    /// Denali, Alaska; Fourth Judicial District.
    #[strum(serialize = "Denali")]
    Denali,
    /// Dillingham, Alaska; Third Judicial District.
    #[strum(serialize = "Dillingham")]
    Dillingham,
    /// Fairbanks, Alaska; Fourth Judicial District.
    #[strum(serialize = "Fairbanks")]
    Fairbanks,
    /// Fairbanks North Star, Alaska; Fourth Judicial District.
    #[strum(serialize = "Fairbanks North Star")]
    FairbanksNorthStar,
    /// First Judicial District, Alaska; deleted; First Judicial District.
    #[strum(serialize = "First Judicial District")]
    FirstJudicialDistrict,
    /// Fourth Judicial District, Alaska; deleted; Fourth Judicial District.
    #[strum(serialize = "Fourth Judicial District")]
    FourthJudicialDistrict,
    /// Haines, Alaska; First Judicial District.
    #[strum(serialize = "Haines")]
    Haines,
    /// Juneau, Alaska; First Judicial District.
    #[strum(serialize = "Juneau")]
    Juneau,
    /// Hoonah-Angoon, Alaska; First Judicial District.
    #[strum(serialize = "Hoonah-Angoon")]
    HoonahAngoon,
    /// Kenai Peninsula, Alaska; Third Judicial District.
    #[strum(serialize = "Kenai Peninsula")]
    KenaiPeninsula,
    /// Kenai-Cook Inlet, Alaska; deleted; Third Judicial District.
    #[strum(serialize = "Kenai-Cook Inlet")]
    KenaiCookInlet,
    /// Ketchikan, Alaska; deleted; First Judicial District.
    #[strum(serialize = "Ketchikan")]
    Ketchikan,
    /// Ketchikan Gateway, Alaska; First Judicial District.
    #[strum(serialize = "Ketchikan Gateway")]
    KetchikanGateway,
    /// Kobuk, Alaska; deleted; Second Judicial District.
    #[strum(serialize = "Kobuk")]
    Kobuk,
    /// Kodiak Island, Alaska; Third Judicial District.
    #[strum(serialize = "Kodiak Island")]
    KodiakIsland,
    /// Kuskokwim, Alaska; deleted; Fourth Judicial District.
    #[strum(serialize = "Kuskokwim")]
    Kuskokwim,
    /// Kusilvak, Alaska; Second Judicial District.
    #[strum(serialize = "Kusilvak")]
    Kusilvak,
    /// Lake and Peninsula, Alaska; Third Judicial District.
    #[strum(serialize = "Lake and Peninsula")]
    LakeAndPeninsula,
    /// Lynn Canal-Icy Straits, Alaska; deleted; First Judicial District.
    #[strum(serialize = "Lynn Canal-Icy Straits")]
    LynnCanalIcyStraits,
    /// Matanuska-Susitna, Alaska; Third Judicial District.
    #[strum(serialize = "Matanuska-Susitna")]
    MatanuskaSusitna,
    /// Nome, Alaska; Second Judicial District.
    #[strum(serialize = "Nome")]
    Nome,
    /// North Slope, Alaska; Second Judicial District.
    #[strum(serialize = "North Slope")]
    NorthSlope,
    /// Northwest Arctic, Alaska; Second Judicial District.
    #[strum(serialize = "Northwest Arctic")]
    NorthwestArctic,
    /// Outer Ketchikan, Alaska; deleted; First Judicial District.
    #[strum(serialize = "Outer Ketchikan")]
    OuterKetchikan,
    /// Palmer-Wasilla-Talkeetna, Alaska; deleted; Third Judicial District.
    #[strum(serialize = "Palmer-Wasilla-Talkeetna")]
    PalmerWasillaTalkeetna,
    /// Petersburg, Alaska; First Judicial District.
    #[strum(serialize = "Petersburg")]
    Petersburg,
    /// Pribilof Islands, Alaska; Third Judicial District.
    #[strum(serialize = "Pribilof Islands")]
    PribilofIslands,
    /// Prince of Wales, Alaska; deleted; First Judicial District.
    #[strum(serialize = "Prince of Wales")]
    PrinceOfWales,
    /// Prince of Wales-Hyder, Alaska; First Judicial District.
    #[strum(serialize = "Prince of Wales-Hyder")]
    PrinceOfWalesHyder,
    /// Prince of Wales-Outer Ketchikan, Alaska; deleted; First Judicial District.
    #[strum(serialize = "Prince of Wales-Outer Ketchikan")]
    PrinceOfWalesOuterKetchikan,
    /// Saint Matthew Island, Alaska; Fourth Judicial District.
    #[strum(serialize = "Saint Matthew Island")]
    SaintMatthewIsland,
    /// Second Judicial District, Alaska; deleted; Second Judicial District.
    #[strum(serialize = "Second Judicial District")]
    SecondJudicialDistrict,
    /// Seward, Alaska; deleted; Third Judicial District.
    #[strum(serialize = "Seward")]
    Seward,
    /// Sitka, Alaska; First Judicial District.
    #[strum(serialize = "Sitka")]
    Sitka,
    /// Skagway-Hoonah-Angoon, Alaska; deleted; First Judicial District.
    #[strum(serialize = "Skagway-Hoonah-Angoon")]
    SkagwayHoonahAngoon,
    /// Skagway, Alaska; First Judicial District.
    #[strum(serialize = "Skagway")]
    Skagway,
    /// Skagway-Yakutat, Alaska; deleted; First Judicial District.
    ///
    /// The ADIF county spelling is `Skagway-Yakuta`, without the final `t`.
    #[strum(serialize = "Skagway-Yakuta")]
    SkagwayYakutat,
    /// Skagway-Yakutat-Angoon, Alaska; deleted; First Judicial District.
    #[strum(serialize = "Skagway-Yakutat-Angoon")]
    SkagwayYakutatAngoon,
    /// Southeast Fairbanks, Alaska; Fourth Judicial District.
    #[strum(serialize = "Southeast Fairbanks")]
    SoutheastFairbanks,
    /// Third Judicial District, Alaska; deleted.
    ///
    /// The ADIF source classifies this identity under the Second Judicial District.
    #[strum(serialize = "Third Judicial District")]
    ThirdJudicialDistrict,
    /// Upper Yukon, Alaska; deleted; Fourth Judicial District.
    #[strum(serialize = "Upper Yukon")]
    UpperYukon,
    /// Valdez-Chitina-Whittier, Alaska; deleted; Third Judicial District.
    #[strum(serialize = "Valdez-Chitina-Whittier")]
    ValdezChitinaWhittier,
    /// Valdez-Cordova, Alaska; Third Judicial District.
    #[strum(serialize = "Valdez-Cordova")]
    ValdezCordova,
    /// Wade Hampton, Alaska; deleted; Second Judicial District.
    #[strum(serialize = "Wade Hampton")]
    WadeHampton,
    /// Wales-Hyder, Alaska; First Judicial District.
    #[strum(serialize = "Wales-Hyder")]
    WalesHyder,
    /// Wrangell, Alaska; First Judicial District.
    #[strum(serialize = "Wrangell")]
    Wrangell,
    /// Wrangell-Petersburg, Alaska; deleted; First Judicial District.
    #[strum(serialize = "Wrangell-Petersburg")]
    WrangellPetersburg,
    /// Yakutat, Alaska; First Judicial District.
    #[strum(serialize = "Yakutat")]
    Yakutat,
    /// Yukon-Koyukuk, Alaska; Fourth Judicial District.
    #[strum(serialize = "Yukon-Koyukuk")]
    YukonKoyukuk,
}

impl AlaskaCounty {
    /// Returns the canonical ADIF county name.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// A secondary-subdivision code paired with an ADIF group.
#[derive(Debug, Clone, PartialEq, Eq, EnumDiscriminants)]
#[strum_discriminants(name(SecondarySubdivisionKind))]
#[strum_discriminants(vis(pub(crate)))]
#[strum_discriminants(derive(EnumIter))]
pub enum SecondarySubdivision<'a> {
    /// An Alaska county.
    Alaska(AlaskaCounty),
    /// A Hawaii county name.
    Hawaii(Cow<'a, str>),
    /// A state and county in the United States group.
    UnitedStates {
        /// The state identified by the input prefix.
        state: UsState,
        /// The county name after the comma.
        county: Cow<'a, str>,
    },
    /// An Asiatic Russia district code.
    AsiaticRussia(Cow<'a, str>),
    /// A European Russia district code.
    EuropeanRussia(Cow<'a, str>),
    /// A Franz Josef Land district code.
    FranzJosefLand(Cow<'a, str>),
    /// A Kaliningrad district code.
    Kaliningrad(Cow<'a, str>),
    /// A Malyj Vysotskij Island district code.
    MalyjVysotskijIsland(Cow<'a, str>),
    /// A Ukrainian district code.
    Ukraine(Cow<'a, str>),
    /// A Japanese city, ku, or gun code.
    Japan(Cow<'a, str>),
    /// A New Zealand county code.
    NewZealand(Cow<'a, str>),
    /// A Balearic Islands DME code.
    BalearicIslands(Cow<'a, str>),
    /// A Canary Islands DME code.
    CanaryIslands(Cow<'a, str>),
    /// A Ceuta and Melilla DME code.
    CeutaAndMelilla(Cow<'a, str>),
    /// A Spain DME code.
    Spain(Cow<'a, str>),
}

impl<'a> SecondarySubdivision<'a> {
    /// Finds interpretations supported by ADIF's formats and supplied membership.
    ///
    /// County prefixes dispatch to a single parser; a failed nested parse has no
    /// fallback. Unprefixed codes can yield multiple format-compatible candidates.
    /// Matching ignores ASCII case without trimming. Payloads borrow the input;
    /// text repertoire checks belong to the selected parsing encoding.
    pub fn candidates(code: &'a str) -> impl Iterator<Item = Self> {
        let (county, unprefixed) = match code.split_once(',') {
            Some((prefix, county)) => (Self::parse_county(prefix, county), None),
            None => (None, Some(code)),
        };

        county.into_iter().chain(
            SecondarySubdivisionKind::iter()
                .filter_map(move |kind| kind.parse_unprefixed(unprefixed?)),
        )
    }

    fn parse_county(prefix: &str, county: &'a str) -> Option<Self> {
        if county.is_empty() {
            return None;
        }

        if prefix.eq_ignore_ascii_case("AK") {
            county.parse().ok().map(Self::Alaska)
        } else if prefix.eq_ignore_ascii_case("HI") {
            Some(Self::Hawaii(Cow::Borrowed(county)))
        } else {
            prefix.parse().ok().map(|state| Self::UnitedStates {
                state,
                county: Cow::Borrowed(county),
            })
        }
    }
}

impl SecondarySubdivisionKind {
    fn parse_unprefixed(self, code: &str) -> Option<SecondarySubdivision<'_>> {
        use SecondarySubdivision as S;

        let district = || {
            let bytes = code.as_bytes();
            bytes.len() == 5
                && bytes[..2].iter().all(u8::is_ascii_alphabetic)
                && bytes[2] == b'-'
                && bytes[3..].iter().all(u8::is_ascii_digit)
        };
        let digits = |widths: &[usize]| {
            widths.contains(&code.len()) && code.bytes().all(|byte| byte.is_ascii_digit())
        };
        let text = Cow::Borrowed(code);

        match self {
            Self::AsiaticRussia if district() => Some(S::AsiaticRussia(text)),
            Self::EuropeanRussia if district() => Some(S::EuropeanRussia(text)),
            Self::FranzJosefLand if district() => Some(S::FranzJosefLand(text)),
            Self::Kaliningrad if district() => Some(S::Kaliningrad(text)),
            Self::MalyjVysotskijIsland if district() => Some(S::MalyjVysotskijIsland(text)),
            Self::Ukraine if district() => Some(S::Ukraine(text)),
            Self::Japan if digits(&[4, 5, 6]) => Some(S::Japan(text)),
            Self::NewZealand if digits(&[3]) => Some(S::NewZealand(text)),
            Self::BalearicIslands if digits(&[5]) => Some(S::BalearicIslands(text)),
            Self::CanaryIslands if digits(&[5]) => Some(S::CanaryIslands(text)),
            Self::CeutaAndMelilla if digits(&[5]) => Some(S::CeutaAndMelilla(text)),
            Self::Spain if digits(&[5]) => Some(S::Spain(text)),
            _ => None,
        }
    }
}
