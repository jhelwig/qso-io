//! Paraguayan primary subdivisions, DXCC entity 132.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Paraguayan primary subdivision, including import-only zero-padded identities.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Alto Paraguay.
    #[strum(serialize = "16")]
    AltoParaguay,
    /// Boquerón.
    #[strum(serialize = "19")]
    Boqueron,
    /// Presidente Hayes.
    #[strum(serialize = "15")]
    PresidenteHayes,
    /// Amambay.
    #[strum(serialize = "13")]
    Amambay,
    /// Concepción; import-only zero-padded identity.
    #[strum(serialize = "01")]
    ConcepcionLegacy,
    /// Concepción.
    #[strum(serialize = "1")]
    Concepcion,
    /// Canindeyú.
    #[strum(serialize = "14")]
    Canindeyu,
    /// San Pedro; import-only zero-padded identity.
    #[strum(serialize = "02")]
    SanPedroLegacy,
    /// San Pedro.
    #[strum(serialize = "2")]
    SanPedro,
    /// Asunción.
    #[strum(serialize = "ASU")]
    Asuncion,
    /// Central.
    #[strum(serialize = "11")]
    Central,
    /// Cordillera; import-only zero-padded identity.
    #[strum(serialize = "03")]
    CordilleraLegacy,
    /// Cordillera.
    #[strum(serialize = "3")]
    Cordillera,
    /// Paraguarí; import-only zero-padded identity.
    #[strum(serialize = "09")]
    ParaguariLegacy,
    /// Paraguarí.
    #[strum(serialize = "9")]
    Paraguari,
    /// Caazapl, as spelled by ADIF; import-only zero-padded identity.
    #[strum(serialize = "06")]
    CaazaplLegacy,
    /// Caazapá.
    #[strum(serialize = "6")]
    Caazapa,
    /// Caeguazú; import-only zero-padded identity.
    #[strum(serialize = "05")]
    CaeguazuLegacy,
    /// Caeguazú.
    #[strum(serialize = "5")]
    Caeguazu,
    /// Guairá; import-only zero-padded identity.
    #[strum(serialize = "04")]
    GuairaLegacy,
    /// Guairá.
    #[strum(serialize = "4")]
    Guaira,
    /// Miaiones, as spelled by ADIF; import-only zero-padded identity.
    #[strum(serialize = "08")]
    MiaionesLegacy,
    /// Misiones.
    #[strum(serialize = "8")]
    Misiones,
    /// Ñeembucú.
    #[strum(serialize = "12")]
    Neembucu,
    /// Alto Paraná.
    #[strum(serialize = "10")]
    AltoParana,
    /// Itapua; import-only zero-padded identity.
    #[strum(serialize = "07")]
    ItapuaLegacy,
    /// Itapúa.
    #[strum(serialize = "7")]
    Itapua,
}
