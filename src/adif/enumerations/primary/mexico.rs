//! Mexican primary subdivisions, DXCC entity 50.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Mexican primary subdivision, including ADIF's import-only historical spellings.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Colima.
    #[strum(serialize = "COL")]
    Colima,
    /// Distrito Federal; import-only, replaced by CMX.
    #[strum(serialize = "DF")]
    DistritoFederal,
    /// Ciudad de México.
    #[strum(serialize = "CMX")]
    CiudadDeMexico,
    /// Estado de México; import-only historical spelling.
    #[strum(serialize = "EMX")]
    EstadoDeMexicoLegacy,
    /// México.
    #[strum(serialize = "MEX")]
    Mexico,
    /// Guanajuato; import-only historical spelling GTO.
    #[strum(serialize = "GTO")]
    GuanajuatoLegacy,
    /// Guanajuato.
    #[strum(serialize = "GUA")]
    Guanajuato,
    /// Hidalgo; import-only historical spelling HGO.
    #[strum(serialize = "HGO")]
    HidalgoLegacy,
    /// Hidalgo.
    #[strum(serialize = "HID")]
    Hidalgo,
    /// Jalisco.
    #[strum(serialize = "JAL")]
    Jalisco,
    /// Michoacán de Ocampo.
    #[strum(serialize = "MIC")]
    MichoacanDeOcampo,
    /// Morelos.
    #[strum(serialize = "MOR")]
    Morelos,
    /// Nayarit.
    #[strum(serialize = "NAY")]
    Nayarit,
    /// Puebla.
    #[strum(serialize = "PUE")]
    Puebla,
    /// Querétaro de Arteaga; import-only historical spelling QRO.
    #[strum(serialize = "QRO")]
    QueretaroDeArteaga,
    /// Querétaro.
    #[strum(serialize = "QUE")]
    Queretaro,
    /// Tlaxcala; import-only historical spelling TLX.
    #[strum(serialize = "TLX")]
    TlaxcalaLegacy,
    /// Tlaxcala.
    #[strum(serialize = "TLA")]
    Tlaxcala,
    /// Veracruz de Ignacio de la Llave.
    #[strum(serialize = "VER")]
    Veracruz,
    /// Aguascalientes; import-only historical spelling AGS.
    #[strum(serialize = "AGS")]
    AguascalientesLegacy,
    /// Aguascalientes.
    #[strum(serialize = "AGU")]
    Aguascalientes,
    /// Baja California; import-only historical spelling BC.
    #[strum(serialize = "BC")]
    BajaCaliforniaLegacy,
    /// Baja California.
    #[strum(serialize = "BCN")]
    BajaCalifornia,
    /// Baja California Sur.
    #[strum(serialize = "BCS")]
    BajaCaliforniaSur,
    /// Chihuahua.
    #[strum(serialize = "CHH")]
    Chihuahua,
    /// Coahuila de Zaragoza.
    #[strum(serialize = "COA")]
    CoahuilaDeZaragoza,
    /// Durango; import-only historical spelling DGO.
    #[strum(serialize = "DGO")]
    DurangoLegacy,
    /// Durango.
    #[strum(serialize = "DUR")]
    Durango,
    /// Nuevo Leon; import-only historical spelling NL.
    #[strum(serialize = "NL")]
    NuevoLeonLegacy,
    /// Nuevo León.
    #[strum(serialize = "NLE")]
    NuevoLeon,
    /// San Luis Potosí.
    #[strum(serialize = "SLP")]
    SanLuisPotosi,
    /// Sinaloa.
    #[strum(serialize = "SIN")]
    Sinaloa,
    /// Sonora.
    #[strum(serialize = "SON")]
    Sonora,
    /// Tamaulipas; import-only historical spelling TMS.
    #[strum(serialize = "TMS")]
    TamaulipasLegacy,
    /// Tamaulipas.
    #[strum(serialize = "TAM")]
    Tamaulipas,
    /// Zacatecas.
    #[strum(serialize = "ZAC")]
    Zacatecas,
    /// Campeche.
    #[strum(serialize = "CAM")]
    Campeche,
    /// Chiapas; import-only historical spelling CHS.
    #[strum(serialize = "CHS")]
    ChiapasLegacy,
    /// Chiapas.
    #[strum(serialize = "CHP")]
    Chiapas,
    /// Guerrero.
    #[strum(serialize = "GRO")]
    Guerrero,
    /// Oaxaca.
    #[strum(serialize = "OAX")]
    Oaxaca,
    /// Quintana Roo; import-only historical spelling QTR.
    #[strum(serialize = "QTR")]
    QuintanaRooLegacy,
    /// Quintana Roo.
    #[strum(serialize = "ROO")]
    QuintanaRoo,
    /// Tabasco.
    #[strum(serialize = "TAB")]
    Tabasco,
    /// Yucatán.
    #[strum(serialize = "YUC")]
    Yucatan,
}
