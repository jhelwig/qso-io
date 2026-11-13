//! Swiss subdivisions, DXCC 287, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Swiss cantons recognized by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Aargau.
    #[strum(serialize = "AG")]
    Aargau,
    /// Appenzell Ausserrhoden.
    #[strum(serialize = "AR")]
    AppenzellAusserrhoden,
    /// Appenzell Innerrhoden.
    #[strum(serialize = "AI")]
    AppenzellInnerrhoden,
    /// Basel Landschaft.
    #[strum(serialize = "BL")]
    BaselLandschaft,
    /// Basel Stadt.
    #[strum(serialize = "BS")]
    BaselStadt,
    /// Bern.
    #[strum(serialize = "BE")]
    Bern,
    /// Freiburg / Fribourg.
    #[strum(serialize = "FR")]
    Freiburg,
    /// Genf / Genève.
    #[strum(serialize = "GE")]
    Genf,
    /// Glarus.
    #[strum(serialize = "GL")]
    Glarus,
    /// Graubuenden / Grisons.
    #[strum(serialize = "GR")]
    Graubuenden,
    /// Jura.
    #[strum(serialize = "JU")]
    Jura,
    /// Luzern.
    #[strum(serialize = "LU")]
    Luzern,
    /// Neuenburg / Neuchâtel.
    #[strum(serialize = "NE")]
    Neuenburg,
    /// Nidwalden.
    #[strum(serialize = "NW")]
    Nidwalden,
    /// Obwalden.
    #[strum(serialize = "OW")]
    Obwalden,
    /// Schaffhausen.
    #[strum(serialize = "SH")]
    Schaffhausen,
    /// Schwyz.
    #[strum(serialize = "SZ")]
    Schwyz,
    /// Solothurn.
    #[strum(serialize = "SO")]
    Solothurn,
    /// St. Gallen.
    #[strum(serialize = "SG")]
    StGallen,
    /// Tessin / Ticino.
    #[strum(serialize = "TI")]
    Tessin,
    /// Thurgau.
    #[strum(serialize = "TG")]
    Thurgau,
    /// Uri.
    #[strum(serialize = "UR")]
    Uri,
    /// Waadt / Vaud.
    #[strum(serialize = "VD")]
    Waadt,
    /// Wallis / Valais.
    #[strum(serialize = "VS")]
    Wallis,
    /// Zuerich.
    #[strum(serialize = "ZH")]
    Zuerich,
    /// Zug.
    #[strum(serialize = "ZG")]
    Zug,
}
