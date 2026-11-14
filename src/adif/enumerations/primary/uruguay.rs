//! Uruguayan primary subdivisions, DXCC entity 144.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Uruguayan primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Montevideo.
    #[strum(serialize = "MO")]
    Montevideo,
    /// Canelones.
    #[strum(serialize = "CA")]
    Canelones,
    /// San José.
    #[strum(serialize = "SJ")]
    SanJose,
    /// Colonia.
    #[strum(serialize = "CO")]
    Colonia,
    /// Soriano.
    #[strum(serialize = "SO")]
    Soriano,
    /// Río Negro.
    #[strum(serialize = "RN")]
    RioNegro,
    /// Paysandú.
    #[strum(serialize = "PA")]
    Paysandu,
    /// Salto.
    #[strum(serialize = "SA")]
    Salto,
    /// Artigas.
    #[strum(serialize = "AR")]
    Artigas,
    /// Florida.
    #[strum(serialize = "FD")]
    Florida,
    /// Flores.
    #[strum(serialize = "FS")]
    Flores,
    /// Durazno.
    #[strum(serialize = "DU")]
    Durazno,
    /// Tacuarembó.
    #[strum(serialize = "TA")]
    Tacuarembo,
    /// Rivera.
    #[strum(serialize = "RV")]
    Rivera,
    /// Maldonado.
    #[strum(serialize = "MA")]
    Maldonado,
    /// Lavalleja.
    #[strum(serialize = "LA")]
    Lavalleja,
    /// Rocha.
    #[strum(serialize = "RO")]
    Rocha,
    /// Treinta y Tres.
    #[strum(serialize = "TT")]
    TreintaYTres,
    /// Cerro Largo.
    #[strum(serialize = "CL")]
    CerroLargo,
}
