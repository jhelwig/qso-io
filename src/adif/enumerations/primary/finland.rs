//! Finland municipality identities supplied by ADIF, DXCC entity 224.
use strum::{EnumIter, EnumString, IntoStaticStr};

/// A Finnish municipality identity in ADIF's primary subdivision table.
///
/// The specification's municipality list includes historical municipalities;
/// the identities describe the ADIF reference set rather than current boundaries.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Somero.
    #[strum(serialize = "100")]
    Somero,
    /// Alastaro.
    #[strum(serialize = "102")]
    Alastaro,
    /// Askainen.
    #[strum(serialize = "103")]
    Askainen,
    /// Aura.
    #[strum(serialize = "104")]
    Aura,
    /// Dragsfjärd.
    #[strum(serialize = "105")]
    Dragsfjard,
    /// Eura.
    #[strum(serialize = "106")]
    Eura,
    /// Eurajoki.
    #[strum(serialize = "107")]
    Eurajoki,
    /// Halikko.
    #[strum(serialize = "108")]
    Halikko,
    /// Harjavalta.
    #[strum(serialize = "109")]
    Harjavalta,
    /// Honkajoki.
    #[strum(serialize = "110")]
    Honkajoki,
    /// Houtskari.
    #[strum(serialize = "111")]
    Houtskari,
    /// Huittinen.
    #[strum(serialize = "112")]
    Huittinen,
    /// Iniö.
    #[strum(serialize = "115")]
    Inio,
    /// Jämijärvi.
    #[strum(serialize = "116")]
    Jamijarvi,
    /// Kaarina.
    #[strum(serialize = "117")]
    Kaarina,
    /// Kankaanpää.
    #[strum(serialize = "119")]
    Kankaanpaa,
    /// Karinainen.
    #[strum(serialize = "120")]
    Karinainen,
    /// Karvia.
    #[strum(serialize = "122")]
    Karvia,
    /// Äetsä.
    #[strum(serialize = "123")]
    Aetsa,
    /// Kemiö.
    #[strum(serialize = "124")]
    Kemio,
    /// Kiikala.
    #[strum(serialize = "126")]
    Kiikala,
    /// Kiikoinen.
    #[strum(serialize = "128")]
    Kiikoinen,
    /// Kisko.
    #[strum(serialize = "129")]
    Kisko,
    /// Kiukainen.
    #[strum(serialize = "130")]
    Kiukainen,
    /// Kodisjoki.
    #[strum(serialize = "131")]
    Kodisjoki,
    /// Kokemäki.
    #[strum(serialize = "132")]
    Kokemaki,
    /// Korppoo.
    #[strum(serialize = "133")]
    Korppoo,
    /// Koski tl.
    #[strum(serialize = "134")]
    KoskiTl,
    /// Kullaa.
    #[strum(serialize = "135")]
    Kullaa,
    /// Kustavi.
    #[strum(serialize = "136")]
    Kustavi,
    /// Kuusjoki.
    #[strum(serialize = "137")]
    Kuusjoki,
    /// Köyliö.
    #[strum(serialize = "138")]
    Koylio,
    /// Laitila.
    #[strum(serialize = "139")]
    Laitila,
    /// Lappi.
    #[strum(serialize = "140")]
    Lappi,
    /// Lavia.
    #[strum(serialize = "141")]
    Lavia,
    /// Lemu.
    #[strum(serialize = "142")]
    Lemu,
    /// Lieto.
    #[strum(serialize = "143")]
    Lieto,
    /// Loimaa.
    #[strum(serialize = "144")]
    Loimaa,
    /// Loimaan kunta.
    #[strum(serialize = "145")]
    LoimaanKunta,
    /// Luvia.
    #[strum(serialize = "147")]
    Luvia,
    /// Marttila.
    #[strum(serialize = "148")]
    Marttila,
    /// Masku.
    #[strum(serialize = "149")]
    Masku,
    /// Mellilä.
    #[strum(serialize = "150")]
    Mellila,
    /// Merikarvia.
    #[strum(serialize = "151")]
    Merikarvia,
    /// Merimasku.
    #[strum(serialize = "152")]
    Merimasku,
    /// Mietoinen.
    #[strum(serialize = "154")]
    Mietoinen,
    /// Muurla.
    #[strum(serialize = "156")]
    Muurla,
    /// Mynämäki.
    #[strum(serialize = "157")]
    Mynamaki,
    /// Naantali.
    #[strum(serialize = "158")]
    Naantali,
    /// Nakkila.
    #[strum(serialize = "159")]
    Nakkila,
    /// Nauvo.
    #[strum(serialize = "160")]
    Nauvo,
    /// Noormarkku.
    #[strum(serialize = "161")]
    Noormarkku,
    /// Nousiainen.
    #[strum(serialize = "162")]
    Nousiainen,
    /// Oripää.
    #[strum(serialize = "163")]
    Oripaa,
    /// Paimio.
    #[strum(serialize = "164")]
    Paimio,
    /// Parainen.
    #[strum(serialize = "165")]
    Parainen,
    /// Perniö.
    #[strum(serialize = "167")]
    Pernio,
    /// Pertteli.
    #[strum(serialize = "168")]
    Pertteli,
    /// Piikkiö.
    #[strum(serialize = "169")]
    Piikkio,
    /// Pomarkku.
    #[strum(serialize = "170")]
    Pomarkku,
    /// Pori.
    #[strum(serialize = "171")]
    Pori,
    /// Punkalaidun.
    #[strum(serialize = "172")]
    Punkalaidun,
    /// Pyhäranta.
    #[strum(serialize = "173")]
    Pyharanta,
    /// Pöytyä.
    #[strum(serialize = "174")]
    Poytya,
    /// Raisio.
    #[strum(serialize = "175")]
    Raisio,
    /// Rauma.
    #[strum(serialize = "176")]
    Rauma,
    /// Rusko.
    #[strum(serialize = "178")]
    Rusko,
    /// Rymättylä.
    #[strum(serialize = "179")]
    Rymattyla,
    /// Salo.
    #[strum(serialize = "180")]
    Salo,
    /// Sauvo.
    #[strum(serialize = "181")]
    Sauvo,
    /// Siikainen.
    #[strum(serialize = "182")]
    Siikainen,
    /// Suodenniemi.
    #[strum(serialize = "183")]
    Suodenniemi,
    /// Suomusjärvi.
    #[strum(serialize = "184")]
    Suomusjarvi,
    /// Säkylä.
    #[strum(serialize = "185")]
    Sakyla,
    /// Särkisalo.
    #[strum(serialize = "186")]
    Sarkisalo,
    /// Taivassalo.
    #[strum(serialize = "187")]
    Taivassalo,
    /// Tarvasjoki.
    #[strum(serialize = "188")]
    Tarvasjoki,
    /// Turku.
    #[strum(serialize = "189")]
    Turku,
    /// Ulvila.
    #[strum(serialize = "190")]
    Ulvila,
    /// Uusikaupunki.
    #[strum(serialize = "191")]
    Uusikaupunki,
    /// Vahto.
    #[strum(serialize = "192")]
    Vahto,
    /// Vammala.
    #[strum(serialize = "193")]
    Vammala,
    /// Vampula.
    #[strum(serialize = "194")]
    Vampula,
    /// Vehmaa.
    #[strum(serialize = "195")]
    Vehmaa,
    /// Velkua.
    #[strum(serialize = "196")]
    Velkua,
    /// Västanfjärd.
    #[strum(serialize = "198")]
    Vastanfjard,
    /// Yläne.
    #[strum(serialize = "199")]
    Ylane,
    /// Artjärvi.
    #[strum(serialize = "201")]
    Artjarvi,
    /// Askola.
    #[strum(serialize = "202")]
    Askola,
    /// Espoo.
    #[strum(serialize = "204")]
    Espoo,
    /// Hanko.
    #[strum(serialize = "205")]
    Hanko,
    /// Helsinki.
    #[strum(serialize = "206")]
    Helsinki,
    /// Hyvinkää.
    #[strum(serialize = "207")]
    Hyvinkaa,
    /// Inkoo.
    #[strum(serialize = "208")]
    Inkoo,
    /// Järvenpää.
    #[strum(serialize = "209")]
    Jarvenpaa,
    /// Karjaa.
    #[strum(serialize = "210")]
    Karjaa,
    /// Karjalohja.
    #[strum(serialize = "211")]
    Karjalohja,
    /// Karkkila.
    #[strum(serialize = "212")]
    Karkkila,
    /// Kauniainen.
    #[strum(serialize = "213")]
    Kauniainen,
    /// Kerava.
    #[strum(serialize = "214")]
    Kerava,
    /// Kirkkonummi.
    #[strum(serialize = "215")]
    Kirkkonummi,
    /// Lapinjärvi.
    #[strum(serialize = "216")]
    Lapinjarvi,
    /// Liljendal.
    #[strum(serialize = "217")]
    Liljendal,
    /// Lohjan kaupunki.
    #[strum(serialize = "218")]
    LohjanKaupunki,
    /// Loviisa.
    #[strum(serialize = "220")]
    Loviisa,
    /// Myrskylä.
    #[strum(serialize = "221")]
    Myrskyla,
    /// Mäntsälä.
    #[strum(serialize = "222")]
    Mantsala,
    /// Nummi-Pusula.
    #[strum(serialize = "223")]
    NummiPusula,
    /// Nurmijärvi.
    #[strum(serialize = "224")]
    Nurmijarvi,
    /// Orimattila.
    #[strum(serialize = "225")]
    Orimattila,
    /// Pernaja.
    #[strum(serialize = "226")]
    Pernaja,
    /// Pohja.
    #[strum(serialize = "227")]
    Pohja,
    /// Pornainen.
    #[strum(serialize = "228")]
    Pornainen,
    /// Porvoo.
    #[strum(serialize = "229")]
    Porvoo,
    /// Pukkila.
    #[strum(serialize = "231")]
    Pukkila,
    /// Ruotsinpyhtää.
    #[strum(serialize = "233")]
    Ruotsinpyhtaa,
    /// Sammatti.
    #[strum(serialize = "234")]
    Sammatti,
    /// Sipoo.
    #[strum(serialize = "235")]
    Sipoo,
    /// Siuntio.
    #[strum(serialize = "236")]
    Siuntio,
    /// Tammisaari.
    #[strum(serialize = "238")]
    Tammisaari,
    /// Tuusula.
    #[strum(serialize = "241")]
    Tuusula,
    /// Vantaa.
    #[strum(serialize = "242")]
    Vantaa,
    /// Vihti.
    #[strum(serialize = "243")]
    Vihti,
    /// Asikkala.
    #[strum(serialize = "301")]
    Asikkala,
    /// Forssa.
    #[strum(serialize = "303")]
    Forssa,
    /// Hattula.
    #[strum(serialize = "304")]
    Hattula,
    /// Hauho.
    #[strum(serialize = "305")]
    Hauho,
    /// Hausjärvi.
    #[strum(serialize = "306")]
    Hausjarvi,
    /// Hollola.
    #[strum(serialize = "307")]
    Hollola,
    /// Humppila.
    #[strum(serialize = "308")]
    Humppila,
    /// Hämeenlinna.
    #[strum(serialize = "309")]
    Hameenlinna,
    /// Janakkala.
    #[strum(serialize = "310")]
    Janakkala,
    /// Jokioinen.
    #[strum(serialize = "311")]
    Jokioinen,
    /// Juupajoki.
    #[strum(serialize = "312")]
    Juupajoki,
    /// Kalvola.
    #[strum(serialize = "313")]
    Kalvola,
    /// Kangasala.
    #[strum(serialize = "314")]
    Kangasala,
    /// Hämeenkoski.
    #[strum(serialize = "315")]
    Hameenkoski,
    /// Kuhmalahti.
    #[strum(serialize = "316")]
    Kuhmalahti,
    /// Kuru.
    #[strum(serialize = "318")]
    Kuru,
    /// Kylmäkoski.
    #[strum(serialize = "319")]
    Kylmakoski,
    /// Kärkölä.
    #[strum(serialize = "320")]
    Karkola,
    /// Lahti.
    #[strum(serialize = "321")]
    Lahti,
    /// Lammi.
    #[strum(serialize = "322")]
    Lammi,
    /// Lempäälä.
    #[strum(serialize = "323")]
    Lempaala,
    /// Loppi.
    #[strum(serialize = "324")]
    Loppi,
    /// Luopioinen.
    #[strum(serialize = "325")]
    Luopioinen,
    /// Längelmäki.
    #[strum(serialize = "326")]
    Langelmaki,
    /// Mänttä.
    #[strum(serialize = "327")]
    Mantta,
    /// Nastola.
    #[strum(serialize = "328")]
    Nastola,
    /// Nokia.
    #[strum(serialize = "329")]
    Nokia,
    /// Orivesi.
    #[strum(serialize = "330")]
    Orivesi,
    /// Padasjoki.
    #[strum(serialize = "331")]
    Padasjoki,
    /// Pirkkala.
    #[strum(serialize = "332")]
    Pirkkala,
    /// Pälkäne.
    #[strum(serialize = "333")]
    Palkane,
    /// Renko.
    #[strum(serialize = "334")]
    Renko,
    /// Riihimäki.
    #[strum(serialize = "335")]
    Riihimaki,
    /// Ruovesi.
    #[strum(serialize = "336")]
    Ruovesi,
    /// Sahalahti.
    #[strum(serialize = "337")]
    Sahalahti,
    /// Tammela.
    #[strum(serialize = "340")]
    Tammela,
    /// Tampere.
    #[strum(serialize = "341")]
    Tampere,
    /// Toijala.
    #[strum(serialize = "342")]
    Toijala,
    /// Tuulos.
    #[strum(serialize = "344")]
    Tuulos,
    /// Urjala.
    #[strum(serialize = "345")]
    Urjala,
    /// Valkeakoski.
    #[strum(serialize = "346")]
    Valkeakoski,
    /// Vesilahti.
    #[strum(serialize = "347")]
    Vesilahti,
    /// Viiala.
    #[strum(serialize = "348")]
    Viiala,
    /// Vilppula.
    #[strum(serialize = "349")]
    Vilppula,
    /// Virrat.
    #[strum(serialize = "350")]
    Virrat,
    /// Ylöjärvi.
    #[strum(serialize = "351")]
    Ylojarvi,
    /// Ypäjä.
    #[strum(serialize = "352")]
    Ypaja,
    /// Hämeenkyrö.
    #[strum(serialize = "353")]
    Hameenkyro,
    /// Ikaalinen.
    #[strum(serialize = "354")]
    Ikaalinen,
    /// Kihniö.
    #[strum(serialize = "355")]
    Kihnio,
    /// Mouhijärvi.
    #[strum(serialize = "356")]
    Mouhijarvi,
    /// Parkano.
    #[strum(serialize = "357")]
    Parkano,
    /// Viljakkala.
    #[strum(serialize = "358")]
    Viljakkala,
    /// Enonkoski.
    #[strum(serialize = "402")]
    Enonkoski,
    /// Hartola.
    #[strum(serialize = "403")]
    Hartola,
    /// Haukivuori.
    #[strum(serialize = "404")]
    Haukivuori,
    /// Heinola.
    #[strum(serialize = "405")]
    Heinola,
    /// Heinävesi.
    #[strum(serialize = "407")]
    Heinavesi,
    /// Hirvensalmi.
    #[strum(serialize = "408")]
    Hirvensalmi,
    /// Joroinen.
    #[strum(serialize = "409")]
    Joroinen,
    /// Juva.
    #[strum(serialize = "410")]
    Juva,
    /// Jäppilä.
    #[strum(serialize = "411")]
    Jappila,
    /// Kangaslampi.
    #[strum(serialize = "412")]
    Kangaslampi,
    /// Kangasniemi.
    #[strum(serialize = "413")]
    Kangasniemi,
    /// Kerimäki.
    #[strum(serialize = "414")]
    Kerimaki,
    /// Mikkeli.
    #[strum(serialize = "415")]
    Mikkeli,
    /// Mäntyharju.
    #[strum(serialize = "417")]
    Mantyharju,
    /// Pertunmaa.
    #[strum(serialize = "418")]
    Pertunmaa,
    /// Pieksämäki.
    #[strum(serialize = "419")]
    Pieksamaki,
    /// Pieksänmaa.
    #[strum(serialize = "420")]
    Pieksanmaa,
    /// Punkaharju.
    #[strum(serialize = "421")]
    Punkaharju,
    /// Puumala.
    #[strum(serialize = "422")]
    Puumala,
    /// Rantasalmi.
    #[strum(serialize = "423")]
    Rantasalmi,
    /// Ristiina.
    #[strum(serialize = "424")]
    Ristiina,
    /// Savonlinna.
    #[strum(serialize = "425")]
    Savonlinna,
    /// Savonranta.
    #[strum(serialize = "426")]
    Savonranta,
    /// Sulkava.
    #[strum(serialize = "427")]
    Sulkava,
    /// Sysmä.
    #[strum(serialize = "428")]
    Sysma,
    /// Elimäki.
    #[strum(serialize = "502")]
    Elimaki,
    /// Hamina.
    #[strum(serialize = "503")]
    Hamina,
    /// Iitti.
    #[strum(serialize = "504")]
    Iitti,
    /// Imatra.
    #[strum(serialize = "505")]
    Imatra,
    /// Jaala.
    #[strum(serialize = "506")]
    Jaala,
    /// Joutseno.
    #[strum(serialize = "507")]
    Joutseno,
    /// Kotka.
    #[strum(serialize = "509")]
    Kotka,
    /// Kouvola.
    #[strum(serialize = "510")]
    Kouvola,
    /// Kuusankoski.
    #[strum(serialize = "511")]
    Kuusankoski,
    /// Lappeenranta.
    #[strum(serialize = "513")]
    Lappeenranta,
    /// Lemi.
    #[strum(serialize = "514")]
    Lemi,
    /// Luumäki.
    #[strum(serialize = "515")]
    Luumaki,
    /// Miehikkälä.
    #[strum(serialize = "516")]
    Miehikkala,
    /// Parikkala.
    #[strum(serialize = "518")]
    Parikkala,
    /// Pyhtää.
    #[strum(serialize = "519")]
    Pyhtaa,
    /// Rautjärvi.
    #[strum(serialize = "520")]
    Rautjarvi,
    /// Ruokolahti.
    #[strum(serialize = "521")]
    Ruokolahti,
    /// Saari.
    #[strum(serialize = "522")]
    Saari,
    /// Savitaipale.
    #[strum(serialize = "523")]
    Savitaipale,
    /// Suomenniemi.
    #[strum(serialize = "525")]
    Suomenniemi,
    /// Taipalsaari.
    #[strum(serialize = "526")]
    Taipalsaari,
    /// Uukuniemi.
    #[strum(serialize = "527")]
    Uukuniemi,
    /// Valkeala.
    #[strum(serialize = "528")]
    Valkeala,
    /// Virolahti.
    #[strum(serialize = "530")]
    Virolahti,
    /// Ylämaa.
    #[strum(serialize = "531")]
    Ylamaa,
    /// Anjalankoski.
    #[strum(serialize = "532")]
    Anjalankoski,
    /// Alahärmä.
    #[strum(serialize = "601")]
    Alaharma,
    /// Alajärvi.
    #[strum(serialize = "602")]
    Alajarvi,
    /// Alavus.
    #[strum(serialize = "603")]
    Alavus,
    /// Evijärvi.
    #[strum(serialize = "604")]
    Evijarvi,
    /// Halsua.
    #[strum(serialize = "605")]
    Halsua,
    /// Hankasalmi.
    #[strum(serialize = "606")]
    Hankasalmi,
    /// Himanka.
    #[strum(serialize = "607")]
    Himanka,
    /// Ilmajoki.
    #[strum(serialize = "608")]
    Ilmajoki,
    /// Isojoki.
    #[strum(serialize = "609")]
    Isojoki,
    /// Isokyrö.
    #[strum(serialize = "610")]
    Isokyro,
    /// Jalasjärvi.
    #[strum(serialize = "611")]
    Jalasjarvi,
    /// Joutsa.
    #[strum(serialize = "612")]
    Joutsa,
    /// Jurva.
    #[strum(serialize = "613")]
    Jurva,
    /// Jyväskylä.
    #[strum(serialize = "614")]
    Jyvaskyla,
    /// Jyväskylän mlk.
    #[strum(serialize = "615")]
    JyvaskylanMlk,
    /// Jämsä.
    #[strum(serialize = "616")]
    Jamsa,
    /// Jämsänkoski.
    #[strum(serialize = "617")]
    Jamsankoski,
    /// Kannonkoski.
    #[strum(serialize = "619")]
    Kannonkoski,
    /// Kannus.
    #[strum(serialize = "620")]
    Kannus,
    /// Karijoki.
    #[strum(serialize = "621")]
    Karijoki,
    /// Karstula.
    #[strum(serialize = "622")]
    Karstula,
    /// Kaskinen.
    #[strum(serialize = "623")]
    Kaskinen,
    /// Kauhajoki.
    #[strum(serialize = "624")]
    Kauhajoki,
    /// Kauhava.
    #[strum(serialize = "625")]
    Kauhava,
    /// Kaustinen.
    #[strum(serialize = "626")]
    Kaustinen,
    /// Keuruu.
    #[strum(serialize = "627")]
    Keuruu,
    /// Kinnula.
    #[strum(serialize = "628")]
    Kinnula,
    /// Kivijärvi.
    #[strum(serialize = "629")]
    Kivijarvi,
    /// Kokkola.
    #[strum(serialize = "630")]
    Kokkola,
    /// Konnevesi.
    #[strum(serialize = "632")]
    Konnevesi,
    /// Korpilahti.
    #[strum(serialize = "633")]
    Korpilahti,
    /// Korsnäs.
    #[strum(serialize = "634")]
    Korsnas,
    /// Kortesjärvi.
    #[strum(serialize = "635")]
    Kortesjarvi,
    /// Kristiinankaupunki.
    #[strum(serialize = "636")]
    Kristiinankaupunki,
    /// Kruunupyy.
    #[strum(serialize = "637")]
    Kruunupyy,
    /// Kuhmoinen.
    #[strum(serialize = "638")]
    Kuhmoinen,
    /// Kuortane.
    #[strum(serialize = "639")]
    Kuortane,
    /// Kurikka.
    #[strum(serialize = "640")]
    Kurikka,
    /// Kyyjärvi.
    #[strum(serialize = "641")]
    Kyyjarvi,
    /// Kälviä.
    #[strum(serialize = "642")]
    Kalvia,
    /// Laihia.
    #[strum(serialize = "643")]
    Laihia,
    /// Lappajärvi.
    #[strum(serialize = "644")]
    Lappajarvi,
    /// Lapua.
    #[strum(serialize = "645")]
    Lapua,
    /// Laukaa.
    #[strum(serialize = "646")]
    Laukaa,
    /// Lehtimäki.
    #[strum(serialize = "647")]
    Lehtimaki,
    /// Leivonmäki.
    #[strum(serialize = "648")]
    Leivonmaki,
    /// Lestijärvi.
    #[strum(serialize = "649")]
    Lestijarvi,
    /// Lohtaja.
    #[strum(serialize = "650")]
    Lohtaja,
    /// Luhanka.
    #[strum(serialize = "651")]
    Luhanka,
    /// Luoto.
    #[strum(serialize = "652")]
    Luoto,
    /// Maalahti.
    #[strum(serialize = "653")]
    Maalahti,
    /// Maksamaa.
    #[strum(serialize = "654")]
    Maksamaa,
    /// Multia.
    #[strum(serialize = "655")]
    Multia,
    /// Mustasaari.
    #[strum(serialize = "656")]
    Mustasaari,
    /// Muurame.
    #[strum(serialize = "657")]
    Muurame,
    /// Nurmo.
    #[strum(serialize = "658")]
    Nurmo,
    /// Närpiö.
    #[strum(serialize = "659")]
    Narpio,
    /// Oravainen.
    #[strum(serialize = "660")]
    Oravainen,
    /// Perho.
    #[strum(serialize = "661")]
    Perho,
    /// Peräseinäjoki.
    #[strum(serialize = "662")]
    Peraseinajoki,
    /// Petäjävesi.
    #[strum(serialize = "663")]
    Petajavesi,
    /// Pietarsaari.
    #[strum(serialize = "664")]
    Pietarsaari,
    /// Pedersöre.
    #[strum(serialize = "665")]
    Pedersore,
    /// Pihtipudas.
    #[strum(serialize = "666")]
    Pihtipudas,
    /// Pylkönmäki.
    #[strum(serialize = "668")]
    Pylkonmaki,
    /// Saarijärvi.
    #[strum(serialize = "669")]
    Saarijarvi,
    /// Seinäjoki.
    #[strum(serialize = "670")]
    Seinajoki,
    /// Soini.
    #[strum(serialize = "671")]
    Soini,
    /// Sumiainen.
    #[strum(serialize = "672")]
    Sumiainen,
    /// Suolahti.
    #[strum(serialize = "673")]
    Suolahti,
    /// Teuva.
    #[strum(serialize = "675")]
    Teuva,
    /// Toholampi.
    #[strum(serialize = "676")]
    Toholampi,
    /// Toivakka.
    #[strum(serialize = "677")]
    Toivakka,
    /// Töysä.
    #[strum(serialize = "678")]
    Toysa,
    /// Ullava.
    #[strum(serialize = "679")]
    Ullava,
    /// Uurainen.
    #[strum(serialize = "680")]
    Uurainen,
    /// Uusikaarlepyy.
    #[strum(serialize = "681")]
    Uusikaarlepyy,
    /// Vaasa.
    #[strum(serialize = "682")]
    Vaasa,
    /// Veteli.
    #[strum(serialize = "683")]
    Veteli,
    /// Viitasaari.
    #[strum(serialize = "684")]
    Viitasaari,
    /// Vimpeli.
    #[strum(serialize = "685")]
    Vimpeli,
    /// Vähäkyrö.
    #[strum(serialize = "686")]
    Vahakyro,
    /// Vöyri.
    #[strum(serialize = "687")]
    Voyri,
    /// Ylihärmä.
    #[strum(serialize = "688")]
    Yliharma,
    /// Ylistaro.
    #[strum(serialize = "689")]
    Ylistaro,
    /// Ähtäri.
    #[strum(serialize = "690")]
    Ahtari,
    /// Äänekoski.
    #[strum(serialize = "692")]
    Aanekoski,
    /// Eno.
    #[strum(serialize = "701")]
    Eno,
    /// Iisalmi.
    #[strum(serialize = "702")]
    Iisalmi,
    /// Ilomantsi.
    #[strum(serialize = "703")]
    Ilomantsi,
    /// Joensuu.
    #[strum(serialize = "704")]
    Joensuu,
    /// Juankoski.
    #[strum(serialize = "705")]
    Juankoski,
    /// Juuka.
    #[strum(serialize = "706")]
    Juuka,
    /// Kaavi.
    #[strum(serialize = "707")]
    Kaavi,
    /// Karttula.
    #[strum(serialize = "708")]
    Karttula,
    /// Keitele.
    #[strum(serialize = "709")]
    Keitele,
    /// Kesälahti.
    #[strum(serialize = "710")]
    Kesalahti,
    /// Kiihtelysvaara.
    #[strum(serialize = "711")]
    Kiihtelysvaara,
    /// Kitee.
    #[strum(serialize = "712")]
    Kitee,
    /// Kiuruvesi.
    #[strum(serialize = "713")]
    Kiuruvesi,
    /// Kontiolahti.
    #[strum(serialize = "714")]
    Kontiolahti,
    /// Kuopio.
    #[strum(serialize = "715")]
    Kuopio,
    /// Lapinlahti.
    #[strum(serialize = "716")]
    Lapinlahti,
    /// Leppävirta.
    #[strum(serialize = "717")]
    Leppavirta,
    /// Lieksa.
    #[strum(serialize = "718")]
    Lieksa,
    /// Liperi.
    #[strum(serialize = "719")]
    Liperi,
    /// Maaninka.
    #[strum(serialize = "720")]
    Maaninka,
    /// Nilsiä.
    #[strum(serialize = "721")]
    Nilsia,
    /// Nurmes.
    #[strum(serialize = "722")]
    Nurmes,
    /// Outokumpu.
    #[strum(serialize = "723")]
    Outokumpu,
    /// Pielavesi.
    #[strum(serialize = "724")]
    Pielavesi,
    /// Polvijärvi.
    #[strum(serialize = "725")]
    Polvijarvi,
    /// Pyhäselkä.
    #[strum(serialize = "726")]
    Pyhaselka,
    /// Rautalampi.
    #[strum(serialize = "727")]
    Rautalampi,
    /// Rautavaara.
    #[strum(serialize = "728")]
    Rautavaara,
    /// Rääkkylä.
    #[strum(serialize = "729")]
    Raakkyla,
    /// Siilinjärvi.
    #[strum(serialize = "730")]
    Siilinjarvi,
    /// Sonkajärvi.
    #[strum(serialize = "731")]
    Sonkajarvi,
    /// Suonenjoki.
    #[strum(serialize = "732")]
    Suonenjoki,
    /// Tervo.
    #[strum(serialize = "733")]
    Tervo,
    /// Tohmajärvi.
    #[strum(serialize = "734")]
    Tohmajarvi,
    /// Tuupovaara.
    #[strum(serialize = "735")]
    Tuupovaara,
    /// Tuusniemi.
    #[strum(serialize = "736")]
    Tuusniemi,
    /// Valtimo.
    #[strum(serialize = "737")]
    Valtimo,
    /// Varkaus.
    #[strum(serialize = "738")]
    Varkaus,
    /// Varpaisjärvi.
    #[strum(serialize = "739")]
    Varpaisjarvi,
    /// Vehmersalmi.
    #[strum(serialize = "740")]
    Vehmersalmi,
    /// Vesanto.
    #[strum(serialize = "741")]
    Vesanto,
    /// Vieremä.
    #[strum(serialize = "742")]
    Vierema,
    /// Värtsilä.
    #[strum(serialize = "743")]
    Vartsila,
    /// Alavieska.
    #[strum(serialize = "801")]
    Alavieska,
    /// Haapajärvi.
    #[strum(serialize = "802")]
    Haapajarvi,
    /// Haapavesi.
    #[strum(serialize = "803")]
    Haapavesi,
    /// Hailuoto.
    #[strum(serialize = "804")]
    Hailuoto,
    /// Haukipudas.
    #[strum(serialize = "805")]
    Haukipudas,
    /// Hyrynsalmi.
    #[strum(serialize = "806")]
    Hyrynsalmi,
    /// Ii.
    #[strum(serialize = "807")]
    Ii,
    /// Kajaani.
    #[strum(serialize = "808")]
    Kajaani,
    /// Kalajoki.
    #[strum(serialize = "810")]
    Kalajoki,
    /// Kempele.
    #[strum(serialize = "811")]
    Kempele,
    /// Kestilä.
    #[strum(serialize = "812")]
    Kestila,
    /// Kiiminki.
    #[strum(serialize = "813")]
    Kiiminki,
    /// Kuhmo.
    #[strum(serialize = "814")]
    Kuhmo,
    /// Kuivaniemi.
    #[strum(serialize = "815")]
    Kuivaniemi,
    /// Kuusamo.
    #[strum(serialize = "816")]
    Kuusamo,
    /// Kärsämäki.
    #[strum(serialize = "817")]
    Karsamaki,
    /// Liminka.
    #[strum(serialize = "818")]
    Liminka,
    /// Lumijoki.
    #[strum(serialize = "819")]
    Lumijoki,
    /// Merijärvi.
    #[strum(serialize = "820")]
    Merijarvi,
    /// Muhos.
    #[strum(serialize = "821")]
    Muhos,
    /// Nivala.
    #[strum(serialize = "822")]
    Nivala,
    /// Oulainen.
    #[strum(serialize = "823")]
    Oulainen,
    /// Oulu.
    #[strum(serialize = "824")]
    Oulu,
    /// Oulunsalo.
    #[strum(serialize = "825")]
    Oulunsalo,
    /// Paltamo.
    #[strum(serialize = "826")]
    Paltamo,
    /// Pattijoki.
    #[strum(serialize = "827")]
    Pattijoki,
    /// Piippola.
    #[strum(serialize = "828")]
    Piippola,
    /// Pudasjärvi.
    #[strum(serialize = "829")]
    Pudasjarvi,
    /// Pulkkila.
    #[strum(serialize = "830")]
    Pulkkila,
    /// Puolanka.
    #[strum(serialize = "831")]
    Puolanka,
    /// Pyhäjoki.
    #[strum(serialize = "832")]
    Pyhajoki,
    /// Pyhäjärvi.
    #[strum(serialize = "833")]
    Pyhajarvi,
    /// Pyhäntä.
    #[strum(serialize = "834")]
    Pyhanta,
    /// Raahe.
    #[strum(serialize = "835")]
    Raahe,
    /// Rantsila.
    #[strum(serialize = "836")]
    Rantsila,
    /// Reisjärvi.
    #[strum(serialize = "837")]
    Reisjarvi,
    /// Ristijärvi.
    #[strum(serialize = "838")]
    Ristijarvi,
    /// Ruukki.
    #[strum(serialize = "839")]
    Ruukki,
    /// Sievi.
    #[strum(serialize = "840")]
    Sievi,
    /// Siikajoki.
    #[strum(serialize = "841")]
    Siikajoki,
    /// Sotkamo.
    #[strum(serialize = "842")]
    Sotkamo,
    /// Suomussalmi.
    #[strum(serialize = "843")]
    Suomussalmi,
    /// Taivalkoski.
    #[strum(serialize = "844")]
    Taivalkoski,
    /// Tyrnävä.
    #[strum(serialize = "846")]
    Tyrnava,
    /// Utajärvi.
    #[strum(serialize = "847")]
    Utajarvi,
    /// Vaala.
    #[strum(serialize = "848")]
    Vaala,
    /// Vihanti.
    #[strum(serialize = "849")]
    Vihanti,
    /// Vuolijoki.
    #[strum(serialize = "850")]
    Vuolijoki,
    /// Yli-Ii.
    #[strum(serialize = "851")]
    YliIi,
    /// Ylikiiminki.
    #[strum(serialize = "852")]
    Ylikiiminki,
    /// Ylivieska.
    #[strum(serialize = "853")]
    Ylivieska,
    /// Enontekiö.
    #[strum(serialize = "901")]
    Enontekio,
    /// Inari.
    #[strum(serialize = "902")]
    Inari,
    /// Kemi.
    #[strum(serialize = "903")]
    Kemi,
    /// Keminmaa.
    #[strum(serialize = "904")]
    Keminmaa,
    /// Kemijärvi.
    #[strum(serialize = "905")]
    Kemijarvi,
    /// Kittilä.
    #[strum(serialize = "907")]
    Kittila,
    /// Kolari.
    #[strum(serialize = "908")]
    Kolari,
    /// Muonio.
    #[strum(serialize = "909")]
    Muonio,
    /// Pelkosenniemi.
    #[strum(serialize = "910")]
    Pelkosenniemi,
    /// Pello.
    #[strum(serialize = "911")]
    Pello,
    /// Posio.
    #[strum(serialize = "912")]
    Posio,
    /// Ranua.
    #[strum(serialize = "913")]
    Ranua,
    /// Rovaniemi.
    #[strum(serialize = "914")]
    Rovaniemi,
    /// Rovaniemen mlk.
    #[strum(serialize = "915")]
    RovaniemenMlk,
    /// Salla.
    #[strum(serialize = "916")]
    Salla,
    /// Savukoski.
    #[strum(serialize = "917")]
    Savukoski,
    /// Simo.
    #[strum(serialize = "918")]
    Simo,
    /// Sodankylä.
    #[strum(serialize = "919")]
    Sodankyla,
    /// Tervola.
    #[strum(serialize = "920")]
    Tervola,
    /// Tornio.
    #[strum(serialize = "921")]
    Tornio,
    /// Utsjoki.
    #[strum(serialize = "922")]
    Utsjoki,
    /// Ylitornio.
    #[strum(serialize = "923")]
    Ylitornio,
}
