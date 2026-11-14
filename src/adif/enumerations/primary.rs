//! Country-specific primary subdivision enums from ADIF 3.1.8 III.B.12.
//!
//! Each module follows one DXCC-specific source group. Equal wire spellings in
//! different groups remain distinct Rust values.
//!
//! Source: [ADIF III.B.12](https://www.adif.org.uk/318/ADIF_318.htm#Primary_Administrative_Subdivision_Enumeration).

use super::DxccEntity;

pub mod aland_islands;
pub mod alaska;
pub mod andaman_and_nicobar_islands;
pub mod argentina;
pub mod asiatic_russia;
pub mod australia;
pub mod austria;
pub mod azores;
pub mod balearic_islands;
pub mod belarus;
pub mod belgium;
pub mod brazil;
pub mod bulgaria;
pub mod canada;
pub mod canary_islands;
pub mod ceuta_and_melilla;
pub mod chile;
pub mod china;
pub mod corsica;
pub mod croatia;
pub mod cuba;
pub mod czech_republic;
pub mod denmark;
pub mod european_russia;
pub mod finland;
pub mod france;
pub mod franz_josef_land;
pub mod germany;
pub mod hawaii;
pub mod hungary;
pub mod india;
pub mod ireland;
pub mod italy;
pub mod jan_mayen;
pub mod japan;
pub mod kaliningrad;
pub mod kure_island;
pub mod lakshadweep;
pub mod lord_howe_island;
pub mod macquarie_island;
pub mod madeira;
pub mod malyj_vysotskij_island;
pub mod mexico;
pub mod minami_torishima;
pub mod netherlands;
pub mod new_zealand;
pub mod norway;
pub mod ogasawara;
pub mod papua_new_guinea;
pub mod paraguay;
pub mod philippines;
pub mod poland;
pub mod portugal;
pub mod republic_of_korea;
pub mod romania;
pub mod sardinia;
pub mod slovak_republic;
pub mod spain;
pub mod svalbard;
pub mod sweden;
pub mod switzerland;
pub mod taiwan;
pub mod ukraine;
pub mod united_states;
pub mod uruguay;
pub mod venezuela;

impl DxccEntity {
    pub(crate) fn recognizes_primary_subdivision(self, code: &str) -> bool {
        match self {
            Self::Canada => code.parse::<canada::PrimarySubdivision>().is_ok(),
            Self::AlandIslands => code.parse::<aland_islands::PrimarySubdivision>().is_ok(),
            Self::Alaska => code.parse::<alaska::PrimarySubdivision>().is_ok(),
            Self::AndamanAndNicobarIslands => code
                .parse::<andaman_and_nicobar_islands::PrimarySubdivision>()
                .is_ok(),
            Self::AsiaticRussia => code.parse::<asiatic_russia::PrimarySubdivision>().is_ok(),
            Self::BalearicIslands => code.parse::<balearic_islands::PrimarySubdivision>().is_ok(),
            Self::Belarus => code.parse::<belarus::PrimarySubdivision>().is_ok(),
            Self::CanaryIslands => code.parse::<canary_islands::PrimarySubdivision>().is_ok(),
            Self::CeutaAndMelilla => code
                .parse::<ceuta_and_melilla::PrimarySubdivision>()
                .is_ok(),
            Self::Mexico => code.parse::<mexico::PrimarySubdivision>().is_ok(),
            Self::EuropeanRussia => code.parse::<european_russia::PrimarySubdivision>().is_ok(),
            Self::FranzJosefLand => code.parse::<franz_josef_land::PrimarySubdivision>().is_ok(),
            Self::Cuba => code.parse::<cuba::PrimarySubdivision>().is_ok(),
            Self::Argentina => code.parse::<argentina::PrimarySubdivision>().is_ok(),
            Self::Brazil => code.parse::<brazil::PrimarySubdivision>().is_ok(),
            Self::Hawaii => code.parse::<hawaii::PrimarySubdivision>().is_ok(),
            Self::Chile => code.parse::<chile::PrimarySubdivision>().is_ok(),
            Self::JanMayen => code.parse::<jan_mayen::PrimarySubdivision>().is_ok(),
            Self::Kaliningrad => code.parse::<kaliningrad::PrimarySubdivision>().is_ok(),
            Self::Paraguay => code.parse::<paraguay::PrimarySubdivision>().is_ok(),
            Self::RepublicOfKorea => code
                .parse::<republic_of_korea::PrimarySubdivision>()
                .is_ok(),
            Self::KureIsland => code.parse::<kure_island::PrimarySubdivision>().is_ok(),
            Self::LakshadweepIslands => code.parse::<lakshadweep::PrimarySubdivision>().is_ok(),
            Self::Uruguay => code.parse::<uruguay::PrimarySubdivision>().is_ok(),
            Self::LordHoweIsland => code.parse::<lord_howe_island::PrimarySubdivision>().is_ok(),
            Self::Venezuela => code.parse::<venezuela::PrimarySubdivision>().is_ok(),
            Self::Azores => code.parse::<azores::PrimarySubdivision>().is_ok(),
            Self::Australia => code.parse::<australia::PrimarySubdivision>().is_ok(),
            Self::MalyjVysotskijIsland => code
                .parse::<malyj_vysotskij_island::PrimarySubdivision>()
                .is_ok(),
            Self::MacquarieIsland => code.parse::<macquarie_island::PrimarySubdivision>().is_ok(),
            Self::PapuaNewGuinea => code.parse::<papua_new_guinea::PrimarySubdivision>().is_ok(),
            Self::NewZealand => code.parse::<new_zealand::PrimarySubdivision>().is_ok(),
            Self::MinamiTorishima => code.parse::<minami_torishima::PrimarySubdivision>().is_ok(),
            Self::Ogasawara => code.parse::<ogasawara::PrimarySubdivision>().is_ok(),
            Self::Austria => code.parse::<austria::PrimarySubdivision>().is_ok(),
            Self::Belgium => code.parse::<belgium::PrimarySubdivision>().is_ok(),
            Self::Bulgaria => code.parse::<bulgaria::PrimarySubdivision>().is_ok(),
            Self::Corsica => code.parse::<corsica::PrimarySubdivision>().is_ok(),
            Self::Denmark => code.parse::<denmark::PrimarySubdivision>().is_ok(),
            Self::Finland => code.parse::<finland::PrimarySubdivision>().is_ok(),
            Self::Sardinia => code.parse::<sardinia::PrimarySubdivision>().is_ok(),
            Self::France => code.parse::<france::PrimarySubdivision>().is_ok(),
            Self::FederalRepublicOfGermany => code.parse::<germany::PrimarySubdivision>().is_ok(),
            Self::Hungary => code.parse::<hungary::PrimarySubdivision>().is_ok(),
            Self::Ireland => code.parse::<ireland::PrimarySubdivision>().is_ok(),
            Self::Italy => code.parse::<italy::PrimarySubdivision>().is_ok(),
            Self::MadeiraIslands => code.parse::<madeira::PrimarySubdivision>().is_ok(),
            Self::Svalbard => code.parse::<svalbard::PrimarySubdivision>().is_ok(),
            Self::Netherlands => code.parse::<netherlands::PrimarySubdivision>().is_ok(),
            Self::Norway => code.parse::<norway::PrimarySubdivision>().is_ok(),
            Self::Poland => code.parse::<poland::PrimarySubdivision>().is_ok(),
            Self::Portugal => code.parse::<portugal::PrimarySubdivision>().is_ok(),
            Self::Romania => code.parse::<romania::PrimarySubdivision>().is_ok(),
            Self::Spain => code.parse::<spain::PrimarySubdivision>().is_ok(),
            Self::Sweden => code.parse::<sweden::PrimarySubdivision>().is_ok(),
            Self::Switzerland => code.parse::<switzerland::PrimarySubdivision>().is_ok(),
            Self::Ukraine => code.parse::<ukraine::PrimarySubdivision>().is_ok(),
            Self::UnitedStates => code.parse::<united_states::PrimarySubdivision>().is_ok(),
            Self::China => code.parse::<china::PrimarySubdivision>().is_ok(),
            Self::India => code.parse::<india::PrimarySubdivision>().is_ok(),
            Self::Japan => code.parse::<japan::PrimarySubdivision>().is_ok(),
            Self::Philippines => code.parse::<philippines::PrimarySubdivision>().is_ok(),
            Self::Taiwan => code.parse::<taiwan::PrimarySubdivision>().is_ok(),
            Self::Croatia => code.parse::<croatia::PrimarySubdivision>().is_ok(),
            Self::CzechRepublic => code.parse::<czech_republic::PrimarySubdivision>().is_ok(),
            Self::SlovakRepublic => code.parse::<slovak_republic::PrimarySubdivision>().is_ok(),
            _ => false,
        }
    }
}
