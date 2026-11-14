//! Venezuelan primary subdivisions, DXCC entity 148.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Venezuelan primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Amazonas.
    #[strum(serialize = "AM")]
    Amazonas,
    /// Anzoátegui.
    #[strum(serialize = "AN")]
    Anzoategui,
    /// Apure.
    #[strum(serialize = "AP")]
    Apure,
    /// Aragua.
    #[strum(serialize = "AR")]
    Aragua,
    /// Barinas.
    #[strum(serialize = "BA")]
    Barinas,
    /// Bolívar.
    #[strum(serialize = "BO")]
    Bolivar,
    /// Carabobo.
    #[strum(serialize = "CA")]
    Carabobo,
    /// Cojedes.
    #[strum(serialize = "CO")]
    Cojedes,
    /// Delta Amacuro.
    #[strum(serialize = "DA")]
    DeltaAmacuro,
    /// Distrito Capital.
    #[strum(serialize = "DC")]
    DistritoCapital,
    /// Falcón.
    #[strum(serialize = "FA")]
    Falcon,
    /// Guárico.
    #[strum(serialize = "GU")]
    Guarico,
    /// Lara.
    #[strum(serialize = "LA")]
    Lara,
    /// Mérida.
    #[strum(serialize = "ME")]
    Merida,
    /// Miranda.
    #[strum(serialize = "MI")]
    Miranda,
    /// Monagas.
    #[strum(serialize = "MO")]
    Monagas,
    /// Nueva Esparta.
    #[strum(serialize = "NE")]
    NuevaEsparta,
    /// Portuguesa.
    #[strum(serialize = "PO")]
    Portuguesa,
    /// Sucre.
    #[strum(serialize = "SU")]
    Sucre,
    /// Táchira.
    #[strum(serialize = "TA")]
    Tachira,
    /// Trujillo.
    #[strum(serialize = "TR")]
    Trujillo,
    /// Vargas.
    #[strum(serialize = "VA")]
    Vargas,
    /// Yaracuy.
    #[strum(serialize = "YA")]
    Yaracuy,
    /// Zulia.
    #[strum(serialize = "ZU")]
    Zulia,
}
