//! Swedish subdivisions, DXCC 284, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Swedish counties recognized by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Stockholms län.
    #[strum(serialize = "AB")]
    Stockholms,
    /// Gotlands län.
    #[strum(serialize = "I")]
    Gotlands,
    /// Norrbottens län.
    #[strum(serialize = "BD")]
    Norrbottens,
    /// Västerbottens län.
    #[strum(serialize = "AC")]
    Vasterbottens,
    /// Gävleborgs län.
    #[strum(serialize = "X")]
    Gavleborgs,
    /// Jämtlands län.
    #[strum(serialize = "Z")]
    Jamtlands,
    /// Västernorrlands län.
    #[strum(serialize = "Y")]
    Vasternorrlands,
    /// Dalarnas län.
    #[strum(serialize = "W")]
    Dalarnas,
    /// Värmlands län.
    #[strum(serialize = "S")]
    Varmlands,
    /// Västra Götalands län.
    #[strum(serialize = "O")]
    VastraGotalands,
    /// Örebro län.
    #[strum(serialize = "T")]
    Orebro,
    /// Östergötlands län.
    #[strum(serialize = "E")]
    Ostergotlands,
    /// Södermanlands län.
    #[strum(serialize = "D")]
    Sodermanlands,
    /// Uppsala län.
    #[strum(serialize = "C")]
    Uppsala,
    /// Västmanlands län.
    #[strum(serialize = "U")]
    Vastmanlands,
    /// Hallands län.
    #[strum(serialize = "N")]
    Hallands,
    /// Blekinge län.
    #[strum(serialize = "K")]
    Blekinge,
    /// Jönköpings län.
    #[strum(serialize = "F")]
    Jonkopings,
    /// Kalmar län.
    #[strum(serialize = "H")]
    Kalmar,
    /// Kronobergs län.
    #[strum(serialize = "G")]
    Kronobergs,
    /// Skåne län.
    #[strum(serialize = "M")]
    Skane,
}
