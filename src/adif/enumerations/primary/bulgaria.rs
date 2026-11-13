//! Bulgarian subdivisions, DXCC 212, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Bulgarian administrative subdivisions recognized by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Burgas.
    #[strum(serialize = "BU")]
    Burgas,
    /// Sliven.
    #[strum(serialize = "SL")]
    Sliven,
    /// Yambol (Jambol).
    #[strum(serialize = "YA")]
    Yambol,
    /// Sofija Grad.
    #[strum(serialize = "SO")]
    SofijaGrad,
    /// Haskovo.
    #[strum(serialize = "HA")]
    Haskovo,
    /// Kărdžali.
    #[strum(serialize = "KA")]
    Kardzali,
    /// Stara Zagora.
    #[strum(serialize = "SZ")]
    StaraZagora,
    /// Pazardžik.
    #[strum(serialize = "PA")]
    Pazardzik,
    /// Plovdiv.
    #[strum(serialize = "PD")]
    Plovdiv,
    /// Smoljan.
    #[strum(serialize = "SM")]
    Smoljan,
    /// Blagoevgrad.
    #[strum(serialize = "BL")]
    Blagoevgrad,
    /// Kjustendil.
    #[strum(serialize = "KD")]
    Kjustendil,
    /// Pernik.
    #[strum(serialize = "PK")]
    Pernik,
    /// Sofija (Sofia).
    #[strum(serialize = "SF")]
    Sofija,
    /// Gabrovo.
    #[strum(serialize = "GA")]
    Gabrovo,
    /// Loveč (Lovech).
    #[strum(serialize = "LV")]
    Lovec,
    /// Pleven.
    #[strum(serialize = "PL")]
    Pleven,
    /// Veliko Tărnovo.
    #[strum(serialize = "VT")]
    VelikoTarnovo,
    /// Montana.
    #[strum(serialize = "MN")]
    Montana,
    /// Vidin.
    #[strum(serialize = "VD")]
    Vidin,
    /// Vraca.
    #[strum(serialize = "VR")]
    Vraca,
    /// Razgrad.
    #[strum(serialize = "RZ")]
    Razgrad,
    /// Ruse.
    #[strum(serialize = "RS")]
    Ruse,
    /// Silistra.
    #[strum(serialize = "SS")]
    Silistra,
    /// Tărgovište.
    #[strum(serialize = "TA")]
    Targoviste,
    /// Dobrič.
    #[strum(serialize = "DO")]
    Dobric,
    /// Šumen.
    #[strum(serialize = "SN")]
    Sumen,
    /// Varna.
    #[strum(serialize = "VN")]
    Varna,
}
