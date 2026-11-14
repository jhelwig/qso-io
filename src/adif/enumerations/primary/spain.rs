//! Spanish subdivisions, DXCC 281, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Spanish provinces in ADIF's Spain entity, including import-only OU.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Ávila.
    #[strum(serialize = "AV")]
    Avila,
    /// Burgos.
    #[strum(serialize = "BU")]
    Burgos,
    /// A Coruña.
    #[strum(serialize = "C")]
    ACoruna,
    /// León.
    #[strum(serialize = "LE")]
    Leon,
    /// La Rioja.
    #[strum(serialize = "LO")]
    LaRioja,
    /// Lugo.
    #[strum(serialize = "LU")]
    Lugo,
    /// Asturias.
    #[strum(serialize = "O")]
    Asturias,
    /// Ourense; import-only legacy code OU.
    #[strum(serialize = "OU")]
    OurenseLegacy,
    /// Ourense.
    #[strum(serialize = "OR")]
    Ourense,
    /// Palencia.
    #[strum(serialize = "P")]
    Palencia,
    /// Pontevedra.
    #[strum(serialize = "PO")]
    Pontevedra,
    /// Cantabria.
    #[strum(serialize = "S")]
    Cantabria,
    /// Salamanca.
    #[strum(serialize = "SA")]
    Salamanca,
    /// Segovia.
    #[strum(serialize = "SG")]
    Segovia,
    /// Soria.
    #[strum(serialize = "SO")]
    Soria,
    /// Valladolid.
    #[strum(serialize = "VA")]
    Valladolid,
    /// Zamora.
    #[strum(serialize = "ZA")]
    Zamora,
    /// Bizkaia.
    #[strum(serialize = "BI")]
    Bizkaia,
    /// Huesca.
    #[strum(serialize = "HU")]
    Huesca,
    /// Navarra.
    #[strum(serialize = "NA")]
    Navarra,
    /// Gipuzkoa.
    #[strum(serialize = "SS")]
    Gipuzkoa,
    /// Teruel.
    #[strum(serialize = "TE")]
    Teruel,
    /// Álava.
    #[strum(serialize = "VI")]
    Alava,
    /// Zaragoza.
    #[strum(serialize = "Z")]
    Zaragoza,
    /// Barcelona.
    #[strum(serialize = "B")]
    Barcelona,
    /// Girona.
    #[strum(serialize = "GI")]
    Girona,
    /// Lleida.
    #[strum(serialize = "L")]
    Lleida,
    /// Tarragona.
    #[strum(serialize = "T")]
    Tarragona,
    /// Badajoz.
    #[strum(serialize = "BA")]
    Badajoz,
    /// Cáceres.
    #[strum(serialize = "CC")]
    Caceres,
    /// Ciudad Real.
    #[strum(serialize = "CR")]
    CiudadReal,
    /// Cuenca.
    #[strum(serialize = "CU")]
    Cuenca,
    /// Guadalajara.
    #[strum(serialize = "GU")]
    Guadalajara,
    /// Madrid.
    #[strum(serialize = "M")]
    Madrid,
    /// Toledo.
    #[strum(serialize = "TO")]
    Toledo,
    /// Alicante.
    #[strum(serialize = "A")]
    Alicante,
    /// Albacete.
    #[strum(serialize = "AB")]
    Albacete,
    /// Castellón.
    #[strum(serialize = "CS")]
    Castellon,
    /// Murcia.
    #[strum(serialize = "MU")]
    Murcia,
    /// València.
    #[strum(serialize = "V")]
    Valencia,
    /// Almeria.
    #[strum(serialize = "AL")]
    Almeria,
    /// Cádiz.
    #[strum(serialize = "CA")]
    Cadiz,
    /// Córdoba.
    #[strum(serialize = "CO")]
    Cordoba,
    /// Granada.
    #[strum(serialize = "GR")]
    Granada,
    /// Huelva.
    #[strum(serialize = "H")]
    Huelva,
    /// Jaén.
    #[strum(serialize = "J")]
    Jaen,
    /// Málaga.
    #[strum(serialize = "MA")]
    Malaga,
    /// Sevilla.
    #[strum(serialize = "SE")]
    Sevilla,
}
