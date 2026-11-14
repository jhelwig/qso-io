use strum::{EnumIter, EnumString, IntoStaticStr};

/// DXCC entity identities from ADIF 3.1.8 section III.B.8.
///
/// Deleted entities remain valid reference identities for historical contacts.
/// [`Self::None`] explicitly means that the station is known to be outside a
/// DXCC entity; an omitted DXCC field does not establish that fact.
///
/// Decimal codes are the ADIF wire values.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum DxccEntity {
    /// None: the contacted station is known to be outside a DXCC entity.
    #[strum(serialize = "0")]
    None,
    /// Canada.
    #[strum(serialize = "1")]
    Canada,
    /// Abu Ail Islands; deleted DXCC entity.
    #[strum(serialize = "2")]
    AbuAilIslands,
    /// Afghanistan.
    #[strum(serialize = "3")]
    Afghanistan,
    /// Agalega and St. Brandon Islands.
    #[strum(serialize = "4")]
    AgalegaAndStBrandonIslands,
    /// Aland Islands.
    #[strum(serialize = "5")]
    AlandIslands,
    /// Alaska.
    #[strum(serialize = "6")]
    Alaska,
    /// Albania.
    #[strum(serialize = "7")]
    Albania,
    /// Aldabra; deleted DXCC entity.
    #[strum(serialize = "8")]
    Aldabra,
    /// American Samoa.
    #[strum(serialize = "9")]
    AmericanSamoa,
    /// Amsterdam and St. Paul Islands.
    #[strum(serialize = "10")]
    AmsterdamAndStPaulIslands,
    /// Andaman and Nicobar Islands.
    #[strum(serialize = "11")]
    AndamanAndNicobarIslands,
    /// Anguilla.
    #[strum(serialize = "12")]
    Anguilla,
    /// Antarctica.
    #[strum(serialize = "13")]
    Antarctica,
    /// Armenia.
    #[strum(serialize = "14")]
    Armenia,
    /// Asiatic Russia.
    #[strum(serialize = "15")]
    AsiaticRussia,
    /// New Zealand Subantarctic Islands.
    #[strum(serialize = "16")]
    NewZealandSubantarcticIslands,
    /// Aves Island.
    #[strum(serialize = "17")]
    AvesIsland,
    /// Azerbaijan.
    #[strum(serialize = "18")]
    Azerbaijan,
    /// Bajo Nuevo; deleted DXCC entity.
    #[strum(serialize = "19")]
    BajoNuevo,
    /// Baker and Howland Islands.
    #[strum(serialize = "20")]
    BakerAndHowlandIslands,
    /// Balearic Islands.
    #[strum(serialize = "21")]
    BalearicIslands,
    /// Palau.
    #[strum(serialize = "22")]
    Palau,
    /// Blenheim Reef; deleted DXCC entity.
    #[strum(serialize = "23")]
    BlenheimReef,
    /// Bouvet.
    #[strum(serialize = "24")]
    Bouvet,
    /// British North Borneo; deleted DXCC entity.
    #[strum(serialize = "25")]
    BritishNorthBorneo,
    /// British Somaliland; deleted DXCC entity.
    #[strum(serialize = "26")]
    BritishSomaliland,
    /// Belarus.
    #[strum(serialize = "27")]
    Belarus,
    /// Canal Zone; deleted DXCC entity.
    #[strum(serialize = "28")]
    CanalZone,
    /// Canary Islands.
    #[strum(serialize = "29")]
    CanaryIslands,
    /// Celebe and Molucca Islands; deleted DXCC entity.
    #[strum(serialize = "30")]
    CelebeAndMoluccaIslands,
    /// C. Kiribati (British Phoenix Islands).
    #[strum(serialize = "31")]
    CentralKiribati,
    /// Ceuta and Melilla.
    #[strum(serialize = "32")]
    CeutaAndMelilla,
    /// Chagos Islands.
    #[strum(serialize = "33")]
    ChagosIslands,
    /// Chatham Islands.
    #[strum(serialize = "34")]
    ChathamIslands,
    /// Christmas Island.
    #[strum(serialize = "35")]
    ChristmasIsland,
    /// Clipperton Island.
    #[strum(serialize = "36")]
    ClippertonIsland,
    /// Cocos Island.
    #[strum(serialize = "37")]
    CocosIsland,
    /// Cocos (Keeling) Islands.
    #[strum(serialize = "38")]
    CocosKeelingIslands,
    /// Comoros; deleted DXCC entity, distinct from code 411.
    #[strum(serialize = "39")]
    DeletedComoros,
    /// Crete.
    #[strum(serialize = "40")]
    Crete,
    /// Crozet Island.
    #[strum(serialize = "41")]
    CrozetIsland,
    /// Damao, Diu; deleted DXCC entity.
    #[strum(serialize = "42")]
    DamaoDiu,
    /// Desecheo Island.
    #[strum(serialize = "43")]
    DesecheoIsland,
    /// Desroches; deleted DXCC entity.
    #[strum(serialize = "44")]
    Desroches,
    /// Dodecanese.
    #[strum(serialize = "45")]
    Dodecanese,
    /// East Malaysia.
    #[strum(serialize = "46")]
    EastMalaysia,
    /// Easter Island.
    #[strum(serialize = "47")]
    EasterIsland,
    /// E. Kiribati (Line Islands).
    #[strum(serialize = "48")]
    EastKiribati,
    /// Equatorial Guinea.
    #[strum(serialize = "49")]
    EquatorialGuinea,
    /// Mexico.
    #[strum(serialize = "50")]
    Mexico,
    /// Eritrea.
    #[strum(serialize = "51")]
    Eritrea,
    /// Estonia.
    #[strum(serialize = "52")]
    Estonia,
    /// Ethiopia.
    #[strum(serialize = "53")]
    Ethiopia,
    /// European Russia.
    #[strum(serialize = "54")]
    EuropeanRussia,
    /// Farquhar; deleted DXCC entity.
    #[strum(serialize = "55")]
    Farquhar,
    /// Fernando de Noronha.
    #[strum(serialize = "56")]
    FernandoDeNoronha,
    /// French Equatorial Africa; deleted DXCC entity.
    #[strum(serialize = "57")]
    FrenchEquatorialAfrica,
    /// French Indo-China; deleted DXCC entity.
    #[strum(serialize = "58")]
    FrenchIndoChina,
    /// French West Africa; deleted DXCC entity.
    #[strum(serialize = "59")]
    FrenchWestAfrica,
    /// Bahamas.
    #[strum(serialize = "60")]
    Bahamas,
    /// Franz Josef Land.
    #[strum(serialize = "61")]
    FranzJosefLand,
    /// Barbados.
    #[strum(serialize = "62")]
    Barbados,
    /// French Guiana.
    #[strum(serialize = "63")]
    FrenchGuiana,
    /// Bermuda.
    #[strum(serialize = "64")]
    Bermuda,
    /// British Virgin Islands.
    #[strum(serialize = "65")]
    BritishVirginIslands,
    /// Belize.
    #[strum(serialize = "66")]
    Belize,
    /// French India; deleted DXCC entity.
    #[strum(serialize = "67")]
    FrenchIndia,
    /// Kuwait/Saudi Arabia Neutral Zone; deleted DXCC entity.
    #[strum(serialize = "68")]
    KuwaitSaudiArabiaNeutralZone,
    /// Cayman Islands.
    #[strum(serialize = "69")]
    CaymanIslands,
    /// Cuba.
    #[strum(serialize = "70")]
    Cuba,
    /// Galapagos Islands.
    #[strum(serialize = "71")]
    GalapagosIslands,
    /// Dominican Republic.
    #[strum(serialize = "72")]
    DominicanRepublic,
    /// El Salvador.
    #[strum(serialize = "74")]
    ElSalvador,
    /// Georgia.
    #[strum(serialize = "75")]
    Georgia,
    /// Guatemala.
    #[strum(serialize = "76")]
    Guatemala,
    /// Grenada.
    #[strum(serialize = "77")]
    Grenada,
    /// Haiti.
    #[strum(serialize = "78")]
    Haiti,
    /// Guadeloupe.
    #[strum(serialize = "79")]
    Guadeloupe,
    /// Honduras.
    #[strum(serialize = "80")]
    Honduras,
    /// Germany; deleted DXCC entity, distinct from code 230.
    #[strum(serialize = "81")]
    DeletedGermany,
    /// Jamaica.
    #[strum(serialize = "82")]
    Jamaica,
    /// Martinique.
    #[strum(serialize = "84")]
    Martinique,
    /// Bonaire, Curacao; deleted DXCC entity.
    #[strum(serialize = "85")]
    BonaireCuracao,
    /// Nicaragua.
    #[strum(serialize = "86")]
    Nicaragua,
    /// Panama.
    #[strum(serialize = "88")]
    Panama,
    /// Turks and Caicos Islands.
    #[strum(serialize = "89")]
    TurksAndCaicosIslands,
    /// Trinidad and Tobago.
    #[strum(serialize = "90")]
    TrinidadAndTobago,
    /// Aruba.
    #[strum(serialize = "91")]
    Aruba,
    /// Geyser Reef; deleted DXCC entity.
    #[strum(serialize = "93")]
    GeyserReef,
    /// Antigua and Barbuda.
    #[strum(serialize = "94")]
    AntiguaAndBarbuda,
    /// Dominica.
    #[strum(serialize = "95")]
    Dominica,
    /// Montserrat.
    #[strum(serialize = "96")]
    Montserrat,
    /// St. Lucia.
    #[strum(serialize = "97")]
    StLucia,
    /// St. Vincent.
    #[strum(serialize = "98")]
    StVincent,
    /// Glorioso Islands.
    #[strum(serialize = "99")]
    GloriosoIslands,
    /// Argentina.
    #[strum(serialize = "100")]
    Argentina,
    /// Goa; deleted DXCC entity.
    #[strum(serialize = "101")]
    Goa,
    /// Gold Coast, Togoland; deleted DXCC entity.
    #[strum(serialize = "102")]
    GoldCoastTogoland,
    /// Guam.
    #[strum(serialize = "103")]
    Guam,
    /// Bolivia.
    #[strum(serialize = "104")]
    Bolivia,
    /// Guantanamo Bay.
    #[strum(serialize = "105")]
    GuantanamoBay,
    /// Guernsey.
    #[strum(serialize = "106")]
    Guernsey,
    /// Guinea.
    #[strum(serialize = "107")]
    Guinea,
    /// Brazil.
    #[strum(serialize = "108")]
    Brazil,
    /// Guinea-Bissau.
    #[strum(serialize = "109")]
    GuineaBissau,
    /// Hawaii.
    #[strum(serialize = "110")]
    Hawaii,
    /// Heard Island.
    #[strum(serialize = "111")]
    HeardIsland,
    /// Chile.
    #[strum(serialize = "112")]
    Chile,
    /// Ifni; deleted DXCC entity.
    #[strum(serialize = "113")]
    Ifni,
    /// Isle of Man.
    #[strum(serialize = "114")]
    IsleOfMan,
    /// Italian Somaliland; deleted DXCC entity.
    #[strum(serialize = "115")]
    ItalianSomaliland,
    /// Colombia.
    #[strum(serialize = "116")]
    Colombia,
    /// ITU headquarters.
    #[strum(serialize = "117")]
    ItuHeadquarters,
    /// Jan Mayen.
    #[strum(serialize = "118")]
    JanMayen,
    /// Java; deleted DXCC entity.
    #[strum(serialize = "119")]
    Java,
    /// Ecuador.
    #[strum(serialize = "120")]
    Ecuador,
    /// Jersey.
    #[strum(serialize = "122")]
    Jersey,
    /// Johnston Island.
    #[strum(serialize = "123")]
    JohnstonIsland,
    /// Juan de Nova, Europa.
    #[strum(serialize = "124")]
    JuanDeNovaEuropa,
    /// Juan Fernandez Islands.
    #[strum(serialize = "125")]
    JuanFernandezIslands,
    /// Kaliningrad.
    #[strum(serialize = "126")]
    Kaliningrad,
    /// Kamaran Islands; deleted DXCC entity.
    #[strum(serialize = "127")]
    KamaranIslands,
    /// Karelo-Finnish Republic; deleted DXCC entity.
    #[strum(serialize = "128")]
    KareloFinnishRepublic,
    /// Guyana.
    #[strum(serialize = "129")]
    Guyana,
    /// Kazakhstan.
    #[strum(serialize = "130")]
    Kazakhstan,
    /// Kerguelen Islands.
    #[strum(serialize = "131")]
    KerguelenIslands,
    /// Paraguay.
    #[strum(serialize = "132")]
    Paraguay,
    /// Kermadec Islands.
    #[strum(serialize = "133")]
    KermadecIslands,
    /// Kingman Reef; deleted DXCC entity.
    #[strum(serialize = "134")]
    KingmanReef,
    /// Kyrgyzstan.
    #[strum(serialize = "135")]
    Kyrgyzstan,
    /// Peru.
    #[strum(serialize = "136")]
    Peru,
    /// Republic of Korea.
    #[strum(serialize = "137")]
    RepublicOfKorea,
    /// Kure Island.
    #[strum(serialize = "138")]
    KureIsland,
    /// Kuria Muria Island; deleted DXCC entity.
    #[strum(serialize = "139")]
    KuriaMuriaIsland,
    /// Suriname.
    #[strum(serialize = "140")]
    Suriname,
    /// Falkland Islands.
    #[strum(serialize = "141")]
    FalklandIslands,
    /// Lakshadweep Islands.
    #[strum(serialize = "142")]
    LakshadweepIslands,
    /// Laos.
    #[strum(serialize = "143")]
    Laos,
    /// Uruguay.
    #[strum(serialize = "144")]
    Uruguay,
    /// Latvia.
    #[strum(serialize = "145")]
    Latvia,
    /// Lithuania.
    #[strum(serialize = "146")]
    Lithuania,
    /// Lord Howe Island.
    #[strum(serialize = "147")]
    LordHoweIsland,
    /// Venezuela.
    #[strum(serialize = "148")]
    Venezuela,
    /// Azores.
    #[strum(serialize = "149")]
    Azores,
    /// Australia.
    #[strum(serialize = "150")]
    Australia,
    /// Malyj Vysotskij Island; deleted DXCC entity.
    #[strum(serialize = "151")]
    MalyjVysotskijIsland,
    /// Macao.
    #[strum(serialize = "152")]
    Macao,
    /// Macquarie Island.
    #[strum(serialize = "153")]
    MacquarieIsland,
    /// Yemen Arab Republic; deleted DXCC entity.
    #[strum(serialize = "154")]
    YemenArabRepublic,
    /// Malaya; deleted DXCC entity.
    #[strum(serialize = "155")]
    Malaya,
    /// Nauru.
    #[strum(serialize = "157")]
    Nauru,
    /// Vanuatu.
    #[strum(serialize = "158")]
    Vanuatu,
    /// Maldives.
    #[strum(serialize = "159")]
    Maldives,
    /// Tonga.
    #[strum(serialize = "160")]
    Tonga,
    /// Malpelo Island.
    #[strum(serialize = "161")]
    MalpeloIsland,
    /// New Caledonia.
    #[strum(serialize = "162")]
    NewCaledonia,
    /// Papua New Guinea.
    #[strum(serialize = "163")]
    PapuaNewGuinea,
    /// Manchuria; deleted DXCC entity.
    #[strum(serialize = "164")]
    Manchuria,
    /// Mauritius.
    #[strum(serialize = "165")]
    Mauritius,
    /// Mariana Islands.
    #[strum(serialize = "166")]
    MarianaIslands,
    /// Market Reef.
    #[strum(serialize = "167")]
    MarketReef,
    /// Marshall Islands.
    #[strum(serialize = "168")]
    MarshallIslands,
    /// Mayotte.
    #[strum(serialize = "169")]
    Mayotte,
    /// New Zealand.
    #[strum(serialize = "170")]
    NewZealand,
    /// Mellish Reef.
    #[strum(serialize = "171")]
    MellishReef,
    /// Pitcairn Island.
    #[strum(serialize = "172")]
    PitcairnIsland,
    /// Micronesia.
    #[strum(serialize = "173")]
    Micronesia,
    /// Midway Island.
    #[strum(serialize = "174")]
    MidwayIsland,
    /// French Polynesia.
    #[strum(serialize = "175")]
    FrenchPolynesia,
    /// Fiji.
    #[strum(serialize = "176")]
    Fiji,
    /// Minami Torishima.
    #[strum(serialize = "177")]
    MinamiTorishima,
    /// Minerva Reef; deleted DXCC entity.
    #[strum(serialize = "178")]
    MinervaReef,
    /// Moldova.
    #[strum(serialize = "179")]
    Moldova,
    /// Mount Athos.
    #[strum(serialize = "180")]
    MountAthos,
    /// Mozambique.
    #[strum(serialize = "181")]
    Mozambique,
    /// Navassa Island.
    #[strum(serialize = "182")]
    NavassaIsland,
    /// Netherlands Borneo; deleted DXCC entity.
    #[strum(serialize = "183")]
    NetherlandsBorneo,
    /// Netherlands New Guinea; deleted DXCC entity.
    #[strum(serialize = "184")]
    NetherlandsNewGuinea,
    /// Solomon Islands.
    #[strum(serialize = "185")]
    SolomonIslands,
    /// Newfoundland, Labrador; deleted DXCC entity.
    #[strum(serialize = "186")]
    NewfoundlandLabrador,
    /// Niger.
    #[strum(serialize = "187")]
    Niger,
    /// Niue.
    #[strum(serialize = "188")]
    Niue,
    /// Norfolk Island.
    #[strum(serialize = "189")]
    NorfolkIsland,
    /// Samoa.
    #[strum(serialize = "190")]
    Samoa,
    /// North Cook Islands.
    #[strum(serialize = "191")]
    NorthCookIslands,
    /// Ogasawara.
    #[strum(serialize = "192")]
    Ogasawara,
    /// Okinawa (Ryukyu Islands); deleted DXCC entity.
    #[strum(serialize = "193")]
    Okinawa,
    /// Okino Tori-Shima; deleted DXCC entity.
    #[strum(serialize = "194")]
    OkinoToriShima,
    /// Annobon Island.
    #[strum(serialize = "195")]
    AnnobonIsland,
    /// Palestine; deleted DXCC entity, distinct from code 510.
    #[strum(serialize = "196")]
    DeletedPalestine,
    /// Palmyra and Jarvis Islands.
    #[strum(serialize = "197")]
    PalmyraAndJarvisIslands,
    /// Papua Territory; deleted DXCC entity.
    #[strum(serialize = "198")]
    PapuaTerritory,
    /// Peter 1 Island.
    #[strum(serialize = "199")]
    PeterOneIsland,
    /// Portuguese Timor; deleted DXCC entity.
    #[strum(serialize = "200")]
    PortugueseTimor,
    /// Prince Edward and Marion Islands.
    #[strum(serialize = "201")]
    PrinceEdwardAndMarionIslands,
    /// Puerto Rico.
    #[strum(serialize = "202")]
    PuertoRico,
    /// Andorra.
    #[strum(serialize = "203")]
    Andorra,
    /// Revillagigedo.
    #[strum(serialize = "204")]
    Revillagigedo,
    /// Ascension Island.
    #[strum(serialize = "205")]
    AscensionIsland,
    /// Austria.
    #[strum(serialize = "206")]
    Austria,
    /// Rodrigues Island.
    #[strum(serialize = "207")]
    RodriguesIsland,
    /// Ruanda-Urundi; deleted DXCC entity.
    #[strum(serialize = "208")]
    RuandaUrundi,
    /// Belgium.
    #[strum(serialize = "209")]
    Belgium,
    /// Saar; deleted DXCC entity.
    #[strum(serialize = "210")]
    Saar,
    /// Sable Island.
    #[strum(serialize = "211")]
    SableIsland,
    /// Bulgaria.
    #[strum(serialize = "212")]
    Bulgaria,
    /// Saint Martin.
    #[strum(serialize = "213")]
    SaintMartin,
    /// Corsica.
    #[strum(serialize = "214")]
    Corsica,
    /// Cyprus.
    #[strum(serialize = "215")]
    Cyprus,
    /// San Andres and Providencia.
    #[strum(serialize = "216")]
    SanAndresAndProvidencia,
    /// San Felix and San Ambrosio.
    #[strum(serialize = "217")]
    SanFelixAndSanAmbrosio,
    /// Czechoslovakia; deleted DXCC entity.
    #[strum(serialize = "218")]
    Czechoslovakia,
    /// Sao Tome and Principe.
    #[strum(serialize = "219")]
    SaoTomeAndPrincipe,
    /// Sarawak; deleted DXCC entity.
    #[strum(serialize = "220")]
    Sarawak,
    /// Denmark.
    #[strum(serialize = "221")]
    Denmark,
    /// Faroe Islands.
    #[strum(serialize = "222")]
    FaroeIslands,
    /// England.
    #[strum(serialize = "223")]
    England,
    /// Finland.
    #[strum(serialize = "224")]
    Finland,
    /// Sardinia.
    #[strum(serialize = "225")]
    Sardinia,
    /// Saudi Arabia/Iraq Neutral Zone; deleted DXCC entity.
    #[strum(serialize = "226")]
    SaudiArabiaIraqNeutralZone,
    /// France.
    #[strum(serialize = "227")]
    France,
    /// Serrana Bank and Roncador Cay; deleted DXCC entity.
    #[strum(serialize = "228")]
    SerranaBankAndRoncadorCay,
    /// German Democratic Republic; deleted DXCC entity.
    #[strum(serialize = "229")]
    GermanDemocraticRepublic,
    /// Federal Republic of Germany.
    #[strum(serialize = "230")]
    FederalRepublicOfGermany,
    /// Sikkim; deleted DXCC entity.
    #[strum(serialize = "231")]
    Sikkim,
    /// Somalia.
    #[strum(serialize = "232")]
    Somalia,
    /// Gibraltar.
    #[strum(serialize = "233")]
    Gibraltar,
    /// South Cook Islands.
    #[strum(serialize = "234")]
    SouthCookIslands,
    /// South Georgia Island.
    #[strum(serialize = "235")]
    SouthGeorgiaIsland,
    /// Greece.
    #[strum(serialize = "236")]
    Greece,
    /// Greenland.
    #[strum(serialize = "237")]
    Greenland,
    /// South Orkney Islands.
    #[strum(serialize = "238")]
    SouthOrkneyIslands,
    /// Hungary.
    #[strum(serialize = "239")]
    Hungary,
    /// South Sandwich Islands.
    #[strum(serialize = "240")]
    SouthSandwichIslands,
    /// South Shetland Islands.
    #[strum(serialize = "241")]
    SouthShetlandIslands,
    /// Iceland.
    #[strum(serialize = "242")]
    Iceland,
    /// People's Democratic Republic of Yemen; deleted DXCC entity.
    #[strum(serialize = "243")]
    PeoplesDemocraticRepublicOfYemen,
    /// Southern Sudan; deleted DXCC entity, distinct from code 521.
    #[strum(serialize = "244")]
    SouthernSudan,
    /// Ireland.
    #[strum(serialize = "245")]
    Ireland,
    /// Sovereign Military Order of Malta.
    #[strum(serialize = "246")]
    SovereignMilitaryOrderOfMalta,
    /// Spratly Islands.
    #[strum(serialize = "247")]
    SpratlyIslands,
    /// Italy.
    #[strum(serialize = "248")]
    Italy,
    /// St. Kitts and Nevis.
    #[strum(serialize = "249")]
    StKittsAndNevis,
    /// St. Helena.
    #[strum(serialize = "250")]
    StHelena,
    /// Liechtenstein.
    #[strum(serialize = "251")]
    Liechtenstein,
    /// St. Paul Island.
    #[strum(serialize = "252")]
    StPaulIsland,
    /// St. Peter and St. Paul Rocks.
    #[strum(serialize = "253")]
    StPeterAndStPaulRocks,
    /// Luxembourg.
    #[strum(serialize = "254")]
    Luxembourg,
    /// St. Maarten, Saba, St. Eustatius; deleted DXCC entity.
    #[strum(serialize = "255")]
    StMaartenSabaStEustatius,
    /// Madeira Islands.
    #[strum(serialize = "256")]
    MadeiraIslands,
    /// Malta.
    #[strum(serialize = "257")]
    Malta,
    /// Sumatra; deleted DXCC entity.
    #[strum(serialize = "258")]
    Sumatra,
    /// Svalbard.
    #[strum(serialize = "259")]
    Svalbard,
    /// Monaco.
    #[strum(serialize = "260")]
    Monaco,
    /// Swan Islands; deleted DXCC entity.
    #[strum(serialize = "261")]
    SwanIslands,
    /// Tajikistan.
    #[strum(serialize = "262")]
    Tajikistan,
    /// Netherlands.
    #[strum(serialize = "263")]
    Netherlands,
    /// Tangier; deleted DXCC entity.
    #[strum(serialize = "264")]
    Tangier,
    /// Northern Ireland.
    #[strum(serialize = "265")]
    NorthernIreland,
    /// Norway.
    #[strum(serialize = "266")]
    Norway,
    /// Territory of New Guinea; deleted DXCC entity.
    #[strum(serialize = "267")]
    TerritoryOfNewGuinea,
    /// Tibet; deleted DXCC entity.
    #[strum(serialize = "268")]
    Tibet,
    /// Poland.
    #[strum(serialize = "269")]
    Poland,
    /// Tokelau Islands.
    #[strum(serialize = "270")]
    TokelauIslands,
    /// Trieste; deleted DXCC entity.
    #[strum(serialize = "271")]
    Trieste,
    /// Portugal.
    #[strum(serialize = "272")]
    Portugal,
    /// Trindade and Martim Vaz Islands.
    #[strum(serialize = "273")]
    TrindadeAndMartimVazIslands,
    /// Tristan da Cunha and Gough Island.
    #[strum(serialize = "274")]
    TristanDaCunhaAndGoughIsland,
    /// Romania.
    #[strum(serialize = "275")]
    Romania,
    /// Tromelin Island.
    #[strum(serialize = "276")]
    TromelinIsland,
    /// St. Pierre and Miquelon.
    #[strum(serialize = "277")]
    StPierreAndMiquelon,
    /// San Marino.
    #[strum(serialize = "278")]
    SanMarino,
    /// Scotland.
    #[strum(serialize = "279")]
    Scotland,
    /// Turkmenistan.
    #[strum(serialize = "280")]
    Turkmenistan,
    /// Spain.
    #[strum(serialize = "281")]
    Spain,
    /// Tuvalu.
    #[strum(serialize = "282")]
    Tuvalu,
    /// UK Sovereign Base Areas on Cyprus.
    #[strum(serialize = "283")]
    UkSovereignBaseAreasOnCyprus,
    /// Sweden.
    #[strum(serialize = "284")]
    Sweden,
    /// Virgin Islands.
    #[strum(serialize = "285")]
    VirginIslands,
    /// Uganda.
    #[strum(serialize = "286")]
    Uganda,
    /// Switzerland.
    #[strum(serialize = "287")]
    Switzerland,
    /// Ukraine.
    #[strum(serialize = "288")]
    Ukraine,
    /// United Nations headquarters.
    #[strum(serialize = "289")]
    UnitedNationsHeadquarters,
    /// United States of America.
    #[strum(serialize = "291")]
    UnitedStates,
    /// Uzbekistan.
    #[strum(serialize = "292")]
    Uzbekistan,
    /// Viet Nam.
    #[strum(serialize = "293")]
    VietNam,
    /// Wales.
    #[strum(serialize = "294")]
    Wales,
    /// Vatican.
    #[strum(serialize = "295")]
    Vatican,
    /// Serbia.
    #[strum(serialize = "296")]
    Serbia,
    /// Wake Island.
    #[strum(serialize = "297")]
    WakeIsland,
    /// Wallis and Futuna Islands.
    #[strum(serialize = "298")]
    WallisAndFutunaIslands,
    /// West Malaysia.
    #[strum(serialize = "299")]
    WestMalaysia,
    /// W. Kiribati (Gilbert Islands).
    #[strum(serialize = "301")]
    WestKiribati,
    /// Western Sahara.
    #[strum(serialize = "302")]
    WesternSahara,
    /// Willis Island.
    #[strum(serialize = "303")]
    WillisIsland,
    /// Bahrain.
    #[strum(serialize = "304")]
    Bahrain,
    /// Bangladesh.
    #[strum(serialize = "305")]
    Bangladesh,
    /// Bhutan.
    #[strum(serialize = "306")]
    Bhutan,
    /// Zanzibar; deleted DXCC entity.
    #[strum(serialize = "307")]
    Zanzibar,
    /// Costa Rica.
    #[strum(serialize = "308")]
    CostaRica,
    /// Myanmar.
    #[strum(serialize = "309")]
    Myanmar,
    /// Cambodia.
    #[strum(serialize = "312")]
    Cambodia,
    /// Sri Lanka.
    #[strum(serialize = "315")]
    SriLanka,
    /// China.
    #[strum(serialize = "318")]
    China,
    /// Hong Kong.
    #[strum(serialize = "321")]
    HongKong,
    /// India.
    #[strum(serialize = "324")]
    India,
    /// Indonesia.
    #[strum(serialize = "327")]
    Indonesia,
    /// Iran.
    #[strum(serialize = "330")]
    Iran,
    /// Iraq.
    #[strum(serialize = "333")]
    Iraq,
    /// Israel.
    #[strum(serialize = "336")]
    Israel,
    /// Japan.
    #[strum(serialize = "339")]
    Japan,
    /// Jordan.
    #[strum(serialize = "342")]
    Jordan,
    /// Democratic People's Republic of Korea.
    #[strum(serialize = "344")]
    DemocraticPeoplesRepublicOfKorea,
    /// Brunei Darussalam.
    #[strum(serialize = "345")]
    BruneiDarussalam,
    /// Kuwait.
    #[strum(serialize = "348")]
    Kuwait,
    /// Lebanon.
    #[strum(serialize = "354")]
    Lebanon,
    /// Mongolia.
    #[strum(serialize = "363")]
    Mongolia,
    /// Nepal.
    #[strum(serialize = "369")]
    Nepal,
    /// Oman.
    #[strum(serialize = "370")]
    Oman,
    /// Pakistan.
    #[strum(serialize = "372")]
    Pakistan,
    /// Philippines.
    #[strum(serialize = "375")]
    Philippines,
    /// Qatar.
    #[strum(serialize = "376")]
    Qatar,
    /// Saudi Arabia.
    #[strum(serialize = "378")]
    SaudiArabia,
    /// Seychelles.
    #[strum(serialize = "379")]
    Seychelles,
    /// Singapore.
    #[strum(serialize = "381")]
    Singapore,
    /// Djibouti.
    #[strum(serialize = "382")]
    Djibouti,
    /// Syria.
    #[strum(serialize = "384")]
    Syria,
    /// Taiwan.
    #[strum(serialize = "386")]
    Taiwan,
    /// Thailand.
    #[strum(serialize = "387")]
    Thailand,
    /// Turkey.
    #[strum(serialize = "390")]
    Turkey,
    /// United Arab Emirates.
    #[strum(serialize = "391")]
    UnitedArabEmirates,
    /// Algeria.
    #[strum(serialize = "400")]
    Algeria,
    /// Angola.
    #[strum(serialize = "401")]
    Angola,
    /// Botswana.
    #[strum(serialize = "402")]
    Botswana,
    /// Burundi.
    #[strum(serialize = "404")]
    Burundi,
    /// Cameroon.
    #[strum(serialize = "406")]
    Cameroon,
    /// Central Africa.
    #[strum(serialize = "408")]
    CentralAfrica,
    /// Cape Verde.
    #[strum(serialize = "409")]
    CapeVerde,
    /// Chad.
    #[strum(serialize = "410")]
    Chad,
    /// Comoros.
    #[strum(serialize = "411")]
    Comoros,
    /// Republic of the Congo.
    #[strum(serialize = "412")]
    RepublicOfTheCongo,
    /// Democratic Republic of the Congo.
    #[strum(serialize = "414")]
    DemocraticRepublicOfTheCongo,
    /// Benin.
    #[strum(serialize = "416")]
    Benin,
    /// Gabon.
    #[strum(serialize = "420")]
    Gabon,
    /// The Gambia.
    #[strum(serialize = "422")]
    TheGambia,
    /// Ghana.
    #[strum(serialize = "424")]
    Ghana,
    /// Cote d'Ivoire.
    #[strum(serialize = "428")]
    CoteDIvoire,
    /// Kenya.
    #[strum(serialize = "430")]
    Kenya,
    /// Lesotho.
    #[strum(serialize = "432")]
    Lesotho,
    /// Liberia.
    #[strum(serialize = "434")]
    Liberia,
    /// Libya.
    #[strum(serialize = "436")]
    Libya,
    /// Madagascar.
    #[strum(serialize = "438")]
    Madagascar,
    /// Malawi.
    #[strum(serialize = "440")]
    Malawi,
    /// Mali.
    #[strum(serialize = "442")]
    Mali,
    /// Mauritania.
    #[strum(serialize = "444")]
    Mauritania,
    /// Morocco.
    #[strum(serialize = "446")]
    Morocco,
    /// Nigeria.
    #[strum(serialize = "450")]
    Nigeria,
    /// Zimbabwe.
    #[strum(serialize = "452")]
    Zimbabwe,
    /// Reunion Island.
    #[strum(serialize = "453")]
    ReunionIsland,
    /// Rwanda.
    #[strum(serialize = "454")]
    Rwanda,
    /// Senegal.
    #[strum(serialize = "456")]
    Senegal,
    /// Sierra Leone.
    #[strum(serialize = "458")]
    SierraLeone,
    /// Rotuma Island.
    #[strum(serialize = "460")]
    RotumaIsland,
    /// Republic of South Africa.
    #[strum(serialize = "462")]
    RepublicOfSouthAfrica,
    /// Namibia.
    #[strum(serialize = "464")]
    Namibia,
    /// Sudan.
    #[strum(serialize = "466")]
    Sudan,
    /// Kingdom of Eswatini.
    #[strum(serialize = "468")]
    KingdomOfEswatini,
    /// Tanzania.
    #[strum(serialize = "470")]
    Tanzania,
    /// Tunisia.
    #[strum(serialize = "474")]
    Tunisia,
    /// Egypt.
    #[strum(serialize = "478")]
    Egypt,
    /// Burkina Faso.
    #[strum(serialize = "480")]
    BurkinaFaso,
    /// Zambia.
    #[strum(serialize = "482")]
    Zambia,
    /// Togo.
    #[strum(serialize = "483")]
    Togo,
    /// Walvis Bay; deleted DXCC entity.
    #[strum(serialize = "488")]
    WalvisBay,
    /// Conway Reef.
    #[strum(serialize = "489")]
    ConwayReef,
    /// Banaba Island (Ocean Island).
    #[strum(serialize = "490")]
    BanabaIsland,
    /// Yemen.
    #[strum(serialize = "492")]
    Yemen,
    /// Penguin Islands; deleted DXCC entity.
    #[strum(serialize = "493")]
    PenguinIslands,
    /// Croatia.
    #[strum(serialize = "497")]
    Croatia,
    /// Slovenia.
    #[strum(serialize = "499")]
    Slovenia,
    /// Bosnia-Herzegovina.
    #[strum(serialize = "501")]
    BosniaHerzegovina,
    /// North Macedonia (Republic of).
    #[strum(serialize = "502")]
    NorthMacedonia,
    /// Czech Republic.
    #[strum(serialize = "503")]
    CzechRepublic,
    /// Slovak Republic.
    #[strum(serialize = "504")]
    SlovakRepublic,
    /// Pratas Island.
    #[strum(serialize = "505")]
    PratasIsland,
    /// Scarborough Reef.
    #[strum(serialize = "506")]
    ScarboroughReef,
    /// Temotu Province.
    #[strum(serialize = "507")]
    TemotuProvince,
    /// Austral Island.
    #[strum(serialize = "508")]
    AustralIsland,
    /// Marquesas Islands.
    #[strum(serialize = "509")]
    MarquesasIslands,
    /// Palestine.
    #[strum(serialize = "510")]
    Palestine,
    /// Timor-Leste.
    #[strum(serialize = "511")]
    TimorLeste,
    /// Chesterfield Islands.
    #[strum(serialize = "512")]
    ChesterfieldIslands,
    /// Ducie Island.
    #[strum(serialize = "513")]
    DucieIsland,
    /// Montenegro.
    #[strum(serialize = "514")]
    Montenegro,
    /// Swains Island.
    #[strum(serialize = "515")]
    SwainsIsland,
    /// Saint Barthelemy.
    #[strum(serialize = "516")]
    SaintBarthelemy,
    /// Curacao.
    #[strum(serialize = "517")]
    Curacao,
    /// Sint Maarten.
    #[strum(serialize = "518")]
    SintMaarten,
    /// Saba and St. Eustatius.
    #[strum(serialize = "519")]
    SabaAndStEustatius,
    /// Bonaire.
    #[strum(serialize = "520")]
    Bonaire,
    /// South Sudan (Republic of).
    #[strum(serialize = "521")]
    SouthSudan,
    /// Republic of Kosovo.
    #[strum(serialize = "522")]
    RepublicOfKosovo,
}

impl DxccEntity {
    /// Returns this entity's canonical decimal ADIF wire code.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}
