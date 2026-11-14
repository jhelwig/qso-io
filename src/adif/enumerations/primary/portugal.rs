//! Portuguese subdivisions, DXCC 272, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Mainland Portuguese administrative subdivisions recognized by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Aveiro.
    #[strum(serialize = "AV")]
    Aveiro,
    /// Beja.
    #[strum(serialize = "BJ")]
    Beja,
    /// Braga.
    #[strum(serialize = "BR")]
    Braga,
    /// Bragança.
    #[strum(serialize = "BG")]
    Braganca,
    /// Castelo Branco.
    #[strum(serialize = "CB")]
    CasteloBranco,
    /// Coimbra.
    #[strum(serialize = "CO")]
    Coimbra,
    /// Evora.
    #[strum(serialize = "EV")]
    Evora,
    /// Faro.
    #[strum(serialize = "FR")]
    Faro,
    /// Guarda.
    #[strum(serialize = "GD")]
    Guarda,
    /// Leiria.
    #[strum(serialize = "LR")]
    Leiria,
    /// Lisboa.
    #[strum(serialize = "LX")]
    Lisboa,
    /// Portalegre.
    #[strum(serialize = "PG")]
    Portalegre,
    /// Porto.
    #[strum(serialize = "PT")]
    Porto,
    /// Santarem.
    #[strum(serialize = "SR")]
    Santarem,
    /// Setubal.
    #[strum(serialize = "ST")]
    Setubal,
    /// Viana do Castelo.
    #[strum(serialize = "VC")]
    VianaDoCastelo,
    /// Vila Real.
    #[strum(serialize = "VR")]
    VilaReal,
    /// Viseu.
    #[strum(serialize = "VS")]
    Viseu,
}
