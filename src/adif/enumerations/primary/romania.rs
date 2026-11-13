//! Romanian subdivisions, DXCC 275, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Romanian administrative subdivisions, including the import-only Bucharest code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Arad.
    #[strum(serialize = "AR")]
    Arad,
    /// Caraș-Severin.
    #[strum(serialize = "CS")]
    CarasSeverin,
    /// Hunedoara.
    #[strum(serialize = "HD")]
    Hunedoara,
    /// Timiș.
    #[strum(serialize = "TM")]
    Timis,
    /// Bucureşti (Bucure'ti); import-only legacy code BU.
    #[strum(serialize = "BU")]
    BucurestiLegacy,
    /// București.
    #[strum(serialize = "B")]
    Bucuresti,
    /// Ilfov.
    #[strum(serialize = "IF")]
    Ilfov,
    /// Brăila.
    #[strum(serialize = "BR")]
    Braila,
    /// Constanța.
    #[strum(serialize = "CT")]
    Constanta,
    /// Galați.
    #[strum(serialize = "GL")]
    Galati,
    /// Tulcea.
    #[strum(serialize = "TL")]
    Tulcea,
    /// Vrancea.
    #[strum(serialize = "VN")]
    Vrancea,
    /// Alba.
    #[strum(serialize = "AB")]
    Alba,
    /// Bihor.
    #[strum(serialize = "BH")]
    Bihor,
    /// Bistrița-Năsăud.
    #[strum(serialize = "BN")]
    BistritaNasaud,
    /// Cluj.
    #[strum(serialize = "CJ")]
    Cluj,
    /// Maramureș.
    #[strum(serialize = "MM")]
    Maramures,
    /// Sălaj.
    #[strum(serialize = "SJ")]
    Salaj,
    /// Satu Mare.
    #[strum(serialize = "SM")]
    SatuMare,
    /// Brașov.
    #[strum(serialize = "BV")]
    Brasov,
    /// Covasna.
    #[strum(serialize = "CV")]
    Covasna,
    /// Harghita.
    #[strum(serialize = "HR")]
    Harghita,
    /// Mureș.
    #[strum(serialize = "MS")]
    Mures,
    /// Sibiu.
    #[strum(serialize = "SB")]
    Sibiu,
    /// Argeș.
    #[strum(serialize = "AG")]
    Arges,
    /// Dolj.
    #[strum(serialize = "DJ")]
    Dolj,
    /// Gorj.
    #[strum(serialize = "GJ")]
    Gorj,
    /// Mehedinți.
    #[strum(serialize = "MH")]
    Mehedinti,
    /// Olt.
    #[strum(serialize = "OT")]
    Olt,
    /// Vâlcea.
    #[strum(serialize = "VL")]
    Valcea,
    /// Bacău.
    #[strum(serialize = "BC")]
    Bacau,
    /// Botoșani.
    #[strum(serialize = "BT")]
    Botosani,
    /// Iași.
    #[strum(serialize = "IS")]
    Iasi,
    /// Neamț.
    #[strum(serialize = "NT")]
    Neamt,
    /// Suceava.
    #[strum(serialize = "SV")]
    Suceava,
    /// Vaslui.
    #[strum(serialize = "VS")]
    Vaslui,
    /// Buzău.
    #[strum(serialize = "BZ")]
    Buzau,
    /// Călărași.
    #[strum(serialize = "CL")]
    Calarasi,
    /// Dâmbovița.
    #[strum(serialize = "DB")]
    Dambovita,
    /// Giurgiu.
    #[strum(serialize = "GR")]
    Giurgiu,
    /// Ialomița.
    #[strum(serialize = "IL")]
    Ialomita,
    /// Prahova.
    #[strum(serialize = "PH")]
    Prahova,
    /// Teleorman.
    #[strum(serialize = "TR")]
    Teleorman,
}
