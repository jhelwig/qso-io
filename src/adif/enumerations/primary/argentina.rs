//! Argentine primary subdivisions, DXCC entity 100.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// An Argentine primary subdivision supplied by ADIF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Capital federal (Buenos Aires City).
    #[strum(serialize = "C")]
    BuenosAiresCity,
    /// Buenos Aires Province.
    #[strum(serialize = "B")]
    BuenosAiresProvince,
    /// Santa Fe.
    #[strum(serialize = "S")]
    SantaFe,
    /// Chaco.
    #[strum(serialize = "H")]
    Chaco,
    /// Formosa.
    #[strum(serialize = "P")]
    Formosa,
    /// Córdoba.
    #[strum(serialize = "X")]
    Cordoba,
    /// Misiones.
    #[strum(serialize = "N")]
    Misiones,
    /// Entre Ríos.
    #[strum(serialize = "E")]
    EntreRios,
    /// Tucumán.
    #[strum(serialize = "T")]
    Tucuman,
    /// Corrientes.
    #[strum(serialize = "W")]
    Corrientes,
    /// Mendoza.
    #[strum(serialize = "M")]
    Mendoza,
    /// Santiago del Estero.
    #[strum(serialize = "G")]
    SantiagoDelEstero,
    /// Salta.
    #[strum(serialize = "A")]
    Salta,
    /// San Juan.
    #[strum(serialize = "J")]
    SanJuan,
    /// San Luis.
    #[strum(serialize = "D")]
    SanLuis,
    /// Catamarca.
    #[strum(serialize = "K")]
    Catamarca,
    /// La Rioja.
    #[strum(serialize = "F")]
    LaRioja,
    /// Jujuy.
    #[strum(serialize = "Y")]
    Jujuy,
    /// La Pampa.
    #[strum(serialize = "L")]
    LaPampa,
    /// Río Negro.
    #[strum(serialize = "R")]
    RioNegro,
    /// Chubut.
    #[strum(serialize = "U")]
    Chubut,
    /// Santa Cruz.
    #[strum(serialize = "Z")]
    SantaCruz,
    /// Tierra del Fuego.
    #[strum(serialize = "V")]
    TierraDelFuego,
    /// Neuquén.
    #[strum(serialize = "Q")]
    Neuquen,
}
