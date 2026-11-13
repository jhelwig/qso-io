//! Austrian subdivisions, DXCC 206, ADIF 3.1.8 III.B.12.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// Austrian districts recognized by ADIF; historical meanings are retained.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Wien.
    #[strum(serialize = "WC")]
    Wien,
    /// Hallein.
    #[strum(serialize = "HA")]
    Hallein,
    /// St. Johann.
    #[strum(serialize = "JO")]
    StJohann,
    /// Salzburg.
    #[strum(serialize = "SC")]
    Salzburg,
    /// Salzburg-Land.
    #[strum(serialize = "SL")]
    SalzburgLand,
    /// Tamsweg.
    #[strum(serialize = "TA")]
    Tamsweg,
    /// Zell Am See.
    #[strum(serialize = "ZE")]
    ZellAmSee,
    /// Amstetten.
    #[strum(serialize = "AM")]
    Amstetten,
    /// Bruck/Leitha.
    #[strum(serialize = "BL")]
    BruckLeitha,
    /// Baden.
    #[strum(serialize = "BN")]
    Baden,
    /// Gmünd.
    #[strum(serialize = "GD")]
    Gmund,
    /// Gänserndorf.
    #[strum(serialize = "GF")]
    Ganserndorf,
    /// Hollabrunn.
    #[strum(serialize = "HL")]
    Hollabrunn,
    /// Horn.
    #[strum(serialize = "HO")]
    Horn,
    /// Korneuburg.
    #[strum(serialize = "KO")]
    Korneuburg,
    /// Krems-Region.
    #[strum(serialize = "KR")]
    KremsRegion,
    /// Krems.
    #[strum(serialize = "KS")]
    Krems,
    /// Lilienfeld.
    #[strum(serialize = "LF")]
    Lilienfeld,
    /// Mödling.
    #[strum(serialize = "MD")]
    Modling,
    /// Melk.
    #[strum(serialize = "ME")]
    Melk,
    /// Mistelbach.
    #[strum(serialize = "MI")]
    Mistelbach,
    /// Neunkirchen.
    #[strum(serialize = "NK")]
    Neunkirchen,
    /// St. Pölten.
    #[strum(serialize = "PC")]
    StPolten,
    /// St. Pölten-Land.
    #[strum(serialize = "PL")]
    StPoltenLand,
    /// Scheibbs.
    #[strum(serialize = "SB")]
    Scheibbs,
    /// Schwechat.
    #[strum(serialize = "SW")]
    Schwechat,
    /// Tulln.
    #[strum(serialize = "TU")]
    Tulln,
    /// Wr.Neustadt-Bezirk.
    #[strum(serialize = "WB")]
    WienerNeustadtBezirk,
    /// Wr.Neustadt.
    #[strum(serialize = "WN")]
    WienerNeustadt,
    /// Waidhofen/Thaya.
    #[strum(serialize = "WT")]
    WaidhofenThaya,
    /// Wien-Umgebung.
    #[strum(serialize = "WU")]
    WienUmgebung,
    /// Waidhofen/Ybbs.
    #[strum(serialize = "WY")]
    WaidhofenYbbs,
    /// Zwettl.
    #[strum(serialize = "ZT")]
    Zwettl,
    /// Eisenstadt.
    #[strum(serialize = "EC")]
    Eisenstadt,
    /// Eisenstadt-Umgebung.
    #[strum(serialize = "EU")]
    EisenstadtUmgebung,
    /// Güssing.
    #[strum(serialize = "GS")]
    Gussing,
    /// Jennersdorf.
    #[strum(serialize = "JE")]
    Jennersdorf,
    /// Mattersburg.
    #[strum(serialize = "MA")]
    Mattersburg,
    /// Neusiedl/See.
    #[strum(serialize = "ND")]
    NeusiedlSee,
    /// Oberpullendorf.
    #[strum(serialize = "OP")]
    Oberpullendorf,
    /// Oberwart.
    #[strum(serialize = "OW")]
    Oberwart,
    /// Braunau/Inn.
    #[strum(serialize = "BR")]
    BraunauInn,
    /// Eferding.
    #[strum(serialize = "EF")]
    Eferding,
    /// Freistadt.
    #[strum(serialize = "FR")]
    Freistadt,
    /// Gmunden.
    #[strum(serialize = "GM")]
    Gmunden,
    /// Grieskirchen.
    #[strum(serialize = "GR")]
    Grieskirchen,
    /// Kirchdorf.
    #[strum(serialize = "KI")]
    Kirchdorf,
    /// Linz.
    #[strum(serialize = "LC")]
    Linz,
    /// Linz-Land.
    #[strum(serialize = "LL")]
    LinzLand,
    /// Perg.
    #[strum(serialize = "PE")]
    Perg,
    /// Ried/Innkreis.
    #[strum(serialize = "RI")]
    RiedInnkreis,
    /// Rohrbach.
    #[strum(serialize = "RO")]
    Rohrbach,
    /// Schärding.
    #[strum(serialize = "SD")]
    Scharding,
    /// Steyr-Land.
    #[strum(serialize = "SE")]
    SteyrLand,
    /// Steyr.
    #[strum(serialize = "SR")]
    Steyr,
    /// Urfahr.
    #[strum(serialize = "UU")]
    Urfahr,
    /// Vöcklabruck.
    #[strum(serialize = "VB")]
    Vocklabruck,
    /// Wels.
    #[strum(serialize = "WE")]
    Wels,
    /// Wels-Land.
    #[strum(serialize = "WL")]
    WelsLand,
    /// Bad Aussee, for contacts before 2012/01/01.
    #[strum(serialize = "BA")]
    BadAussee,
    /// Bruck/Mur before 2013/01/01; Bruck-Mürzzuschlag on or after that date.
    #[strum(serialize = "BM")]
    BruckMurBruckMurzzuschlag,
    /// Deutschlandsberg.
    #[strum(serialize = "DL")]
    Deutschlandsberg,
    /// Feldbach, for contacts before 2013/01/01.
    #[strum(serialize = "FB")]
    Feldbach,
    /// Fürstenfeld, for contacts before 2013/01/01.
    #[strum(serialize = "FF")]
    Furstenfeld,
    /// Gröbming.
    #[strum(serialize = "GB")]
    Grobming,
    /// Graz.
    #[strum(serialize = "GC")]
    Graz,
    /// Graz-Umgebung.
    #[strum(serialize = "GU")]
    GrazUmgebung,
    /// Hartberg, for contacts before 2013/01/01.
    #[strum(serialize = "HB")]
    Hartberg,
    /// Hartberg-Fürstenfeld, for contacts on or after 2013/01/01.
    #[strum(serialize = "HF")]
    HartbergFurstenfeld,
    /// Judenburg, for contacts before 2012/01/01.
    #[strum(serialize = "JU")]
    Judenburg,
    /// Knittelfeld, for contacts before 2012/01/01.
    #[strum(serialize = "KF")]
    Knittelfeld,
    /// Leibnitz.
    #[strum(serialize = "LB")]
    Leibnitz,
    /// Leoben.
    #[strum(serialize = "LE")]
    Leoben,
    /// Liezen.
    #[strum(serialize = "LI")]
    Liezen,
    /// Leoben-Land.
    #[strum(serialize = "LN")]
    LeobenLand,
    /// Murtal, for contacts on or after 2012/01/01.
    #[strum(serialize = "MT")]
    Murtal,
    /// Murau.
    #[strum(serialize = "MU")]
    Murau,
    /// Mürzzuschlag, for contacts before 2013/01/01.
    #[strum(serialize = "MZ")]
    Murzzuschlag,
    /// Radkersburg, for contacts before 2013/01/01.
    #[strum(serialize = "RA")]
    Radkersburg,
    /// Südoststeiermark, for contacts on or after 2013/01/01.
    #[strum(serialize = "SO")]
    Sudoststeiermark,
    /// Voitsberg.
    #[strum(serialize = "VO")]
    Voitsberg,
    /// Weiz.
    #[strum(serialize = "WZ")]
    Weiz,
    /// Innsbruck.
    #[strum(serialize = "IC")]
    Innsbruck,
    /// Innsbruck-Land.
    #[strum(serialize = "IL")]
    InnsbruckLand,
    /// Imst.
    #[strum(serialize = "IM")]
    Imst,
    /// Kitzbühel.
    #[strum(serialize = "KB")]
    Kitzbuhel,
    /// Kufstein.
    #[strum(serialize = "KU")]
    Kufstein,
    /// Landeck.
    #[strum(serialize = "LA")]
    Landeck,
    /// Lienz.
    #[strum(serialize = "LZ")]
    Lienz,
    /// Reutte.
    #[strum(serialize = "RE")]
    Reutte,
    /// Schwaz.
    #[strum(serialize = "SZ")]
    Schwaz,
    /// Feldkirchen.
    #[strum(serialize = "FE")]
    Feldkirchen,
    /// Hermagor.
    #[strum(serialize = "HE")]
    Hermagor,
    /// Klagenfurt.
    #[strum(serialize = "KC")]
    Klagenfurt,
    /// Klagenfurt-Land.
    #[strum(serialize = "KL")]
    KlagenfurtLand,
    /// Spittal/Drau.
    #[strum(serialize = "SP")]
    SpittalDrau,
    /// St.Veit/Glan.
    #[strum(serialize = "SV")]
    StVeitGlan,
    /// Villach.
    #[strum(serialize = "VI")]
    Villach,
    /// Völkermarkt.
    #[strum(serialize = "VK")]
    Volkermarkt,
    /// Villach-Land.
    #[strum(serialize = "VL")]
    VillachLand,
    /// Wolfsberg.
    #[strum(serialize = "WO")]
    Wolfsberg,
    /// Bregenz.
    #[strum(serialize = "BC")]
    Bregenz,
    /// Bludenz.
    #[strum(serialize = "BZ")]
    Bludenz,
    /// Dornbirn.
    #[strum(serialize = "DO")]
    Dornbirn,
    /// Feldkirch.
    #[strum(serialize = "FK")]
    Feldkirch,
}
