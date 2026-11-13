//! Brazilian primary subdivisions, DXCC entity 108.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Brazilian primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Espírito Santo.
    #[strum(serialize = "ES")]
    EspiritoSanto,
    /// Goiás.
    #[strum(serialize = "GO")]
    Goias,
    /// Santa Catarina.
    #[strum(serialize = "SC")]
    SantaCatarina,
    /// Sergipe.
    #[strum(serialize = "SE")]
    Sergipe,
    /// Alagoas.
    #[strum(serialize = "AL")]
    Alagoas,
    /// Amazonas.
    #[strum(serialize = "AM")]
    Amazonas,
    /// Tocantins.
    #[strum(serialize = "TO")]
    Tocantins,
    /// Amapá.
    #[strum(serialize = "AP")]
    Amapa,
    /// Paraíba.
    #[strum(serialize = "PB")]
    Paraiba,
    /// Maranhão.
    #[strum(serialize = "MA")]
    Maranhao,
    /// Rio Grande do Norte.
    #[strum(serialize = "RN")]
    RioGrandeDoNorte,
    /// Piauí.
    #[strum(serialize = "PI")]
    Piaui,
    /// Distrito Federal.
    #[strum(serialize = "DF")]
    DistritoFederal,
    /// Ceará.
    #[strum(serialize = "CE")]
    Ceara,
    /// Acre.
    #[strum(serialize = "AC")]
    Acre,
    /// Mato Grosso do Sul.
    #[strum(serialize = "MS")]
    MatoGrossoDoSul,
    /// Roraima.
    #[strum(serialize = "RR")]
    Roraima,
    /// Rondônia.
    #[strum(serialize = "RO")]
    Rondonia,
    /// Rio de Janeiro.
    #[strum(serialize = "RJ")]
    RioDeJaneiro,
    /// São Paulo.
    #[strum(serialize = "SP")]
    SaoPaulo,
    /// Rio Grande do Sul.
    #[strum(serialize = "RS")]
    RioGrandeDoSul,
    /// Minas Gerais.
    #[strum(serialize = "MG")]
    MinasGerais,
    /// Paraná.
    #[strum(serialize = "PR")]
    Parana,
    /// Bahia.
    #[strum(serialize = "BA")]
    Bahia,
    /// Pernambuco.
    #[strum(serialize = "PE")]
    Pernambuco,
    /// Pará.
    #[strum(serialize = "PA")]
    Para,
    /// Mato Grosso.
    #[strum(serialize = "MT")]
    MatoGrosso,
}
