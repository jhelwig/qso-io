//! Slovak subdivisions, DXCC 504, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};
/// Slovak districts recognized by ADIF.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Bratislava 1.
    #[strum(serialize = "BAA")]
    Bratislava1,
    /// Bratislava 2.
    #[strum(serialize = "BAB")]
    Bratislava2,
    /// Bratislava 3.
    #[strum(serialize = "BAC")]
    Bratislava3,
    /// Bratislava 4.
    #[strum(serialize = "BAD")]
    Bratislava4,
    /// Bratislava 5.
    #[strum(serialize = "BAE")]
    Bratislava5,
    /// Malacky.
    #[strum(serialize = "MAL")]
    Malacky,
    /// Pezinok.
    #[strum(serialize = "PEZ")]
    Pezinok,
    /// Senec.
    #[strum(serialize = "SEN")]
    Senec,
    /// Dunajska Streda.
    #[strum(serialize = "DST")]
    DunajskaStreda,
    /// Galanta.
    #[strum(serialize = "GAL")]
    Galanta,
    /// Hlohovec.
    #[strum(serialize = "HLO")]
    Hlohovec,
    /// Piestany.
    #[strum(serialize = "PIE")]
    Piestany,
    /// Senica.
    #[strum(serialize = "SEA")]
    Senica,
    /// Skalica.
    #[strum(serialize = "SKA")]
    Skalica,
    /// Trnava.
    #[strum(serialize = "TRN")]
    Trnava,
    /// Banovce n. Bebr.
    #[strum(serialize = "BAN")]
    BanovceNadBebr,
    /// Ilava.
    #[strum(serialize = "ILA")]
    Ilava,
    /// Myjava.
    #[strum(serialize = "MYJ")]
    Myjava,
    /// Nove Mesto n. Vah.
    #[strum(serialize = "NMV")]
    NoveMestoNadVah,
    /// Partizanske.
    #[strum(serialize = "PAR")]
    Partizanske,
    /// Povazska Bystrica.
    #[strum(serialize = "PBY")]
    PovazskaBystrica,
    /// Prievidza.
    #[strum(serialize = "PRI")]
    Prievidza,
    /// Puchov.
    #[strum(serialize = "PUC")]
    Puchov,
    /// Trencin.
    #[strum(serialize = "TNC")]
    Trencin,
    /// Komarno.
    #[strum(serialize = "KOM")]
    Komarno,
    /// Levice.
    #[strum(serialize = "LVC")]
    Levice,
    /// Nitra.
    #[strum(serialize = "NIT")]
    Nitra,
    /// Nove Zamky.
    #[strum(serialize = "NZA")]
    NoveZamky,
    /// Sala.
    #[strum(serialize = "SAL")]
    Sala,
    /// Topolcany.
    #[strum(serialize = "TOP")]
    Topolcany,
    /// Zlate Moravce.
    #[strum(serialize = "ZMO")]
    ZlateMoravce,
    /// Bytca.
    #[strum(serialize = "BYT")]
    Bytca,
    /// Cadca.
    #[strum(serialize = "CAD")]
    Cadca,
    /// Dolny Kubin.
    #[strum(serialize = "DKU")]
    DolnyKubin,
    /// Kysucke N. Mesto.
    #[strum(serialize = "KNM")]
    KysuckeNoveMesto,
    /// Liptovsky Mikulas.
    #[strum(serialize = "LMI")]
    LiptovskyMikulas,
    /// Martin.
    #[strum(serialize = "MAR")]
    Martin,
    /// Namestovo.
    #[strum(serialize = "NAM")]
    Namestovo,
    /// Ruzomberok.
    #[strum(serialize = "RUZ")]
    Ruzomberok,
    /// Turcianske Teplice.
    #[strum(serialize = "TTE")]
    TurcianskeTeplice,
    /// Tvrdosin.
    #[strum(serialize = "TVR")]
    Tvrdosin,
    /// Zilina.
    #[strum(serialize = "ZIL")]
    Zilina,
    /// Banska Bystrica.
    #[strum(serialize = "BBY")]
    BanskaBystrica,
    /// Banska Stiavnica.
    #[strum(serialize = "BST")]
    BanskaStiavnica,
    /// Brezno.
    #[strum(serialize = "BRE")]
    Brezno,
    /// Detva.
    #[strum(serialize = "DET")]
    Detva,
    /// Krupina.
    #[strum(serialize = "KRU")]
    Krupina,
    /// Lucenec.
    #[strum(serialize = "LUC")]
    Lucenec,
    /// Poltar.
    #[strum(serialize = "POL")]
    Poltar,
    /// Revuca.
    #[strum(serialize = "REV")]
    Revuca,
    /// Rimavska Sobota.
    #[strum(serialize = "RSO")]
    RimavskaSobota,
    /// Velky Krtis.
    #[strum(serialize = "VKR")]
    VelkyKrtis,
    /// Zarnovica.
    #[strum(serialize = "ZAR")]
    Zarnovica,
    /// Ziar nad Hronom.
    #[strum(serialize = "ZIH")]
    ZiarNadHronom,
    /// Zvolen.
    #[strum(serialize = "ZVO")]
    Zvolen,
    /// Gelnica.
    #[strum(serialize = "GEL")]
    Gelnica,
    /// Kosice 1.
    #[strum(serialize = "KEA")]
    Kosice1,
    /// Kosice 2.
    #[strum(serialize = "KEB")]
    Kosice2,
    /// Kosice 3.
    #[strum(serialize = "KEC")]
    Kosice3,
    /// Kosice 4.
    #[strum(serialize = "KED")]
    Kosice4,
    /// Kosice-okolie.
    #[strum(serialize = "KEO")]
    KosiceOkolie,
    /// Michalovce.
    #[strum(serialize = "MIC")]
    Michalovce,
    /// Roznava.
    #[strum(serialize = "ROZ")]
    Roznava,
    /// Sobrance.
    #[strum(serialize = "SOB")]
    Sobrance,
    /// Spisska Nova Ves.
    #[strum(serialize = "SNV")]
    SpisskaNovaVes,
    /// Trebisov.
    #[strum(serialize = "TRE")]
    Trebisov,
    /// Bardejov.
    #[strum(serialize = "BAR")]
    Bardejov,
    /// Humenne.
    #[strum(serialize = "HUM")]
    Humenne,
    /// Kezmarok.
    #[strum(serialize = "KEZ")]
    Kezmarok,
    /// Levoca.
    #[strum(serialize = "LEV")]
    Levoca,
    /// Medzilaborce.
    #[strum(serialize = "MED")]
    Medzilaborce,
    /// Poprad.
    #[strum(serialize = "POP")]
    Poprad,
    /// Presov.
    #[strum(serialize = "PRE")]
    Presov,
    /// Sabinov.
    #[strum(serialize = "SAB")]
    Sabinov,
    /// Snina.
    #[strum(serialize = "SNI")]
    Snina,
    /// Stara Lubovna.
    #[strum(serialize = "SLU")]
    StaraLubovna,
    /// Stropkov.
    #[strum(serialize = "STR")]
    Stropkov,
    /// Svidnik.
    #[strum(serialize = "SVI")]
    Svidnik,
    /// Vranov nad Toplou.
    #[strum(serialize = "VRT")]
    VranovNadToplou,
}
