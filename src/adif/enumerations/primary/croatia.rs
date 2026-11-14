//! Croatian subdivisions, DXCC 497, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Croatian counties and Grad Zagreb recognized by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Zagrebačka županija.
    #[strum(serialize = "01")]
    Zagrebacka,
    /// Krapinsko-Zagorska županija.
    #[strum(serialize = "02")]
    KrapinskoZagorska,
    /// Sisačko-Moslavačka županija.
    #[strum(serialize = "03")]
    SisackoMoslavacka,
    /// Karlovačka županija.
    #[strum(serialize = "04")]
    Karlovacka,
    /// Varaždinska županija.
    #[strum(serialize = "05")]
    Varazdinska,
    /// Koprivničko-Križevačka županija.
    #[strum(serialize = "06")]
    KoprivnickoKrizevacka,
    /// Bjelovarsko-Bilogorska županija.
    #[strum(serialize = "07")]
    BjelovarskoBilogorska,
    /// Primorsko-Goranska županija.
    #[strum(serialize = "08")]
    PrimorskoGoranska,
    /// Ličko-Senjska županija.
    #[strum(serialize = "09")]
    LickoSenjska,
    /// Virovitičko-Podravska županija.
    #[strum(serialize = "10")]
    VirovitickoPodravska,
    /// Požeško-Slavonska županija.
    #[strum(serialize = "11")]
    PozeskoSlavonska,
    /// Brodsko-Posavska županija.
    #[strum(serialize = "12")]
    BrodskoPosavska,
    /// Zadarska županija.
    #[strum(serialize = "13")]
    Zadarska,
    /// Osječko-Baranjska županija.
    #[strum(serialize = "14")]
    OsjeckoBaranjska,
    /// Šibensko-Kninska županija.
    #[strum(serialize = "15")]
    SibenskoKninska,
    /// Vukovarsko-Srijemska županija.
    #[strum(serialize = "16")]
    VukovarskoSrijemska,
    /// Splitsko-Dalmatinska županija.
    #[strum(serialize = "17")]
    SplitskoDalmatinska,
    /// Istarska županija.
    #[strum(serialize = "18")]
    Istarska,
    /// Dubrovačko-Neretvanska županija.
    #[strum(serialize = "19")]
    DubrovackoNeretvanska,
    /// Međimurska županija.
    #[strum(serialize = "20")]
    Medimurska,
    /// Grad Zagreb.
    #[strum(serialize = "21")]
    GradZagreb,
}
