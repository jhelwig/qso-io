//! French subdivisions, DXCC 227, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Metropolitan French departments in ADIF's France entity (Corsica is separate).
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Ain.
    #[strum(serialize = "01")]
    Ain,
    /// Aisne.
    #[strum(serialize = "02")]
    Aisne,
    /// Allier.
    #[strum(serialize = "03")]
    Allier,
    /// Alpes-de-Haute-Provence.
    #[strum(serialize = "04")]
    AlpesDeHauteProvence,
    /// Hautes-Alpes.
    #[strum(serialize = "05")]
    HautesAlpes,
    /// Alpes-Maritimes.
    #[strum(serialize = "06")]
    AlpesMaritimes,
    /// Ardèche.
    #[strum(serialize = "07")]
    Ardeche,
    /// Ardennes.
    #[strum(serialize = "08")]
    Ardennes,
    /// Ariège.
    #[strum(serialize = "09")]
    Ariege,
    /// Aube.
    #[strum(serialize = "10")]
    Aube,
    /// Aude.
    #[strum(serialize = "11")]
    Aude,
    /// Aveyron.
    #[strum(serialize = "12")]
    Aveyron,
    /// Bouches-du-Rhône.
    #[strum(serialize = "13")]
    BouchesDuRhone,
    /// Calvados.
    #[strum(serialize = "14")]
    Calvados,
    /// Cantal.
    #[strum(serialize = "15")]
    Cantal,
    /// Charente.
    #[strum(serialize = "16")]
    Charente,
    /// Charente-Maritime.
    #[strum(serialize = "17")]
    CharenteMaritime,
    /// Cher.
    #[strum(serialize = "18")]
    Cher,
    /// Corrèze.
    #[strum(serialize = "19")]
    Correze,
    /// Côte-d'Or.
    #[strum(serialize = "21")]
    CoteDOr,
    /// Côtes-d'Armor.
    #[strum(serialize = "22")]
    CotesDArmor,
    /// Creuse.
    #[strum(serialize = "23")]
    Creuse,
    /// Dordogne.
    #[strum(serialize = "24")]
    Dordogne,
    /// Doubs.
    #[strum(serialize = "25")]
    Doubs,
    /// Drôme.
    #[strum(serialize = "26")]
    Drome,
    /// Eure.
    #[strum(serialize = "27")]
    Eure,
    /// Eure-et-Loir.
    #[strum(serialize = "28")]
    EureEtLoir,
    /// Finistère.
    #[strum(serialize = "29")]
    Finistere,
    /// Gard.
    #[strum(serialize = "30")]
    Gard,
    /// Haute-Garonne.
    #[strum(serialize = "31")]
    HauteGaronne,
    /// Gers.
    #[strum(serialize = "32")]
    Gers,
    /// Gironde.
    #[strum(serialize = "33")]
    Gironde,
    /// Hérault.
    #[strum(serialize = "34")]
    Herault,
    /// Ille-et-Vilaine.
    #[strum(serialize = "35")]
    IlleEtVilaine,
    /// Indre.
    #[strum(serialize = "36")]
    Indre,
    /// Indre-et-Loire.
    #[strum(serialize = "37")]
    IndreEtLoire,
    /// Isère.
    #[strum(serialize = "38")]
    Isere,
    /// Jura.
    #[strum(serialize = "39")]
    Jura,
    /// Landes.
    #[strum(serialize = "40")]
    Landes,
    /// Loir-et-Cher.
    #[strum(serialize = "41")]
    LoirEtCher,
    /// Loire.
    #[strum(serialize = "42")]
    Loire,
    /// Haute-Loire.
    #[strum(serialize = "43")]
    HauteLoire,
    /// Loire-Atlantique.
    #[strum(serialize = "44")]
    LoireAtlantique,
    /// Loiret.
    #[strum(serialize = "45")]
    Loiret,
    /// Lot.
    #[strum(serialize = "46")]
    Lot,
    /// Lot-et-Garonne.
    #[strum(serialize = "47")]
    LotEtGaronne,
    /// Lozère.
    #[strum(serialize = "48")]
    Lozere,
    /// Maine-et-Loire.
    #[strum(serialize = "49")]
    MaineEtLoire,
    /// Manche.
    #[strum(serialize = "50")]
    Manche,
    /// Marne.
    #[strum(serialize = "51")]
    Marne,
    /// Haute-Marne.
    #[strum(serialize = "52")]
    HauteMarne,
    /// Mayenne.
    #[strum(serialize = "53")]
    Mayenne,
    /// Meurthe-et-Moselle.
    #[strum(serialize = "54")]
    MeurtheEtMoselle,
    /// Meuse.
    #[strum(serialize = "55")]
    Meuse,
    /// Morbihan.
    #[strum(serialize = "56")]
    Morbihan,
    /// Moselle.
    #[strum(serialize = "57")]
    Moselle,
    /// Nièvre.
    #[strum(serialize = "58")]
    Nievre,
    /// Nord.
    #[strum(serialize = "59")]
    Nord,
    /// Oise.
    #[strum(serialize = "60")]
    Oise,
    /// Orne.
    #[strum(serialize = "61")]
    Orne,
    /// Pas-de-Calais.
    #[strum(serialize = "62")]
    PasDeCalais,
    /// Puy-de-Dôme.
    #[strum(serialize = "63")]
    PuyDeDome,
    /// Pyrénées-Atlantiques.
    #[strum(serialize = "64")]
    PyreneesAtlantiques,
    /// Hautes-Pyrénées.
    #[strum(serialize = "65")]
    HautesPyrenees,
    /// Pyrénées-Orientales.
    #[strum(serialize = "66")]
    PyreneesOrientales,
    /// Bas-Rhin.
    #[strum(serialize = "67")]
    BasRhin,
    /// Haut-Rhin.
    #[strum(serialize = "68")]
    HautRhin,
    /// Rhône.
    #[strum(serialize = "69")]
    Rhone,
    /// Haute-Saône.
    #[strum(serialize = "70")]
    HauteSaone,
    /// Saône-et-Loire.
    #[strum(serialize = "71")]
    SaoneEtLoire,
    /// Sarthe.
    #[strum(serialize = "72")]
    Sarthe,
    /// Savoie.
    #[strum(serialize = "73")]
    Savoie,
    /// Haute-Savoie.
    #[strum(serialize = "74")]
    HauteSavoie,
    /// Paris.
    #[strum(serialize = "75")]
    Paris,
    /// Seine-Maritime.
    #[strum(serialize = "76")]
    SeineMaritime,
    /// Seine-et-Marne.
    #[strum(serialize = "77")]
    SeineEtMarne,
    /// Yvelines.
    #[strum(serialize = "78")]
    Yvelines,
    /// Deux-Sèvres.
    #[strum(serialize = "79")]
    DeuxSevres,
    /// Somme.
    #[strum(serialize = "80")]
    Somme,
    /// Tarn.
    #[strum(serialize = "81")]
    Tarn,
    /// Tarn-et-Garonne.
    #[strum(serialize = "82")]
    TarnEtGaronne,
    /// Var.
    #[strum(serialize = "83")]
    Var,
    /// Vaucluse.
    #[strum(serialize = "84")]
    Vaucluse,
    /// Vendée.
    #[strum(serialize = "85")]
    Vendee,
    /// Vienne.
    #[strum(serialize = "86")]
    Vienne,
    /// Haute-Vienne.
    #[strum(serialize = "87")]
    HauteVienne,
    /// Vosges.
    #[strum(serialize = "88")]
    Vosges,
    /// Yonne.
    #[strum(serialize = "89")]
    Yonne,
    /// Territoire de Belfort.
    #[strum(serialize = "90")]
    TerritoireDeBelfort,
    /// Essonne.
    #[strum(serialize = "91")]
    Essonne,
    /// Hauts-de-Seine.
    #[strum(serialize = "92")]
    HautsDeSeine,
    /// Seine-Saint-Denis.
    #[strum(serialize = "93")]
    SeineSaintDenis,
    /// Val-de-Marne.
    #[strum(serialize = "94")]
    ValDeMarne,
    /// Val-d'Oise.
    #[strum(serialize = "95")]
    ValDOise,
}
