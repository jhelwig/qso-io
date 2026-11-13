use strum::{EnumIter, EnumString, IntoStaticStr};

/// Contest identifiers recommended by ADIF III.B.5 for `CONTEST_ID`.
///
/// Historical identifiers remain distinct from their replacements.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum ContestId {
    /// PODXS Great Pumpkin Sprint.
    #[strum(serialize = "070-160M-SPRINT")]
    Contest070160MSprint,
    /// PODXS Three Day Weekend.
    #[strum(serialize = "070-3-DAY")]
    Contest0703Day,
    /// PODXS 31 Flavors.
    #[strum(serialize = "070-31-FLAVORS")]
    Contest07031Flavors,
    /// PODXS 40m Firecracker Sprint.
    #[strum(serialize = "070-40M-SPRINT")]
    Contest07040MSprint,
    /// PODXS 80m Jay Hudak Memorial Sprint.
    #[strum(serialize = "070-80M-SPRINT")]
    Contest07080MSprint,
    /// PODXS PSKFest.
    #[strum(serialize = "070-PSKFEST")]
    Contest070Pskfest,
    /// PODXS St. Patricks Day.
    #[strum(serialize = "070-ST-PATS-DAY")]
    Contest070StPatsDay,
    /// PODXS Valentine Sprint.
    #[strum(serialize = "070-VALENTINE-SPRINT")]
    Contest070ValentineSprint,
    /// Ten-Meter RTTY Contest (2011 onwards).
    #[strum(serialize = "10-RTTY")]
    Contest10Rtty,
    /// Open Season Ten Meter QSO Party.
    #[strum(serialize = "1010-OPEN-SEASON")]
    Contest1010OpenSeason,
    /// 7th-Area QSO Party.
    #[strum(serialize = "7QP")]
    Contest7Qp,
    /// Alabama QSO Party.
    #[strum(serialize = "AL-QSO-PARTY")]
    AlQsoParty,
    /// JARL All Asian DX Contest (CW).
    #[strum(serialize = "ALL-ASIAN-DX-CW")]
    AllAsianDxCw,
    /// JARL All Asian DX Contest (PHONE).
    #[strum(serialize = "ALL-ASIAN-DX-PHONE")]
    AllAsianDxPhone,
    /// ANARTS WW RTTY.
    #[strum(serialize = "ANARTS-RTTY")]
    AnartsRtty,
    /// Anatolian WW RTTY.
    #[strum(serialize = "ANATOLIAN-RTTY")]
    AnatolianRtty,
    /// Asia - Pacific Sprint.
    #[strum(serialize = "AP-SPRINT")]
    ApSprint,
    /// Arkansas QSO Party.
    #[strum(serialize = "AR-QSO-PARTY")]
    ArQsoParty,
    /// ARI DX Contest.
    #[strum(serialize = "ARI-DX")]
    AriDx,
    /// ARI Italian EME Trophy.
    #[strum(serialize = "ARI-EME")]
    AriEme,
    /// ARI Italian Activity Contest (13cm+).
    #[strum(serialize = "ARI-IAC-13CM")]
    AriIac13Cm,
    /// ARI Italian Activity Contest (23cm).
    #[strum(serialize = "ARI-IAC-23CM")]
    AriIac23Cm,
    /// ARI Italian Activity Contest (6m).
    #[strum(serialize = "ARI-IAC-6M")]
    AriIac6M,
    /// ARI Italian Activity Contest (UHF).
    #[strum(serialize = "ARI-IAC-UHF")]
    AriIacUhf,
    /// ARI Italian Activity Contest (VHF).
    #[strum(serialize = "ARI-IAC-VHF")]
    AriIacVhf,
    /// ARRL 10 Meter Contest.
    #[strum(serialize = "ARRL-10")]
    Arrl10,
    /// ARRL 10 GHz and Up Contest.
    #[strum(serialize = "ARRL-10-GHZ")]
    Arrl10Ghz,
    /// ARRL 160 Meter Contest.
    #[strum(serialize = "ARRL-160")]
    Arrl160,
    /// ARRL 222 MHz and Up Distance Contest.
    #[strum(serialize = "ARRL-222")]
    Arrl222,
    /// ARRL International Digital Contest.
    #[strum(serialize = "ARRL-DIGI")]
    ArrlDigi,
    /// ARRL International DX Contest (CW).
    #[strum(serialize = "ARRL-DX-CW")]
    ArrlDxCw,
    /// ARRL International DX Contest (Phone).
    #[strum(serialize = "ARRL-DX-SSB")]
    ArrlDxSsb,
    /// ARRL EME contest.
    #[strum(serialize = "ARRL-EME")]
    ArrlEme,
    /// ARRL Field Day.
    #[strum(serialize = "ARRL-FIELD-DAY")]
    ArrlFieldDay,
    /// ARRL Rookie Roundup (CW).
    #[strum(serialize = "ARRL-RR-CW")]
    ArrlRrCw,
    /// ARRL Rookie Roundup (RTTY).
    #[strum(serialize = "ARRL-RR-RTTY")]
    ArrlRrRtty,
    /// ARRL Rookie Roundup (Phone).
    #[strum(serialize = "ARRL-RR-SSB")]
    ArrlRrSsb,
    /// ARRL RTTY Round-Up.
    #[strum(serialize = "ARRL-RTTY")]
    ArrlRtty,
    /// ARRL School Club Roundup.
    #[strum(serialize = "ARRL-SCR")]
    ArrlScr,
    /// ARRL November Sweepstakes (CW).
    #[strum(serialize = "ARRL-SS-CW")]
    ArrlSsCw,
    /// ARRL November Sweepstakes (Phone).
    #[strum(serialize = "ARRL-SS-SSB")]
    ArrlSsSsb,
    /// ARRL August UHF Contest.
    #[strum(serialize = "ARRL-UHF-AUG")]
    ArrlUhfAug,
    /// ARRL January VHF Sweepstakes.
    #[strum(serialize = "ARRL-VHF-JAN")]
    ArrlVhfJan,
    /// ARRL June VHF QSO Party.
    #[strum(serialize = "ARRL-VHF-JUN")]
    ArrlVhfJun,
    /// ARRL September VHF QSO Party.
    #[strum(serialize = "ARRL-VHF-SEP")]
    ArrlVhfSep,
    /// Arizona QSO Party.
    #[strum(serialize = "AZ-QSO-PARTY")]
    AzQsoParty,
    /// ORARI Banggai DX Contest.
    #[strum(serialize = "BANGGAI-DX")]
    BanggaiDx,
    /// BARTG Spring RTTY Contest.
    #[strum(serialize = "BARTG-RTTY")]
    BartgRtty,
    /// BARTG Sprint Contest.
    #[strum(serialize = "BARTG-SPRINT")]
    BartgSprint,
    /// British Columbia QSO Party.
    #[strum(serialize = "BC-QSO-PARTY")]
    BcQsoParty,
    /// ORARI Bekasi Merdeka Contest.
    #[strum(serialize = "BEKASI-MERDEKA-CONTEST")]
    BekasiMerdekaContest,
    /// California QSO Party.
    #[strum(serialize = "CA-QSO-PARTY")]
    CaQsoParty,
    /// CIS DX Contest.
    #[strum(serialize = "CIS-DX")]
    CisDx,
    /// Colorado QSO Party.
    #[strum(serialize = "CO-QSO-PARTY")]
    CoQsoParty,
    /// CQ WW 160 Meter DX Contest (CW).
    #[strum(serialize = "CQ-160-CW")]
    Cq160Cw,
    /// CQ WW 160 Meter DX Contest (SSB).
    #[strum(serialize = "CQ-160-SSB")]
    Cq160Ssb,
    /// CQ-M International DX Contest.
    #[strum(serialize = "CQ-M")]
    CqM,
    /// CQ World-Wide VHF Contest.
    #[strum(serialize = "CQ-VHF")]
    CqVhf,
    /// CQ WW WPX Contest (CW).
    #[strum(serialize = "CQ-WPX-CW")]
    CqWpxCw,
    /// CQ/RJ WW RTTY WPX Contest.
    #[strum(serialize = "CQ-WPX-RTTY")]
    CqWpxRtty,
    /// CQ WW WPX Contest (SSB).
    #[strum(serialize = "CQ-WPX-SSB")]
    CqWpxSsb,
    /// CQ WW DX Contest (CW).
    #[strum(serialize = "CQ-WW-CW")]
    CqWwCw,
    /// CQ/RJ WW RTTY DX Contest.
    #[strum(serialize = "CQ-WW-RTTY")]
    CqWwRtty,
    /// CQ WW DX Contest (SSB).
    #[strum(serialize = "CQ-WW-SSB")]
    CqWwSsb,
    /// Connecticut QSO Party.
    #[strum(serialize = "CT-QSO-PARTY")]
    CtQsoParty,
    /// Concurso Verde e Amarelo DX CW Contest.
    #[strum(serialize = "CVA-DX-CW")]
    CvaDxCw,
    /// Concurso Verde e Amarelo DX SSB Contest.
    #[strum(serialize = "CVA-DX-SSB")]
    CvaDxSsb,
    /// CWops CW Open Competition.
    #[strum(serialize = "CWOPS-CW-OPEN")]
    CwopsCwOpen,
    /// CWops Mini-CWT Test.
    #[strum(serialize = "CWOPS-CWT")]
    CwopsCwt,
    /// DARC 10m Contest.
    #[strum(serialize = "DARC-10")]
    Darc10,
    /// DARC CW Trainee Contest.
    #[strum(serialize = "DARC-CWA")]
    DarcCwa,
    /// DARC FT4 Contest.
    #[strum(serialize = "DARC-FT4")]
    DarcFt4,
    /// DARC Hell Contest.
    #[strum(serialize = "DARC-HELL")]
    DarcHell,
    /// DARC Microwave Contest.
    #[strum(serialize = "DARC-MICROWAVE")]
    DarcMicrowave,
    /// DARC Trainee Contest.
    #[strum(serialize = "DARC-TRAINEE")]
    DarcTrainee,
    /// DARC UKW Spring Contest.
    #[strum(serialize = "DARC-UKW-SPRING")]
    DarcUkwSpring,
    /// DARC UKW Summer Contest.
    #[strum(serialize = "DARC-UKW-FIELD-DAY")]
    DarcUkwFieldDay,
    /// DARC VHF-, UHF-, Microwave Contest (May).
    #[strum(serialize = "DARC-VHF-UHF-MICROWAVE")]
    DarcVhfUhfMicrowave,
    /// WAE DX Contest (CW).
    #[strum(serialize = "DARC-WAEDC-CW")]
    DarcWaedcCw,
    /// WAE DX Contest (RTTY).
    #[strum(serialize = "DARC-WAEDC-RTTY")]
    DarcWaedcRtty,
    /// WAE DX Contest (SSB).
    #[strum(serialize = "DARC-WAEDC-SSB")]
    DarcWaedcSsb,
    /// DARC Worked All Germany.
    #[strum(serialize = "DARC-WAG")]
    DarcWag,
    /// Delaware QSO Party.
    #[strum(serialize = "DE-QSO-PARTY")]
    DeQsoParty,
    /// DL-DX RTTY Contest.
    #[strum(serialize = "DL-DX-RTTY")]
    DlDxRtty,
    /// DMC RTTY Contest.
    #[strum(serialize = "DMC-RTTY")]
    DmcRtty,
    /// Concurso Nacional de Telegrafía.
    #[strum(serialize = "EA-CNCW")]
    EaCncw,
    /// Municipios Españoles.
    #[strum(serialize = "EA-DME")]
    EaDme,
    /// His Majesty The King of Spain CW Contest (2022 and later).
    #[strum(serialize = "EA-MAJESTAD-CW")]
    EaMajestadCw,
    /// His Majesty The King of Spain SSB Contest (2022 and later).
    #[strum(serialize = "EA-MAJESTAD-SSB")]
    EaMajestadSsb,
    /// EA PSK63.
    #[strum(serialize = "EA-PSK63")]
    EaPsk63,
    /// Unión de Radioaficionados Españoles RTTY Contest (import-only).
    #[strum(serialize = "EA-RTTY")]
    EaRtty,
    /// Su Majestad El Rey de España - CW (2021 and earlier).
    #[strum(serialize = "EA-SMRE-CW")]
    EaSmreCw,
    /// Su Majestad El Rey de España - SSB (2021 and earlier).
    #[strum(serialize = "EA-SMRE-SSB")]
    EaSmreSsb,
    /// Atlántico V-UHF.
    #[strum(serialize = "EA-VHF-ATLANTIC")]
    EaVhfAtlantic,
    /// Combinado de V-UHF.
    #[strum(serialize = "EA-VHF-COM")]
    EaVhfCom,
    /// Costa del Sol V-UHF.
    #[strum(serialize = "EA-VHF-COSTA-SOL")]
    EaVhfCostaSol,
    /// Nacional VHF.
    #[strum(serialize = "EA-VHF-EA")]
    EaVhfEa,
    /// Segovia EA1RCS V-UHF.
    #[strum(serialize = "EA-VHF-EA1RCS")]
    EaVhfEa1Rcs,
    /// QSL V-UHF & 50MHz.
    #[strum(serialize = "EA-VHF-QSL")]
    EaVhfQsl,
    /// Sant Sadurni V-UHF.
    #[strum(serialize = "EA-VHF-SADURNI")]
    EaVhfSadurni,
    /// Unión de Radioaficionados Españoles RTTY Contest.
    #[strum(serialize = "EA-WW-RTTY")]
    EaWwRtty,
    /// DARC Easter Contest.
    #[strum(serialize = "EASTER")]
    Easter,
    /// PSK63 QSO Party.
    #[strum(serialize = "EPC-PSK63")]
    EpcPsk63,
    /// EU Sprint.
    #[strum(serialize = "EU Sprint")]
    EuSprint,
    /// EU HF Championship.
    #[strum(serialize = "EU-HF")]
    EuHf,
    /// EU PSK DX Contest.
    #[strum(serialize = "EU-PSK-DX")]
    EuPskDx,
    /// European CW Association 160m CW Party.
    #[strum(serialize = "EUCW160M")]
    Eucw160M,
    /// FISTS Fall Sprint.
    #[strum(serialize = "FALL SPRINT")]
    FallSprint,
    /// Florida QSO Party.
    #[strum(serialize = "FL-QSO-PARTY")]
    FlQsoParty,
    /// Georgia QSO Party.
    #[strum(serialize = "GA-QSO-PARTY")]
    GaQsoParty,
    /// Hungarian DX Contest.
    #[strum(serialize = "HA-DX")]
    HaDx,
    /// Helvetia Contest.
    #[strum(serialize = "HELVETIA")]
    Helvetia,
    /// Hawaiian QSO Party.
    #[strum(serialize = "HI-QSO-PARTY")]
    HiQsoParty,
    /// IARC Holyland Contest.
    #[strum(serialize = "HOLYLAND")]
    Holyland,
    /// Iowa QSO Party.
    #[strum(serialize = "IA-QSO-PARTY")]
    IaQsoParty,
    /// DARC IARU Region 1 Field Day.
    #[strum(serialize = "IARU-FIELD-DAY")]
    IaruFieldDay,
    /// IARU HF World Championship.
    #[strum(serialize = "IARU-HF")]
    IaruHf,
    /// ICWC Medium Speed Test.
    #[strum(serialize = "ICWC-MST")]
    IcwcMst,
    /// Idaho QSO Party.
    #[strum(serialize = "ID-QSO-PARTY")]
    IdQsoParty,
    /// Illinois QSO Party.
    #[strum(serialize = "IL QSO Party")]
    IlQsoParty,
    /// Indiana QSO Party.
    #[strum(serialize = "IN-QSO-PARTY")]
    InQsoParty,
    /// JARTS WW RTTY.
    #[strum(serialize = "JARTS-WW-RTTY")]
    JartsWwRtty,
    /// Japan International DX Contest (CW).
    #[strum(serialize = "JIDX-CW")]
    JidxCw,
    /// Japan International DX Contest (SSB).
    #[strum(serialize = "JIDX-SSB")]
    JidxSsb,
    /// Mongolian RTTY DX Contest.
    #[strum(serialize = "JT-DX-RTTY")]
    JtDxRtty,
    /// K1USN Slow Speed Open.
    #[strum(serialize = "K1USN-SSO")]
    K1UsnSso,
    /// K1USN Slow Speed Test.
    #[strum(serialize = "K1USN-SST")]
    K1UsnSst,
    /// Kansas QSO Party.
    #[strum(serialize = "KS-QSO-PARTY")]
    KsQsoParty,
    /// Kentucky QSO Party.
    #[strum(serialize = "KY-QSO-PARTY")]
    KyQsoParty,
    /// Louisiana QSO Party.
    #[strum(serialize = "LA-QSO-PARTY")]
    LaQsoParty,
    /// DRCG Long Distance Contest (RTTY).
    #[strum(serialize = "LDC-RTTY")]
    LdcRtty,
    /// LZ DX Contest.
    #[strum(serialize = "LZ DX")]
    LzDx,
    /// Maritimes QSO Party.
    #[strum(serialize = "MAR-QSO-PARTY")]
    MarQsoParty,
    /// Maryland QSO Party.
    #[strum(serialize = "MD-QSO-PARTY")]
    MdQsoParty,
    /// Maine QSO Party.
    #[strum(serialize = "ME-QSO-PARTY")]
    MeQsoParty,
    /// Michigan QSO Party.
    #[strum(serialize = "MI-QSO-PARTY")]
    MiQsoParty,
    /// Mid-Atlantic QSO Party.
    #[strum(serialize = "MIDATLANTIC-QSO-PARTY")]
    MidatlanticQsoParty,
    /// Minnesota QSO Party.
    #[strum(serialize = "MN-QSO-PARTY")]
    MnQsoParty,
    /// Missouri QSO Party.
    #[strum(serialize = "MO-QSO-PARTY")]
    MoQsoParty,
    /// Mississippi QSO Party.
    #[strum(serialize = "MS-QSO-PARTY")]
    MsQsoParty,
    /// Montana QSO Party.
    #[strum(serialize = "MT-QSO-PARTY")]
    MtQsoParty,
    /// North America Sprint (CW).
    #[strum(serialize = "NA-SPRINT-CW")]
    NaSprintCw,
    /// North America Sprint (RTTY).
    #[strum(serialize = "NA-SPRINT-RTTY")]
    NaSprintRtty,
    /// North America Sprint (Phone).
    #[strum(serialize = "NA-SPRINT-SSB")]
    NaSprintSsb,
    /// North America QSO Party (CW).
    #[strum(serialize = "NAQP-CW")]
    NaqpCw,
    /// North America QSO Party (RTTY).
    #[strum(serialize = "NAQP-RTTY")]
    NaqpRtty,
    /// North America QSO Party (Phone).
    #[strum(serialize = "NAQP-SSB")]
    NaqpSsb,
    /// International Naval Contest (INC).
    #[strum(serialize = "NAVAL")]
    Naval,
    /// North Carolina QSO Party.
    #[strum(serialize = "NC-QSO-PARTY")]
    NcQsoParty,
    /// North Dakota QSO Party.
    #[strum(serialize = "ND-QSO-PARTY")]
    NdQsoParty,
    /// Nebraska QSO Party.
    #[strum(serialize = "NE-QSO-PARTY")]
    NeQsoParty,
    /// New England QSO Party.
    #[strum(serialize = "NEQP")]
    Neqp,
    /// New Hampshire QSO Party.
    #[strum(serialize = "NH-QSO-PARTY")]
    NhQsoParty,
    /// New Jersey QSO Party.
    #[strum(serialize = "NJ-QSO-PARTY")]
    NjQsoParty,
    /// New Mexico QSO Party.
    #[strum(serialize = "NM-QSO-PARTY")]
    NmQsoParty,
    /// NRAU-Baltic Contest (CW).
    #[strum(serialize = "NRAU-BALTIC-CW")]
    NrauBalticCw,
    /// NRAU-Baltic Contest (SSB).
    #[strum(serialize = "NRAU-BALTIC-SSB")]
    NrauBalticSsb,
    /// Nevada QSO Party.
    #[strum(serialize = "NV-QSO-PARTY")]
    NvQsoParty,
    /// New York QSO Party.
    #[strum(serialize = "NY-QSO-PARTY")]
    NyQsoParty,
    /// Oceania DX Contest (CW).
    #[strum(serialize = "OCEANIA-DX-CW")]
    OceaniaDxCw,
    /// Oceania DX Contest (SSB).
    #[strum(serialize = "OCEANIA-DX-SSB")]
    OceaniaDxSsb,
    /// Ohio QSO Party.
    #[strum(serialize = "OH-QSO-PARTY")]
    OhQsoParty,
    /// Czech Radio Club OK DX Contest.
    #[strum(serialize = "OK-DX-RTTY")]
    OkDxRtty,
    /// Czech Radio Club OK-OM DX Contest.
    #[strum(serialize = "OK-OM-DX")]
    OkOmDx,
    /// Oklahoma QSO Party.
    #[strum(serialize = "OK-QSO-PARTY")]
    OkQsoParty,
    /// Old Man International Sideband Society QSO Party.
    #[strum(serialize = "OMISS-QSO-PARTY")]
    OmissQsoParty,
    /// Ontario QSO Party.
    #[strum(serialize = "ON-QSO-PARTY")]
    OnQsoParty,
    /// Oregon QSO Party.
    #[strum(serialize = "OR-QSO-PARTY")]
    OrQsoParty,
    /// ORARI DX Contest.
    #[strum(serialize = "ORARI-DX")]
    OrariDx,
    /// Pennsylvania QSO Party.
    #[strum(serialize = "PA-QSO-PARTY")]
    PaQsoParty,
    /// Dutch PACC Contest.
    #[strum(serialize = "PACC")]
    Pacc,
    /// PCCPro CW Contest.
    #[strum(serialize = "PCC")]
    Pcc,
    /// MDXA PSK DeathMatch (2005-2010).
    #[strum(serialize = "PSK-DEATHMATCH")]
    PskDeathmatch,
    /// Quebec QSO Party.
    #[strum(serialize = "QC-QSO-PARTY")]
    QcQsoParty,
    /// Canadian Amateur Radio Society Contest (import-only).
    #[strum(serialize = "RAC")]
    Rac,
    /// RAC Canada Day Contest.
    #[strum(serialize = "RAC-CANADA-DAY")]
    RacCanadaDay,
    /// RAC Canada Winter Contest.
    #[strum(serialize = "RAC-CANADA-WINTER")]
    RacCanadaWinter,
    /// Russian District Award Contest.
    #[strum(serialize = "RDAC")]
    Rdac,
    /// Russian DX Contest.
    #[strum(serialize = "RDXC")]
    Rdxc,
    /// Reseau des Emetteurs Francais 160m Contest.
    #[strum(serialize = "REF-160M")]
    Ref160M,
    /// Reseau des Emetteurs Francais Contest (CW).
    #[strum(serialize = "REF-CW")]
    RefCw,
    /// Reseau des Emetteurs Francais Contest (SSB).
    #[strum(serialize = "REF-SSB")]
    RefSsb,
    /// Rede dos Emissores Portugueses Portugal Day HF Contest.
    #[strum(serialize = "REP-PORTUGAL-DAY-HF")]
    RepPortugalDayHf,
    /// Rhode Island QSO Party.
    #[strum(serialize = "RI-QSO-PARTY")]
    RiQsoParty,
    /// 1.8MHz Contest.
    #[strum(serialize = "RSGB-160")]
    Rsgb160,
    /// 21/28 MHz Contest (CW).
    #[strum(serialize = "RSGB-21/28-CW")]
    Rsgb2128Cw,
    /// 21/28 MHz Contest (SSB).
    #[strum(serialize = "RSGB-21/28-SSB")]
    Rsgb2128Ssb,
    /// 80m Club Championships.
    #[strum(serialize = "RSGB-80M-CC")]
    Rsgb80MCc,
    /// Affiliated Societies Team Contest (CW).
    #[strum(serialize = "RSGB-AFS-CW")]
    RsgbAfsCw,
    /// Affiliated Societies Team Contest (SSB).
    #[strum(serialize = "RSGB-AFS-SSB")]
    RsgbAfsSsb,
    /// Club Calls.
    #[strum(serialize = "RSGB-CLUB-CALLS")]
    RsgbClubCalls,
    /// Commonwealth Contest.
    #[strum(serialize = "RSGB-COMMONWEALTH")]
    RsgbCommonwealth,
    /// IOTA Contest.
    #[strum(serialize = "RSGB-IOTA")]
    RsgbIota,
    /// Low Power Field Day.
    #[strum(serialize = "RSGB-LOW-POWER")]
    RsgbLowPower,
    /// National Field Day.
    #[strum(serialize = "RSGB-NFD")]
    RsgbNfd,
    /// RoPoCo.
    #[strum(serialize = "RSGB-ROPOCO")]
    RsgbRopoco,
    /// SSB Field Day.
    #[strum(serialize = "RSGB-SSB-FD")]
    RsgbSsbFd,
    /// Russian Radio RTTY Worldwide Contest.
    #[strum(serialize = "RUSSIAN-RTTY")]
    RussianRtty,
    /// Scandinavian Activity Contest (CW).
    #[strum(serialize = "SAC-CW")]
    SacCw,
    /// Scandinavian Activity Contest (SSB).
    #[strum(serialize = "SAC-SSB")]
    SacSsb,
    /// SARTG WW RTTY.
    #[strum(serialize = "SARTG-RTTY")]
    SartgRtty,
    /// South Carolina QSO Party.
    #[strum(serialize = "SC-QSO-PARTY")]
    ScQsoParty,
    /// SCC RTTY Championship.
    #[strum(serialize = "SCC-RTTY")]
    SccRtty,
    /// South Dakota QSO Party.
    #[strum(serialize = "SD-QSO-PARTY")]
    SdQsoParty,
    /// DARC RTTY Short Contest.
    #[strum(serialize = "ShortRY")]
    Shortry,
    /// SKCC Straight Key Sprint.
    #[strum(serialize = "SKCC-SKS")]
    SkccSks,
    /// SKCC Straight Key Sprint Asia.
    #[strum(serialize = "SKCC-SKSA")]
    SkccSksa,
    /// SKCC Straight Key Sprint Europe.
    #[strum(serialize = "SKCC-SKSE")]
    SkccSkse,
    /// SKCC Weekend Sprintathon.
    #[strum(serialize = "SKCC-WES")]
    SkccWes,
    /// SSA Portabeltest.
    #[strum(serialize = "SMP-AUG")]
    SmpAug,
    /// SSA Portabeltest.
    #[strum(serialize = "SMP-MAY")]
    SmpMay,
    /// PRC SPDX Contest (RTTY).
    #[strum(serialize = "SP-DX-RTTY")]
    SpDxRtty,
    /// SPAR Winter Field Day(2016 and earlier).
    #[strum(serialize = "SPAR-WINTER-FD")]
    SparWinterFd,
    /// SP DX Contest.
    #[strum(serialize = "SPDXContest")]
    Spdxcontest,
    /// FISTS Spring Sprint.
    #[strum(serialize = "SPRING SPRINT")]
    SpringSprint,
    /// Scottish-Russian Marathon.
    #[strum(serialize = "SR-MARATHON")]
    SrMarathon,
    /// Stew Perry Topband Distance Challenge.
    #[strum(serialize = "STEW-PERRY")]
    StewPerry,
    /// FISTS Summer Sprint.
    #[strum(serialize = "SUMMER SPRINT")]
    SummerSprint,
    /// TARA Grid Dip PSK-RTTY Shindig.
    #[strum(serialize = "TARA-GRID-DIP")]
    TaraGridDip,
    /// TARA RTTY Mêlée.
    #[strum(serialize = "TARA-RTTY")]
    TaraRtty,
    /// TARA Rumble PSK Contest.
    #[strum(serialize = "TARA-RUMBLE")]
    TaraRumble,
    /// TARA Skirmish Digital Prefix Contest.
    #[strum(serialize = "TARA-SKIRMISH")]
    TaraSkirmish,
    /// Ten-Meter RTTY Contest (before 2011).
    #[strum(serialize = "TEN-RTTY")]
    TenRtty,
    /// The Makrothen Contest.
    #[strum(serialize = "TMC-RTTY")]
    TmcRtty,
    /// Tennessee QSO Party.
    #[strum(serialize = "TN-QSO-PARTY")]
    TnQsoParty,
    /// Texas QSO Party.
    #[strum(serialize = "TX-QSO-PARTY")]
    TxQsoParty,
    /// UBA Contest (CW).
    #[strum(serialize = "UBA-DX-CW")]
    UbaDxCw,
    /// UBA Contest (SSB).
    #[strum(serialize = "UBA-DX-SSB")]
    UbaDxSsb,
    /// European PSK Club BPSK63 Contest.
    #[strum(serialize = "UK-DX-BPSK63")]
    UkDxBpsk63,
    /// UK DX RTTY Contest.
    #[strum(serialize = "UK-DX-RTTY")]
    UkDxRtty,
    /// Open Ukraine RTTY Championship.
    #[strum(serialize = "UKR-CHAMP-RTTY")]
    UkrChampRtty,
    /// Ukrainian DX.
    #[strum(serialize = "UKRAINIAN DX")]
    UkrainianDx,
    /// UKSMG 6m Marathon.
    #[strum(serialize = "UKSMG-6M-MARATHON")]
    Uksmg6MMarathon,
    /// UKSMG Summer Es Contest.
    #[strum(serialize = "UKSMG-SUMMER-ES")]
    UksmgSummerEs,
    /// Ukrainian DX Contest (import-only).
    #[strum(serialize = "URE-DX")]
    UreDx,
    /// Mobile Amateur Awards Club.
    #[strum(serialize = "US-COUNTIES-QSO")]
    UsCountiesQso,
    /// Utah QSO Party.
    #[strum(serialize = "UT-QSO-PARTY")]
    UtQsoParty,
    /// Virginia QSO Party.
    #[strum(serialize = "VA-QSO-PARTY")]
    VaQsoParty,
    /// RCV Venezuelan Independence Day Contest.
    #[strum(serialize = "VENEZ-IND-DAY")]
    VenezIndDay,
    /// Virginia QSO Party (import-only).
    #[strum(serialize = "VIRGINIA QSO PARTY")]
    VirginiaQsoParty,
    /// Alessandro Volta RTTY DX Contest.
    #[strum(serialize = "VOLTA-RTTY")]
    VoltaRtty,
    /// Vermont QSO Party.
    #[strum(serialize = "VT-QSO-PARTY")]
    VtQsoParty,
    /// Washington QSO Party.
    #[strum(serialize = "WA-QSO-PARTY")]
    WaQsoParty,
    /// Winter Field Day (2017 and later).
    #[strum(serialize = "WFD")]
    Wfd,
    /// Wisconsin QSO Party.
    #[strum(serialize = "WI-QSO-PARTY")]
    WiQsoParty,
    /// WIA Harry Angel Memorial 80m Sprint.
    #[strum(serialize = "WIA-HARRY ANGEL")]
    WiaHarryAngel,
    /// WIA John Moyle Memorial Field Day.
    #[strum(serialize = "WIA-JMMFD")]
    WiaJmmfd,
    /// WIA Oceania DX (OCDX) Contest.
    #[strum(serialize = "WIA-OCDX")]
    WiaOcdx,
    /// WIA Remembrance Day.
    #[strum(serialize = "WIA-REMEMBRANCE")]
    WiaRemembrance,
    /// WIA Ross Hull Memorial VHF/UHF Contest.
    #[strum(serialize = "WIA-ROSS HULL")]
    WiaRossHull,
    /// WIA Trans Tasman Low Bands Challenge.
    #[strum(serialize = "WIA-TRANS TASMAN")]
    WiaTransTasman,
    /// WIA VHF UHF Field Days.
    #[strum(serialize = "WIA-VHF/UHF FD")]
    WiaVhfUhfFd,
    /// WIA VK Shires.
    #[strum(serialize = "WIA-VK SHIRES")]
    WiaVkShires,
    /// FISTS Winter Sprint.
    #[strum(serialize = "WINTER SPRINT")]
    WinterSprint,
    /// West Virginia QSO Party.
    #[strum(serialize = "WV-QSO-PARTY")]
    WvQsoParty,
    /// World Wide Digi DX Contest.
    #[strum(serialize = "WW-DIGI")]
    WwDigi,
    /// Wyoming QSO Party.
    #[strum(serialize = "WY-QSO-PARTY")]
    WyQsoParty,
    /// Mexico International Contest (RTTY).
    #[strum(serialize = "XE-INTL-RTTY")]
    XeIntlRtty,
    /// YODX HF contest.
    #[strum(serialize = "YOHFDX")]
    Yohfdx,
    /// YU DX Contest.
    #[strum(serialize = "YUDXC")]
    Yudxc,
}

impl ContestId {
    /// Returns the canonical ADIF contest identifier.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// Reports whether ADIF retains this identifier only for importing old logs.
    pub fn is_import_only(self) -> bool {
        matches!(
            self,
            Self::EaRtty | Self::Rac | Self::UreDx | Self::VirginiaQsoParty
        )
    }
}
