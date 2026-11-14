//! Åland Islands municipalities, DXCC entity 5.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// An Åland Islands primary subdivision supplied by ADIF, including deleted Märket.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Brändö.
    #[strum(serialize = "001")]
    Brando,
    /// Eckerö.
    #[strum(serialize = "002")]
    Eckero,
    /// Finström.
    #[strum(serialize = "003")]
    Finstrom,
    /// Föglö.
    #[strum(serialize = "004")]
    Foglo,
    /// Geta.
    #[strum(serialize = "005")]
    Geta,
    /// Hammarland.
    #[strum(serialize = "006")]
    Hammarland,
    /// Jomala.
    #[strum(serialize = "007")]
    Jomala,
    /// Kumlinge.
    #[strum(serialize = "008")]
    Kumlinge,
    /// Kökar.
    #[strum(serialize = "009")]
    Kokar,
    /// Lemland.
    #[strum(serialize = "010")]
    Lemland,
    /// Lumparland.
    #[strum(serialize = "011")]
    Lumparland,
    /// Maarianhamina.
    #[strum(serialize = "012")]
    Maarianhamina,
    /// Saltvik.
    #[strum(serialize = "013")]
    Saltvik,
    /// Sottunga.
    #[strum(serialize = "014")]
    Sottunga,
    /// Sund.
    #[strum(serialize = "015")]
    Sund,
    /// Vårdö.
    #[strum(serialize = "016")]
    Vardo,
    /// Märket; marked deleted in the ADIF subdivision table.
    #[strum(serialize = "051")]
    Market,
}
