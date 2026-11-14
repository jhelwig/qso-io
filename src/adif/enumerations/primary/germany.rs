//! German subdivisions, DXCC 230, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// German states recognized by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Brandenburg.
    #[strum(serialize = "BB")]
    Brandenburg,
    /// Berlin.
    #[strum(serialize = "BE")]
    Berlin,
    /// Baden-Württemberg.
    #[strum(serialize = "BW")]
    BadenWurttemberg,
    /// Bayern.
    #[strum(serialize = "BY")]
    Bayern,
    /// Bremen.
    #[strum(serialize = "HB")]
    Bremen,
    /// Hessen.
    #[strum(serialize = "HE")]
    Hessen,
    /// Hamburg.
    #[strum(serialize = "HH")]
    Hamburg,
    /// Mecklenburg-Vorpommern.
    #[strum(serialize = "MV")]
    MecklenburgVorpommern,
    /// Niedersachsen.
    #[strum(serialize = "NI")]
    Niedersachsen,
    /// Nordrhein-Westfalen.
    #[strum(serialize = "NW")]
    NordrheinWestfalen,
    /// Rheinland-Pfalz.
    #[strum(serialize = "RP")]
    RheinlandPfalz,
    /// Saarland.
    #[strum(serialize = "SL")]
    Saarland,
    /// Schleswig-Holstein.
    #[strum(serialize = "SH")]
    SchleswigHolstein,
    /// Sachsen.
    #[strum(serialize = "SN")]
    Sachsen,
    /// Sachsen-Anhalt.
    #[strum(serialize = "ST")]
    SachsenAnhalt,
    /// Thüringen.
    #[strum(serialize = "TH")]
    Thuringen,
}
