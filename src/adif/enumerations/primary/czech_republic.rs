//! Czech subdivisions, DXCC 503, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Czech districts and Prague districts recognized by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Praha 1.
    #[strum(serialize = "APA")]
    Praha1,
    /// Praha 2.
    #[strum(serialize = "APB")]
    Praha2,
    /// Praha 3.
    #[strum(serialize = "APC")]
    Praha3,
    /// Praha 4.
    #[strum(serialize = "APD")]
    Praha4,
    /// Praha 5.
    #[strum(serialize = "APE")]
    Praha5,
    /// Praha 6.
    #[strum(serialize = "APF")]
    Praha6,
    /// Praha 7.
    #[strum(serialize = "APG")]
    Praha7,
    /// Praha 8.
    #[strum(serialize = "APH")]
    Praha8,
    /// Praha 9.
    #[strum(serialize = "API")]
    Praha9,
    /// Praha 10.
    #[strum(serialize = "APJ")]
    Praha10,
    /// Benesov.
    #[strum(serialize = "BBN")]
    Benesov,
    /// Beroun.
    #[strum(serialize = "BBE")]
    Beroun,
    /// Kladno.
    #[strum(serialize = "BKD")]
    Kladno,
    /// Kolin.
    #[strum(serialize = "BKO")]
    Kolin,
    /// Kutna Hora.
    #[strum(serialize = "BKH")]
    KutnaHora,
    /// Melnik.
    #[strum(serialize = "BME")]
    Melnik,
    /// Mlada Boleslav.
    #[strum(serialize = "BMB")]
    MladaBoleslav,
    /// Nymburk.
    #[strum(serialize = "BNY")]
    Nymburk,
    /// Praha zapad.
    #[strum(serialize = "BPZ")]
    PrahaZapad,
    /// Praha vychod.
    #[strum(serialize = "BPV")]
    PrahaVychod,
    /// Pribram.
    #[strum(serialize = "BPB")]
    Pribram,
    /// Rakovnik.
    #[strum(serialize = "BRA")]
    Rakovnik,
    /// Ceske Budejovice.
    #[strum(serialize = "CBU")]
    CeskeBudejovice,
    /// Cesky Krumlov.
    #[strum(serialize = "CCK")]
    CeskyKrumlov,
    /// Jindrichuv Hradec.
    #[strum(serialize = "CJH")]
    JindrichuvHradec,
    /// Pelhrimov.
    #[strum(serialize = "CPE")]
    Pelhrimov,
    /// Pisek.
    #[strum(serialize = "CPI")]
    Pisek,
    /// Prachatice.
    #[strum(serialize = "CPR")]
    Prachatice,
    /// Strakonice.
    #[strum(serialize = "CST")]
    Strakonice,
    /// Tabor.
    #[strum(serialize = "CTA")]
    Tabor,
    /// Domazlice.
    #[strum(serialize = "DDO")]
    Domazlice,
    /// Cheb.
    #[strum(serialize = "DCH")]
    Cheb,
    /// Karlovy Vary.
    #[strum(serialize = "DKV")]
    KarlovyVary,
    /// Klatovy.
    #[strum(serialize = "DKL")]
    Klatovy,
    /// Plzen mesto.
    #[strum(serialize = "DPM")]
    PlzenMesto,
    /// Plzen jih.
    #[strum(serialize = "DPJ")]
    PlzenJih,
    /// Plzen sever.
    #[strum(serialize = "DPS")]
    PlzenSever,
    /// Rokycany.
    #[strum(serialize = "DRO")]
    Rokycany,
    /// Sokolov.
    #[strum(serialize = "DSO")]
    Sokolov,
    /// Tachov.
    #[strum(serialize = "DTA")]
    Tachov,
    /// Ceska Lipa.
    #[strum(serialize = "ECL")]
    CeskaLipa,
    /// Decin.
    #[strum(serialize = "EDE")]
    Decin,
    /// Chomutov.
    #[strum(serialize = "ECH")]
    Chomutov,
    /// Jablonec n. Nisou.
    #[strum(serialize = "EJA")]
    JablonecNadNisou,
    /// Liberec.
    #[strum(serialize = "ELI")]
    Liberec,
    /// Litomerice.
    #[strum(serialize = "ELT")]
    Litomerice,
    /// Louny.
    #[strum(serialize = "ELO")]
    Louny,
    /// Most.
    #[strum(serialize = "EMO")]
    Most,
    /// Teplice.
    #[strum(serialize = "ETE")]
    Teplice,
    /// Usti nad Labem.
    #[strum(serialize = "EUL")]
    UstiNadLabem,
    /// Havlickuv Brod.
    #[strum(serialize = "FHB")]
    HavlickuvBrod,
    /// Hradec Kralove.
    #[strum(serialize = "FHK")]
    HradecKralove,
    /// Chrudim.
    #[strum(serialize = "FCR")]
    Chrudim,
    /// Jicin.
    #[strum(serialize = "FJI")]
    Jicin,
    /// Nachod.
    #[strum(serialize = "FNA")]
    Nachod,
    /// Pardubice.
    #[strum(serialize = "FPA")]
    Pardubice,
    /// Rychn n. Kneznou.
    #[strum(serialize = "FRK")]
    RychnNadKneznou,
    /// Semily.
    #[strum(serialize = "FSE")]
    Semily,
    /// Svitavy.
    #[strum(serialize = "FSV")]
    Svitavy,
    /// Trutnov.
    #[strum(serialize = "FTR")]
    Trutnov,
    /// Usti nad Orlici.
    #[strum(serialize = "FUO")]
    UstiNadOrlici,
    /// Blansko.
    #[strum(serialize = "GBL")]
    Blansko,
    /// Brno mesto.
    #[strum(serialize = "GBM")]
    BrnoMesto,
    /// Brno venkov.
    #[strum(serialize = "GBV")]
    BrnoVenkov,
    /// Breclav.
    #[strum(serialize = "GBR")]
    Breclav,
    /// Hodonin.
    #[strum(serialize = "GHO")]
    Hodonin,
    /// Jihlava.
    #[strum(serialize = "GJI")]
    Jihlava,
    /// Kromeriz.
    #[strum(serialize = "GKR")]
    Kromeriz,
    /// Prostejov.
    #[strum(serialize = "GPR")]
    Prostejov,
    /// Trebic.
    #[strum(serialize = "GTR")]
    Trebic,
    /// Uherske Hradiste.
    #[strum(serialize = "GUH")]
    UherskeHradiste,
    /// Vyskov.
    #[strum(serialize = "GVY")]
    Vyskov,
    /// Zlin.
    #[strum(serialize = "GZL")]
    Zlin,
    /// Znojmo.
    #[strum(serialize = "GZN")]
    Znojmo,
    /// Zdar nad Sazavou.
    #[strum(serialize = "GZS")]
    ZdarNadSazavou,
    /// Bruntal.
    #[strum(serialize = "HBR")]
    Bruntal,
    /// Frydek-Mistek.
    #[strum(serialize = "HFM")]
    FrydekMistek,
    /// Jesenik.
    #[strum(serialize = "HJE")]
    Jesenik,
    /// Karvina.
    #[strum(serialize = "HKA")]
    Karvina,
    /// Novy Jicin.
    #[strum(serialize = "HNJ")]
    NovyJicin,
    /// Olomouc.
    #[strum(serialize = "HOL")]
    Olomouc,
    /// Opava.
    #[strum(serialize = "HOP")]
    Opava,
    /// Ostrava.
    #[strum(serialize = "HOS")]
    Ostrava,
    /// Prerov.
    #[strum(serialize = "HPR")]
    Prerov,
    /// Sumperk.
    #[strum(serialize = "HSU")]
    Sumperk,
    /// Vsetin.
    #[strum(serialize = "HVS")]
    Vsetin,
}
