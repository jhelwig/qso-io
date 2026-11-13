//! Italian subdivisions, DXCC 248, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Italian provinces, including import-only codes; Sardinia is a separate entity.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Genova.
    #[strum(serialize = "GE")]
    Genova,
    /// Imperia.
    #[strum(serialize = "IM")]
    Imperia,
    /// La Spezia.
    #[strum(serialize = "SP")]
    LaSpezia,
    /// Savona.
    #[strum(serialize = "SV")]
    Savona,
    /// Alessandria.
    #[strum(serialize = "AL")]
    Alessandria,
    /// Asti.
    #[strum(serialize = "AT")]
    Asti,
    /// Biella.
    #[strum(serialize = "BI")]
    Biella,
    /// Cuneo.
    #[strum(serialize = "CN")]
    Cuneo,
    /// Novara.
    #[strum(serialize = "NO")]
    Novara,
    /// Torino.
    #[strum(serialize = "TO")]
    Torino,
    /// Verbano Cusio Ossola.
    #[strum(serialize = "VB")]
    VerbanoCusioOssola,
    /// Vercelli.
    #[strum(serialize = "VC")]
    Vercelli,
    /// Aosta.
    #[strum(serialize = "AO")]
    Aosta,
    /// Bergamo.
    #[strum(serialize = "BG")]
    Bergamo,
    /// Brescia.
    #[strum(serialize = "BS")]
    Brescia,
    /// Como.
    #[strum(serialize = "CO")]
    Como,
    /// Cremona.
    #[strum(serialize = "CR")]
    Cremona,
    /// Lecco.
    #[strum(serialize = "LC")]
    Lecco,
    /// Lodi.
    #[strum(serialize = "LO")]
    Lodi,
    /// Monza e Brianza.
    #[strum(serialize = "MB")]
    MonzaEBrianza,
    /// Mantova.
    #[strum(serialize = "MN")]
    Mantova,
    /// Milano.
    #[strum(serialize = "MI")]
    Milano,
    /// Pavia.
    #[strum(serialize = "PV")]
    Pavia,
    /// Sondrio.
    #[strum(serialize = "SO")]
    Sondrio,
    /// Varese.
    #[strum(serialize = "VA")]
    Varese,
    /// Belluno.
    #[strum(serialize = "BL")]
    Belluno,
    /// Padova.
    #[strum(serialize = "PD")]
    Padova,
    /// Rovigo.
    #[strum(serialize = "RO")]
    Rovigo,
    /// Treviso.
    #[strum(serialize = "TV")]
    Treviso,
    /// Venezia.
    #[strum(serialize = "VE")]
    Venezia,
    /// Verona.
    #[strum(serialize = "VR")]
    Verona,
    /// Vicenza.
    #[strum(serialize = "VI")]
    Vicenza,
    /// Bolzano.
    #[strum(serialize = "BZ")]
    Bolzano,
    /// Trento.
    #[strum(serialize = "TN")]
    Trento,
    /// Gorizia.
    #[strum(serialize = "GO")]
    Gorizia,
    /// Pordenone.
    #[strum(serialize = "PN")]
    Pordenone,
    /// Trieste.
    #[strum(serialize = "TS")]
    Trieste,
    /// Udine.
    #[strum(serialize = "UD")]
    Udine,
    /// Bologna.
    #[strum(serialize = "BO")]
    Bologna,
    /// Ferrara.
    #[strum(serialize = "FE")]
    Ferrara,
    /// Forlì; import-only legacy code FO.
    #[strum(serialize = "FO")]
    ForliLegacy,
    /// Forlì-Cesena.
    #[strum(serialize = "FC")]
    ForliCesena,
    /// Modena.
    #[strum(serialize = "MO")]
    Modena,
    /// Parma.
    #[strum(serialize = "PR")]
    Parma,
    /// Piacenza.
    #[strum(serialize = "PC")]
    Piacenza,
    /// Ravenna.
    #[strum(serialize = "RA")]
    Ravenna,
    /// Reggio Emilia.
    #[strum(serialize = "RE")]
    ReggioEmilia,
    /// Rimini.
    #[strum(serialize = "RN")]
    Rimini,
    /// Arezzo.
    #[strum(serialize = "AR")]
    Arezzo,
    /// Firenze.
    #[strum(serialize = "FI")]
    Firenze,
    /// Grosseto.
    #[strum(serialize = "GR")]
    Grosseto,
    /// Livorno.
    #[strum(serialize = "LI")]
    Livorno,
    /// Lucca.
    #[strum(serialize = "LU")]
    Lucca,
    /// Massa Carrara.
    #[strum(serialize = "MS")]
    MassaCarrara,
    /// Pistoia.
    #[strum(serialize = "PT")]
    Pistoia,
    /// Pisa.
    #[strum(serialize = "PI")]
    Pisa,
    /// Prato.
    #[strum(serialize = "PO")]
    Prato,
    /// Siena.
    #[strum(serialize = "SI")]
    Siena,
    /// Chieti.
    #[strum(serialize = "CH")]
    Chieti,
    /// L'Aquila.
    #[strum(serialize = "AQ")]
    LAquila,
    /// Pescara.
    #[strum(serialize = "PE")]
    Pescara,
    /// Teramo.
    #[strum(serialize = "TE")]
    Teramo,
    /// Ancona.
    #[strum(serialize = "AN")]
    Ancona,
    /// Ascoli Piceno.
    #[strum(serialize = "AP")]
    AscoliPiceno,
    /// Fermo.
    #[strum(serialize = "FM")]
    Fermo,
    /// Macerata.
    #[strum(serialize = "MC")]
    Macerata,
    /// Pesaro e Urbino; import-only legacy code PS.
    #[strum(serialize = "PS")]
    PesaroEUrbinoLegacy,
    /// Pesaro e Urbino.
    #[strum(serialize = "PU")]
    PesaroEUrbino,
    /// Matera.
    #[strum(serialize = "MT")]
    Matera,
    /// Potenza.
    #[strum(serialize = "PZ")]
    Potenza,
    /// Bari.
    #[strum(serialize = "BA")]
    Bari,
    /// Barletta-Andria-Trani.
    #[strum(serialize = "BT")]
    BarlettaAndriaTrani,
    /// Brindisi.
    #[strum(serialize = "BR")]
    Brindisi,
    /// Foggia.
    #[strum(serialize = "FG")]
    Foggia,
    /// Lecce.
    #[strum(serialize = "LE")]
    Lecce,
    /// Taranto.
    #[strum(serialize = "TA")]
    Taranto,
    /// Catanzaro.
    #[strum(serialize = "CZ")]
    Catanzaro,
    /// Cosenza.
    #[strum(serialize = "CS")]
    Cosenza,
    /// Crotone.
    #[strum(serialize = "KR")]
    Crotone,
    /// Reggio Calabria.
    #[strum(serialize = "RC")]
    ReggioCalabria,
    /// Vibo Valentia.
    #[strum(serialize = "VV")]
    ViboValentia,
    /// Avellino.
    #[strum(serialize = "AV")]
    Avellino,
    /// Benevento.
    #[strum(serialize = "BN")]
    Benevento,
    /// Caserta.
    #[strum(serialize = "CE")]
    Caserta,
    /// Napoli.
    #[strum(serialize = "NA")]
    Napoli,
    /// Salerno.
    #[strum(serialize = "SA")]
    Salerno,
    /// Isernia.
    #[strum(serialize = "IS")]
    Isernia,
    /// Campobasso.
    #[strum(serialize = "CB")]
    Campobasso,
    /// Frosinone.
    #[strum(serialize = "FR")]
    Frosinone,
    /// Latina.
    #[strum(serialize = "LT")]
    Latina,
    /// Rieti.
    #[strum(serialize = "RI")]
    Rieti,
    /// Roma.
    #[strum(serialize = "RM")]
    Roma,
    /// Viterbo.
    #[strum(serialize = "VT")]
    Viterbo,
    /// Perugia.
    #[strum(serialize = "PG")]
    Perugia,
    /// Terni.
    #[strum(serialize = "TR")]
    Terni,
    /// Agrigento.
    #[strum(serialize = "AG")]
    Agrigento,
    /// Caltanissetta.
    #[strum(serialize = "CL")]
    Caltanissetta,
    /// Catania.
    #[strum(serialize = "CT")]
    Catania,
    /// Enna.
    #[strum(serialize = "EN")]
    Enna,
    /// Messina.
    #[strum(serialize = "ME")]
    Messina,
    /// Palermo.
    #[strum(serialize = "PA")]
    Palermo,
    /// Ragusa.
    #[strum(serialize = "RG")]
    Ragusa,
    /// Siracusa.
    #[strum(serialize = "SR")]
    Siracusa,
    /// Trapani.
    #[strum(serialize = "TP")]
    Trapani,
}
