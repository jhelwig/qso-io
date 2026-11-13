//! Irish subdivisions, DXCC 245, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Irish counties, including the import-only Cork code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Carlow (Ceatharlach).
    #[strum(serialize = "CW")]
    Carlow,
    /// Cavan (An Cabhán).
    #[strum(serialize = "CN")]
    Cavan,
    /// Clare (An Clár).
    #[strum(serialize = "CE")]
    Clare,
    /// Cork (Corcaigh); import-only legacy code C.
    #[strum(serialize = "C")]
    CorkLegacy,
    /// Cork (Corcaigh).
    #[strum(serialize = "CO")]
    Cork,
    /// Donegal (Dún na nGall).
    #[strum(serialize = "DL")]
    Donegal,
    /// Dublin (Baile Áth Cliath).
    #[strum(serialize = "D")]
    Dublin,
    /// Galway (Gaillimh).
    #[strum(serialize = "G")]
    Galway,
    /// Kerry (Ciarraí).
    #[strum(serialize = "KY")]
    Kerry,
    /// Kildare (Cill Dara).
    #[strum(serialize = "KE")]
    Kildare,
    /// Kilkenny (Cill Chainnigh).
    #[strum(serialize = "KK")]
    Kilkenny,
    /// Laois (Laois).
    #[strum(serialize = "LS")]
    Laois,
    /// Leitrim (Liatroim).
    #[strum(serialize = "LM")]
    Leitrim,
    /// Limerick (Luimneach).
    #[strum(serialize = "LK")]
    Limerick,
    /// Longford (An Longfort).
    #[strum(serialize = "LD")]
    Longford,
    /// Louth (Lú).
    #[strum(serialize = "LH")]
    Louth,
    /// Mayo (Maigh Eo).
    #[strum(serialize = "MO")]
    Mayo,
    /// Meath (An Mhí).
    #[strum(serialize = "MH")]
    Meath,
    /// Monaghan (Muineachán).
    #[strum(serialize = "MN")]
    Monaghan,
    /// Offaly (Uíbh Fhailí).
    #[strum(serialize = "OY")]
    Offaly,
    /// Roscommon (Ros Comáin).
    #[strum(serialize = "RN")]
    Roscommon,
    /// Sligo (Sligeach).
    #[strum(serialize = "SO")]
    Sligo,
    /// Tipperary (Tiobraid Árann).
    #[strum(serialize = "TA")]
    Tipperary,
    /// Waterford (Port Láirge).
    #[strum(serialize = "WD")]
    Waterford,
    /// Westmeath (An Iarmhí).
    #[strum(serialize = "WH")]
    Westmeath,
    /// Wexford (Loch Garman).
    #[strum(serialize = "WX")]
    Wexford,
    /// Wicklow (Cill Mhantáin).
    #[strum(serialize = "WW")]
    Wicklow,
}
