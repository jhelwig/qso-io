//! Chilean primary subdivisions, DXCC entity 112.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Chilean primary subdivision, including import-only Roman numeral spellings.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Antofagasta; import-only Roman numeral spelling.
    #[strum(serialize = "II")]
    AntofagastaLegacy,
    /// Antofagasta.
    #[strum(serialize = "AN")]
    Antofagasta,
    /// Atacama; import-only Roman numeral spelling.
    #[strum(serialize = "III")]
    AtacamaLegacy,
    /// Atacama.
    #[strum(serialize = "AT")]
    Atacama,
    /// Tarapacá; import-only Roman numeral spelling.
    #[strum(serialize = "I")]
    TarapacaLegacy,
    /// Tarapacá.
    #[strum(serialize = "TA")]
    Tarapaca,
    /// Arica y Parinacota; import-only Roman numeral spelling.
    #[strum(serialize = "XV")]
    AricaYParinacotaLegacy,
    /// Arica y Parinacota.
    #[strum(serialize = "AP")]
    AricaYParinacota,
    /// Coquimbo; import-only Roman numeral spelling.
    #[strum(serialize = "IV")]
    CoquimboLegacy,
    /// Coquimbo.
    #[strum(serialize = "CO")]
    Coquimbo,
    /// Valparaíso; import-only Roman numeral spelling.
    #[strum(serialize = "V")]
    ValparaisoLegacy,
    /// Valparaíso.
    #[strum(serialize = "VS")]
    Valparaiso,
    /// Region Metropolitana de Santiago.
    #[strum(serialize = "RM")]
    MetropolitanaDeSantiago,
    /// Libertador General Bernardo O'Higgins; import-only Roman numeral spelling.
    #[strum(serialize = "VI")]
    OHigginsLegacy,
    /// Libertador General Bernardo O'Higgins.
    #[strum(serialize = "LI")]
    OHiggins,
    /// Maule; import-only Roman numeral spelling.
    #[strum(serialize = "VII")]
    MauleLegacy,
    /// Maule.
    #[strum(serialize = "ML")]
    Maule,
    /// Bío-Bío; import-only Roman numeral spelling.
    #[strum(serialize = "VIII")]
    BioBioLegacy,
    /// Biobío.
    #[strum(serialize = "BI")]
    Biobio,
    /// La Araucanía; import-only Roman numeral spelling.
    #[strum(serialize = "IX")]
    LaAraucaniaLegacy,
    /// La Araucanía.
    #[strum(serialize = "AR")]
    LaAraucania,
    /// Los Ríos; import-only Roman numeral spelling.
    #[strum(serialize = "XIV")]
    LosRiosLegacy,
    /// Los Ríos.
    #[strum(serialize = "LR")]
    LosRios,
    /// Los Lagos; import-only Roman numeral spelling.
    #[strum(serialize = "X")]
    LosLagosLegacy,
    /// Los Lagos.
    #[strum(serialize = "LL")]
    LosLagos,
    /// Aisén del General Carlos Ibáñez del Campo; import-only Roman numeral spelling.
    #[strum(serialize = "XI")]
    AisenLegacy,
    /// Aisén del General Carlos Ibañez del Campo.
    #[strum(serialize = "AI")]
    Aisen,
    /// Magallanes; import-only Roman numeral spelling.
    #[strum(serialize = "XII")]
    MagallanesLegacy,
    /// Magallanes.
    #[strum(serialize = "MA")]
    Magallanes,
    /// Ñuble.
    #[strum(serialize = "NB")]
    Nuble,
}
