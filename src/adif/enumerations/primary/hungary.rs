//! Hungarian subdivisions, DXCC 239, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Hungarian administrative subdivisions recognized by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Gyõr (Gyõr-Moson-Sopron).
    #[strum(serialize = "GY")]
    Gyor,
    /// Vas.
    #[strum(serialize = "VA")]
    Vas,
    /// Zala.
    #[strum(serialize = "ZA")]
    Zala,
    /// Komárom (Komárom-Esztergom).
    #[strum(serialize = "KO")]
    Komarom,
    /// Veszprém.
    #[strum(serialize = "VE")]
    Veszprem,
    /// Baranya.
    #[strum(serialize = "BA")]
    Baranya,
    /// Somogy.
    #[strum(serialize = "SO")]
    Somogy,
    /// Tolna.
    #[strum(serialize = "TO")]
    Tolna,
    /// Fejér.
    #[strum(serialize = "FE")]
    Fejer,
    /// Budapest.
    #[strum(serialize = "BP")]
    Budapest,
    /// Heves.
    #[strum(serialize = "HE")]
    Heves,
    /// Nógrád.
    #[strum(serialize = "NG")]
    Nograd,
    /// Pest.
    #[strum(serialize = "PE")]
    Pest,
    /// Szolnok (Jász-Nagykun-Szolnok).
    #[strum(serialize = "SZ")]
    Szolnok,
    /// Békés.
    #[strum(serialize = "BE")]
    Bekes,
    /// Bács-Kiskun.
    #[strum(serialize = "BN")]
    BacsKiskun,
    /// Csongrád.
    #[strum(serialize = "CS")]
    Csongrad,
    /// Borsod (Borsod-Abaúj-Zemplén).
    #[strum(serialize = "BO")]
    Borsod,
    /// Hajdú-Bihar.
    #[strum(serialize = "HB")]
    HajduBihar,
    /// Szabolcs (Szabolcs-Szatmár-Bereg).
    #[strum(serialize = "SA")]
    Szabolcs,
}
