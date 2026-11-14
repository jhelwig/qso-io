//! Standard header and QSO fields from ADIF 3.1.8, sections III.C.1.a/b.
//!
//! The maintained definitions below describe field identity and declared data
//! types. Individual field constraints, import-only values, and dependencies on
//! other fields remain part of the [ADIF specification](https://www.adif.org.uk/318/ADIF_318.htm#ADIF_defined_Fields).
//! Parameterized `USERDEF` and application fields have separate grammars and are
//! not fixed standard field names.

use strum::{EnumIter, EnumString, IntoStaticStr};

use super::{DataType, Enumeration};

/// The portion of an ADIF document in which a standard field is defined.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FieldSection {
    /// File metadata before the end-of-header marker.
    Header,
    /// QSO data within a record.
    Record,
}

/// A standard ADIF header or QSO field name (sections III.C.1.a/b).
///
/// International text fields are defined for ADX only.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum StandardField {
    /// ADIF version as epoch, major, and minor components separated by periods.
    #[strum(serialize = "ADIF_VER")]
    AdifVer,
    /// File creation date and time in UTC, formatted `YYYYMMDD HHMMSS`.
    #[strum(serialize = "CREATED_TIMESTAMP")]
    CreatedTimestamp,
    /// Name of the logger, converter, or utility that created or processed the file.
    #[strum(serialize = "PROGRAMID")]
    ProgramId,
    /// Version of the program that created or processed the file.
    #[strum(serialize = "PROGRAMVERSION")]
    ProgramVersion,
    /// Complete postal address of the contacted station, including its name.
    #[strum(serialize = "ADDRESS")]
    Address,
    /// International-text postal address of the contacted station.
    #[strum(serialize = "ADDRESS_INTL")]
    AddressIntl,
    /// Contacted operator's age in years, from 0 through 120.
    #[strum(serialize = "AGE")]
    Age,
    /// Contacted station's height above mean sea level in meters.
    #[strum(serialize = "ALTITUDE")]
    Altitude,
    /// Logging antenna's azimuth in clockwise degrees from true north, 0 through 360.
    /// Out-of-range imports must be normalized before export.
    #[strum(serialize = "ANT_AZ")]
    AntAz,
    /// Logging antenna's elevation in degrees above the horizon, -90 through 90.
    /// Out-of-range imports must be normalized before export.
    #[strum(serialize = "ANT_EL")]
    AntEl,
    /// Signal path used for the contact, such as short path or long path.
    #[strum(serialize = "ANT_PATH")]
    AntPath,
    /// ARRL section of the contacted station.
    #[strum(serialize = "ARRL_SECT")]
    ArrlSect,
    /// Sponsored awards submitted; this can describe an award account outside a QSO.
    #[strum(serialize = "AWARD_SUBMITTED")]
    AwardSubmitted,
    /// Sponsored awards granted; this can describe an award account outside a QSO.
    #[strum(serialize = "AWARD_GRANTED")]
    AwardGranted,
    /// Geomagnetic A index at contact time, from 0 through 400.
    #[strum(serialize = "A_INDEX")]
    AIndex,
    /// Amateur band on which the contact took place.
    #[strum(serialize = "BAND")]
    Band,
    /// Logging station's receive band for a split-frequency contact.
    #[strum(serialize = "BAND_RX")]
    BandRx,
    /// Callsign of the contacted station.
    #[strum(serialize = "CALL")]
    Call,
    /// Contest check, for example the check exchanged in ARRL Sweepstakes.
    #[strum(serialize = "CHECK")]
    Check,
    /// Contest class, for example the class exchanged in ARRL Field Day.
    #[strum(serialize = "CLASS")]
    Class,
    /// Date of the most recent upload of this contact to Club Log.
    #[strum(serialize = "CLUBLOG_QSO_UPLOAD_DATE")]
    ClublogQsoUploadDate,
    /// Upload status of this contact on Club Log.
    #[strum(serialize = "CLUBLOG_QSO_UPLOAD_STATUS")]
    ClublogQsoUploadStatus,
    /// Contacted station's secondary subdivision, interpreted using its DXCC entity.
    #[strum(serialize = "CNTY")]
    Cnty,
    /// Unordered secondary subdivision alternative codes for the contacted station.
    #[strum(serialize = "CNTY_ALT")]
    CntyAlt,
    /// Contact comment intended to interest the contacted operator; QSL text uses QSLMSG.
    #[strum(serialize = "COMMENT")]
    Comment,
    /// International-text contact comment, distinct from a QSL message.
    #[strum(serialize = "COMMENT_INTL")]
    CommentIntl,
    /// Continent in which the contacted station is located.
    #[strum(serialize = "CONT")]
    Cont,
    /// Callsign of the person operating the contacted station.
    #[strum(serialize = "CONTACTED_OP")]
    ContactedOp,
    /// Contest identifier; the recommended enumeration improves interoperability.
    #[strum(serialize = "CONTEST_ID")]
    ContestId,
    /// Name of the contacted station's DXCC entity.
    #[strum(serialize = "COUNTRY")]
    Country,
    /// International-text name of the contacted station's DXCC entity.
    #[strum(serialize = "COUNTRY_INTL")]
    CountryIntl,
    /// Contacted station's CQ zone, from 1 through 40.
    #[strum(serialize = "CQZ")]
    Cqz,
    /// Credits requested for this contact; legacy AwardList values are import-only.
    #[strum(serialize = "CREDIT_SUBMITTED")]
    CreditSubmitted,
    /// Credits granted for this contact; legacy AwardList values are import-only.
    #[strum(serialize = "CREDIT_GRANTED")]
    CreditGranted,
    /// Contacted station's DARC district code, from DARC's DOK or special DOK lists.
    #[strum(serialize = "DARC_DOK")]
    DarcDok,
    /// Date a DCL confirmation was received, conditional on its received status.
    #[strum(serialize = "DCL_QSLRDATE")]
    DclQslrdate,
    /// Date a confirmation was sent to DCL, conditional on its sent status.
    #[strum(serialize = "DCL_QSLSDATE")]
    DclQslsdate,
    /// DCL confirmation received status; the default is N.
    #[strum(serialize = "DCL_QSL_RCVD")]
    DclQslRcvd,
    /// DCL confirmation sent status; the default is N.
    #[strum(serialize = "DCL_QSL_SENT")]
    DclQslSent,
    /// Nonnegative path distance between the stations, in kilometers.
    #[strum(serialize = "DISTANCE")]
    Distance,
    /// Contacted station's DXCC entity code; zero explicitly means no DXCC entity.
    #[strum(serialize = "DXCC")]
    Dxcc,
    /// Email address of the contacted station.
    #[strum(serialize = "EMAIL")]
    Email,
    /// Callsign of the owner of the contacted station.
    #[strum(serialize = "EQ_CALL")]
    EqCall,
    /// Whether eQSL Authenticity Guaranteed status is known; the default is U.
    #[strum(serialize = "EQSL_AG")]
    EqslAg,
    /// Date an eQSL.cc confirmation was received, conditional on its received status.
    #[strum(serialize = "EQSL_QSLRDATE")]
    EqslQslrdate,
    /// Date a confirmation was sent to eQSL.cc, conditional on its sent status.
    #[strum(serialize = "EQSL_QSLSDATE")]
    EqslQslsdate,
    /// eQSL.cc confirmation received status; verified V is import-only.
    #[strum(serialize = "EQSL_QSL_RCVD")]
    EqslQslRcvd,
    /// eQSL.cc confirmation sent status; the default is N.
    #[strum(serialize = "EQSL_QSL_SENT")]
    EqslQslSent,
    /// Positive FISTS CW Club membership number of the contacted station.
    #[strum(serialize = "FISTS")]
    Fists,
    /// Positive FISTS Century Certificate number of the contacted station.
    #[strum(serialize = "FISTS_CC")]
    FistsCc,
    /// Whether this is a new Earth-Moon-Earth initial contact.
    #[strum(serialize = "FORCE_INIT")]
    ForceInit,
    /// Contact frequency in megahertz.
    #[strum(serialize = "FREQ")]
    Freq,
    /// Logging station's receive frequency in megahertz for a split contact.
    #[strum(serialize = "FREQ_RX")]
    FreqRx,
    /// Contacted station's Maidenhead locator, using 2, 4, 6, or 8 characters.
    /// Longer locators use the extension field for the remaining characters.
    #[strum(serialize = "GRIDSQUARE")]
    Gridsquare,
    /// Characters 9 through 10 or 12 of the contacted station's Maidenhead locator.
    /// Export uses 2 or 4 characters; imports ignore characters beyond the fourth.
    #[strum(serialize = "GRIDSQUARE_EXT")]
    GridsquareExt,
    /// Legacy logging operator callsign; import-only, superseded by OPERATOR.
    #[strum(serialize = "GUEST_OP")]
    GuestOp,
    /// Date of the most recent upload of this contact to HAMLOG.EU.
    #[strum(serialize = "HAMLOGEU_QSO_UPLOAD_DATE")]
    HamlogeuQsoUploadDate,
    /// Upload status of this contact on HAMLOG.EU.
    #[strum(serialize = "HAMLOGEU_QSO_UPLOAD_STATUS")]
    HamlogeuQsoUploadStatus,
    /// Date of the most recent upload of this contact to HamQTH.com.
    #[strum(serialize = "HAMQTH_QSO_UPLOAD_DATE")]
    HamqthQsoUploadDate,
    /// Upload status of this contact on HamQTH.com.
    #[strum(serialize = "HAMQTH_QSO_UPLOAD_STATUS")]
    HamqthQsoUploadStatus,
    /// Date of the most recent upload of this contact to HRDLog.net.
    #[strum(serialize = "HRDLOG_QSO_UPLOAD_DATE")]
    HrdlogQsoUploadDate,
    /// Upload status of this contact on HRDLog.net.
    #[strum(serialize = "HRDLOG_QSO_UPLOAD_STATUS")]
    HrdlogQsoUploadStatus,
    /// Contacted station's IOTA group reference: continent and three-digit group number.
    #[strum(serialize = "IOTA")]
    Iota,
    /// Contacted station's IOTA island identifier, from 1 through 99999999.
    #[strum(serialize = "IOTA_ISLAND_ID")]
    IotaIslandId,
    /// Contacted station's ITU zone, from 1 through 90.
    #[strum(serialize = "ITUZ")]
    Ituz,
    /// Geomagnetic K index at contact time, from 0 through 9.
    #[strum(serialize = "K_INDEX")]
    KIndex,
    /// Latitude of the contacted station.
    #[strum(serialize = "LAT")]
    Lat,
    /// Longitude of the contacted station.
    #[strum(serialize = "LON")]
    Lon,
    /// Date a Logbook of the World confirmation was received, subject to its status.
    #[strum(serialize = "LOTW_QSLRDATE")]
    LotwQslrdate,
    /// Date a confirmation was sent to Logbook of the World, subject to its status.
    #[strum(serialize = "LOTW_QSLSDATE")]
    LotwQslsdate,
    /// Logbook of the World confirmation received status; verified V is import-only.
    #[strum(serialize = "LOTW_QSL_RCVD")]
    LotwQslRcvd,
    /// Logbook of the World confirmation sent status; the default is N.
    #[strum(serialize = "LOTW_QSL_SENT")]
    LotwQslSent,
    /// Longest meteor-scatter burst heard by the logging station, in nonnegative seconds.
    #[strum(serialize = "MAX_BURSTS")]
    MaxBursts,
    /// Operating mode of the contact.
    #[strum(serialize = "MODE")]
    Mode,
    /// Description of the contacted station's Morse key, such as its make and model.
    #[strum(serialize = "MORSE_KEY_INFO")]
    MorseKeyInfo,
    /// Kind of Morse key used by the contacted station.
    #[strum(serialize = "MORSE_KEY_TYPE")]
    MorseKeyType,
    /// Meteor shower active during a meteor-scatter contact.
    #[strum(serialize = "MS_SHOWER")]
    MsShower,
    /// Logging station's height above mean sea level in meters.
    #[strum(serialize = "MY_ALTITUDE")]
    MyAltitude,
    /// Description of the logging station's antenna.
    #[strum(serialize = "MY_ANTENNA")]
    MyAntenna,
    /// International-text description of the logging station's antenna.
    #[strum(serialize = "MY_ANTENNA_INTL")]
    MyAntennaIntl,
    /// ARRL section of the logging station.
    #[strum(serialize = "MY_ARRL_SECT")]
    MyArrlSect,
    /// City of the logging station.
    #[strum(serialize = "MY_CITY")]
    MyCity,
    /// International-text city of the logging station.
    #[strum(serialize = "MY_CITY_INTL")]
    MyCityIntl,
    /// Logging station's secondary subdivision, interpreted using its DXCC entity.
    #[strum(serialize = "MY_CNTY")]
    MyCnty,
    /// Unordered secondary subdivision alternative codes for the logging station.
    #[strum(serialize = "MY_CNTY_ALT")]
    MyCntyAlt,
    /// Name of the logging station's DXCC entity.
    #[strum(serialize = "MY_COUNTRY")]
    MyCountry,
    /// International-text name of the logging station's DXCC entity.
    #[strum(serialize = "MY_COUNTRY_INTL")]
    MyCountryIntl,
    /// Logging station's CQ zone, from 1 through 40.
    #[strum(serialize = "MY_CQ_ZONE")]
    MyCqZone,
    /// Logging station's DARC district code, from DARC's DOK or special DOK lists.
    #[strum(serialize = "MY_DARC_DOK")]
    MyDarcDok,
    /// Logging station's DXCC entity code; zero explicitly means no DXCC entity.
    #[strum(serialize = "MY_DXCC")]
    MyDxcc,
    /// Positive FISTS CW Club membership number of the logging station.
    #[strum(serialize = "MY_FISTS")]
    MyFists,
    /// Logging station's Maidenhead locator, using 2, 4, 6, or 8 characters.
    #[strum(serialize = "MY_GRIDSQUARE")]
    MyGridsquare,
    /// Characters 9 through 10 or 12 of the logging station's Maidenhead locator.
    /// Export uses 2 or 4 characters; imports ignore characters beyond the fourth.
    #[strum(serialize = "MY_GRIDSQUARE_EXT")]
    MyGridsquareExt,
    /// Logging station's IOTA group reference.
    #[strum(serialize = "MY_IOTA")]
    MyIota,
    /// Logging station's IOTA island identifier, from 1 through 99999999.
    #[strum(serialize = "MY_IOTA_ISLAND_ID")]
    MyIotaIslandId,
    /// Logging station's ITU zone, from 1 through 90.
    #[strum(serialize = "MY_ITU_ZONE")]
    MyItuZone,
    /// Latitude of the logging station.
    #[strum(serialize = "MY_LAT")]
    MyLat,
    /// Longitude of the logging station.
    #[strum(serialize = "MY_LON")]
    MyLon,
    /// Description of the logging station's Morse key, such as its make and model.
    #[strum(serialize = "MY_MORSE_KEY_INFO")]
    MyMorseKeyInfo,
    /// Kind of Morse key used by the logging station.
    #[strum(serialize = "MY_MORSE_KEY_TYPE")]
    MyMorseKeyType,
    /// Name of the logging operator.
    #[strum(serialize = "MY_NAME")]
    MyName,
    /// International-text name of the logging operator.
    #[strum(serialize = "MY_NAME_INTL")]
    MyNameIntl,
    /// Postal code of the logging station.
    #[strum(serialize = "MY_POSTAL_CODE")]
    MyPostalCode,
    /// International-text postal code of the logging station.
    #[strum(serialize = "MY_POSTAL_CODE_INTL")]
    MyPostalCodeIntl,
    /// One or more Parks on the Air references for the logging station.
    #[strum(serialize = "MY_POTA_REF")]
    MyPotaRef,
    /// Description of the logging station's equipment.
    #[strum(serialize = "MY_RIG")]
    MyRig,
    /// International-text description of the logging station's equipment.
    #[strum(serialize = "MY_RIG_INTL")]
    MyRigIntl,
    /// Logging station's special interest activity or event.
    #[strum(serialize = "MY_SIG")]
    MySig,
    /// International-text logging station activity or event name.
    #[strum(serialize = "MY_SIG_INTL")]
    MySigIntl,
    /// Additional information about the logging station's special activity or event.
    #[strum(serialize = "MY_SIG_INFO")]
    MySigInfo,
    /// International-text information about the logging station's activity or event.
    #[strum(serialize = "MY_SIG_INFO_INTL")]
    MySigInfoIntl,
    /// International Summits on the Air reference of the logging station.
    #[strum(serialize = "MY_SOTA_REF")]
    MySotaRef,
    /// Logging station's primary subdivision code, interpreted using its DXCC entity.
    #[strum(serialize = "MY_STATE")]
    MyState,
    /// Street address of the logging station.
    #[strum(serialize = "MY_STREET")]
    MyStreet,
    /// International-text street address of the logging station.
    #[strum(serialize = "MY_STREET_INTL")]
    MyStreetIntl,
    /// Two border counties claimable by the contacted station for the USA-CA award.
    #[strum(serialize = "MY_USACA_COUNTIES")]
    MyUsacaCounties,
    /// Two or four adjacent logging-station grid locators claimable for VUCC.
    /// Each locator has four or six characters.
    #[strum(serialize = "MY_VUCC_GRIDS")]
    MyVuccGrids,
    /// World Wildlife Flora & Fauna reference of the logging station.
    #[strum(serialize = "MY_WWFF_REF")]
    MyWwffRef,
    /// Name of the contacted operator.
    #[strum(serialize = "NAME")]
    Name,
    /// International-text name of the contacted operator.
    #[strum(serialize = "NAME_INTL")]
    NameIntl,
    /// Contact notes intended for the logging operator.
    #[strum(serialize = "NOTES")]
    Notes,
    /// International-text contact notes intended for the logging operator.
    #[strum(serialize = "NOTES_INTL")]
    NotesIntl,
    /// Nonnegative count of meteor-scatter bursts heard by the logging station.
    #[strum(serialize = "NR_BURSTS")]
    NrBursts,
    /// Nonnegative count of meteor-scatter pings heard by the logging station.
    #[strum(serialize = "NR_PINGS")]
    NrPings,
    /// Logging operator's callsign; also supplies the station callsign when absent.
    #[strum(serialize = "OPERATOR")]
    Operator,
    /// Callsign of the owner hosting the logging operator.
    /// When absent, the station callsign also identifies its owner.
    #[strum(serialize = "OWNER_CALLSIGN")]
    OwnerCallsign,
    /// WPX prefix of the contacted station.
    #[strum(serialize = "PFX")]
    Pfx,
    /// One or more Parks on the Air references for the contacted station.
    #[strum(serialize = "POTA_REF")]
    PotaRef,
    /// Contest precedence, for example the ARRL Sweepstakes exchange component.
    #[strum(serialize = "PRECEDENCE")]
    Precedence,
    /// Propagation mechanism used for the contact.
    #[strum(serialize = "PROP_MODE")]
    PropMode,
    /// Public encryption key.
    #[strum(serialize = "PUBLIC_KEY")]
    PublicKey,
    /// Date the contact was downloaded from the QRZ.COM logbook.
    #[strum(serialize = "QRZCOM_QSO_DOWNLOAD_DATE")]
    QrzcomQsoDownloadDate,
    /// Download status of the contact in the QRZ.COM logbook.
    #[strum(serialize = "QRZCOM_QSO_DOWNLOAD_STATUS")]
    QrzcomQsoDownloadStatus,
    /// Date of the most recent upload of this contact to QRZ.COM.
    #[strum(serialize = "QRZCOM_QSO_UPLOAD_DATE")]
    QrzcomQsoUploadDate,
    /// Upload status of this contact on QRZ.COM.
    #[strum(serialize = "QRZCOM_QSO_UPLOAD_STATUS")]
    QrzcomQsoUploadStatus,
    /// Message for the contacted operator to include in a paper or electronic QSL.
    #[strum(serialize = "QSLMSG")]
    Qslmsg,
    /// International-text message for the contacted operator's QSL.
    #[strum(serialize = "QSLMSG_INTL")]
    QslmsgIntl,
    /// Message received on a QSL addressed to the logging operator.
    #[strum(serialize = "QSLMSG_RCVD")]
    QslmsgRcvd,
    /// Date a QSL was received, conditional on the received status.
    #[strum(serialize = "QSLRDATE")]
    Qslrdate,
    /// Date a QSL was sent, conditional on the sent status.
    #[strum(serialize = "QSLSDATE")]
    Qslsdate,
    /// QSL received status; the default is N and verified V is import-only.
    #[strum(serialize = "QSL_RCVD")]
    QslRcvd,
    /// How the received QSL was, or is intended to be, conveyed; manager M is import-only.
    #[strum(serialize = "QSL_RCVD_VIA")]
    QslRcvdVia,
    /// QSL sent status; the default is N.
    #[strum(serialize = "QSL_SENT")]
    QslSent,
    /// How the sent QSL was, or is intended to be, conveyed; manager M is import-only.
    #[strum(serialize = "QSL_SENT_VIA")]
    QslSentVia,
    /// QSL routing information for the contacted station.
    #[strum(serialize = "QSL_VIA")]
    QslVia,
    /// Whether the logging station considers the contact complete, incomplete, unheard,
    /// or uncertain.
    #[strum(serialize = "QSO_COMPLETE")]
    QsoComplete,
    /// UTC date on which the contact began.
    #[strum(serialize = "QSO_DATE")]
    QsoDate,
    /// UTC date on which the contact ended.
    #[strum(serialize = "QSO_DATE_OFF")]
    QsoDateOff,
    /// Whether the contact was random rather than scheduled.
    #[strum(serialize = "QSO_RANDOM")]
    QsoRandom,
    /// City of the contacted station.
    #[strum(serialize = "QTH")]
    Qth,
    /// International-text city of the contacted station.
    #[strum(serialize = "QTH_INTL")]
    QthIntl,
    /// WAE or CQ entity within the contacted station's DXCC entity.
    /// Explicit None means the DXCC entity itself; absence carries no inference.
    #[strum(serialize = "REGION")]
    Region,
    /// Description of the contacted station's equipment.
    #[strum(serialize = "RIG")]
    Rig,
    /// International-text description of the contacted station's equipment.
    #[strum(serialize = "RIG_INTL")]
    RigIntl,
    /// Signal report received from the contacted station.
    #[strum(serialize = "RST_RCVD")]
    RstRcvd,
    /// Signal report sent to the contacted station.
    #[strum(serialize = "RST_SENT")]
    RstSent,
    /// Contacted station's nonnegative transmitter power in watts.
    #[strum(serialize = "RX_PWR")]
    RxPwr,
    /// Satellite mode code describing its uplink and downlink bands.
    #[strum(serialize = "SAT_MODE")]
    SatMode,
    /// Name of the satellite used for the contact.
    #[strum(serialize = "SAT_NAME")]
    SatName,
    /// Solar flux at contact time, from 0 through 300.
    #[strum(serialize = "SFI")]
    Sfi,
    /// Contacted station's special activity or interest group name.
    #[strum(serialize = "SIG")]
    Sig,
    /// International-text contacted station activity or interest group name.
    #[strum(serialize = "SIG_INTL")]
    SigIntl,
    /// Information about the contacted station's special activity or interest group.
    #[strum(serialize = "SIG_INFO")]
    SigInfo,
    /// International-text information about the contacted station's activity or group.
    #[strum(serialize = "SIG_INFO_INTL")]
    SigInfoIntl,
    /// Whether the contacted operator is now a Silent Key.
    #[strum(serialize = "SILENT_KEY")]
    SilentKey,
    /// Straight Key Century Club membership information of the contacted station.
    #[strum(serialize = "SKCC")]
    Skcc,
    /// International Summits on the Air reference of the contacted station.
    #[strum(serialize = "SOTA_REF")]
    SotaRef,
    /// Nonnegative contest serial number received.
    #[strum(serialize = "SRX")]
    Srx,
    /// Received contest exchange in Cabrillo form; dedicated contest fields prevail.
    #[strum(serialize = "SRX_STRING")]
    SrxString,
    /// Contacted station's primary subdivision code, interpreted using its DXCC entity.
    #[strum(serialize = "STATE")]
    State,
    /// Callsign used on air by the logging station; falls back to the operator callsign.
    #[strum(serialize = "STATION_CALLSIGN")]
    StationCallsign,
    /// Nonnegative contest serial number transmitted.
    #[strum(serialize = "STX")]
    Stx,
    /// Transmitted contest exchange in Cabrillo form; dedicated contest fields prevail.
    #[strum(serialize = "STX_STRING")]
    StxString,
    /// Operating submode; the recommended values depend on MODE.
    #[strum(serialize = "SUBMODE")]
    Submode,
    /// Whether the record describes a shortwave listener report.
    #[strum(serialize = "SWL")]
    Swl,
    /// Positive Ten-Ten membership number.
    #[strum(serialize = "TEN_TEN")]
    TenTen,
    /// UTC end time as HHMM or HHMMSS; without an end date, duration is under 24 hours.
    #[strum(serialize = "TIME_OFF")]
    TimeOff,
    /// UTC start time as HHMM or HHMMSS.
    #[strum(serialize = "TIME_ON")]
    TimeOn,
    /// Logging station's nonnegative transmitter power in watts.
    #[strum(serialize = "TX_PWR")]
    TxPwr,
    /// Positive UK Six Metre Group membership number of the contacted station.
    #[strum(serialize = "UKSMG")]
    Uksmg,
    /// Two contacted-station border counties credited for the USA-CA award.
    #[strum(serialize = "USACA_COUNTIES")]
    UsacaCounties,
    /// Legacy Canadian province; import-only, superseded by STATE.
    #[strum(serialize = "VE_PROV")]
    VeProv,
    /// Two or four adjacent contacted-station grid locators credited for VUCC.
    /// Each locator has four or six characters.
    #[strum(serialize = "VUCC_GRIDS")]
    VuccGrids,
    /// Web address of the contacted station.
    #[strum(serialize = "WEB")]
    Web,
    /// World Wildlife Flora & Fauna reference of the contacted station.
    #[strum(serialize = "WWFF_REF")]
    WwffRef,
}

impl StandardField {
    /// Canonical ADIF spelling of this field name.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// Header metadata or QSO record section containing this field.
    pub fn section(self) -> FieldSection {
        match self {
            Self::AdifVer | Self::CreatedTimestamp | Self::ProgramId | Self::ProgramVersion => {
                FieldSection::Header
            }
            _ => FieldSection::Record,
        }
    }

    /// Declared data types, with the current export type first.
    ///
    /// CREDIT fields additionally accept the legacy AwardList type on import.
    /// A field's declared type does not encode its additional semantic limits.
    pub fn data_types(self) -> &'static [DataType] {
        match self {
            Self::CreditSubmitted | Self::CreditGranted => {
                &[DataType::CreditList, DataType::AwardList]
            }
            Self::AwardSubmitted | Self::AwardGranted => &[DataType::SponsoredAwardList],
            Self::Address | Self::Notes | Self::Qslmsg | Self::QslmsgRcvd | Self::Rig => {
                &[DataType::MultilineString]
            }
            Self::AddressIntl | Self::NotesIntl | Self::QslmsgIntl | Self::RigIntl => {
                &[DataType::IntlMultilineString]
            }
            Self::Age
            | Self::Altitude
            | Self::AntAz
            | Self::AntEl
            | Self::AIndex
            | Self::Distance
            | Self::Freq
            | Self::FreqRx
            | Self::MaxBursts
            | Self::MyAltitude
            | Self::RxPwr
            | Self::TxPwr => &[DataType::Number],
            Self::AntPath
            | Self::ArrlSect
            | Self::Band
            | Self::BandRx
            | Self::Cnty
            | Self::Cont
            | Self::DarcDok
            | Self::DclQslRcvd
            | Self::DclQslSent
            | Self::Dxcc
            | Self::EqslAg
            | Self::EqslQslRcvd
            | Self::EqslQslSent
            | Self::ClublogQsoUploadStatus
            | Self::HamlogeuQsoUploadStatus
            | Self::HamqthQsoUploadStatus
            | Self::HrdlogQsoUploadStatus
            | Self::LotwQslRcvd
            | Self::LotwQslSent
            | Self::Mode
            | Self::MorseKeyType
            | Self::MyArrlSect
            | Self::MyCnty
            | Self::MyDarcDok
            | Self::MyDxcc
            | Self::MyMorseKeyType
            | Self::MyState
            | Self::PropMode
            | Self::QrzcomQsoDownloadStatus
            | Self::QrzcomQsoUploadStatus
            | Self::QslRcvd
            | Self::QslRcvdVia
            | Self::QslSent
            | Self::QslSentVia
            | Self::QsoComplete
            | Self::Region
            | Self::State => &[DataType::Enumeration],
            Self::ClublogQsoUploadDate
            | Self::DclQslrdate
            | Self::DclQslsdate
            | Self::EqslQslrdate
            | Self::EqslQslsdate
            | Self::HamlogeuQsoUploadDate
            | Self::HamqthQsoUploadDate
            | Self::HrdlogQsoUploadDate
            | Self::LotwQslrdate
            | Self::LotwQslsdate
            | Self::QrzcomQsoDownloadDate
            | Self::QrzcomQsoUploadDate
            | Self::Qslrdate
            | Self::Qslsdate
            | Self::QsoDate
            | Self::QsoDateOff => &[DataType::Date],
            Self::CntyAlt | Self::MyCntyAlt => {
                &[DataType::SecondaryAdministrativeSubdivisionListAlt]
            }
            Self::CommentIntl
            | Self::CountryIntl
            | Self::MyAntennaIntl
            | Self::MyCityIntl
            | Self::MyCountryIntl
            | Self::MyNameIntl
            | Self::MyPostalCodeIntl
            | Self::MyRigIntl
            | Self::MySigIntl
            | Self::MySigInfoIntl
            | Self::MyStreetIntl
            | Self::NameIntl
            | Self::QthIntl
            | Self::SigIntl
            | Self::SigInfoIntl => &[DataType::IntlString],
            Self::Cqz
            | Self::Fists
            | Self::FistsCc
            | Self::IotaIslandId
            | Self::Ituz
            | Self::MyCqZone
            | Self::MyFists
            | Self::MyIotaIslandId
            | Self::MyItuZone
            | Self::TenTen
            | Self::Uksmg => &[DataType::PositiveInteger],
            Self::ForceInit | Self::QsoRandom | Self::SilentKey | Self::Swl => &[DataType::Boolean],
            Self::Gridsquare | Self::MyGridsquare => &[DataType::GridSquare],
            Self::GridsquareExt | Self::MyGridsquareExt => &[DataType::GridSquareExt],
            Self::Iota | Self::MyIota => &[DataType::IOTARefNo],
            Self::KIndex | Self::NrBursts | Self::NrPings | Self::Sfi | Self::Srx | Self::Stx => {
                &[DataType::Integer]
            }
            Self::Lat | Self::Lon | Self::MyLat | Self::MyLon => &[DataType::Location],
            Self::MyPotaRef | Self::PotaRef => &[DataType::POTARefList],
            Self::MySotaRef | Self::SotaRef => &[DataType::SOTARef],
            Self::MyUsacaCounties | Self::UsacaCounties => &[DataType::SecondarySubdivisionList],
            Self::MyVuccGrids | Self::VuccGrids => &[DataType::GridSquareList],
            Self::MyWwffRef | Self::WwffRef => &[DataType::WWFFRef],
            Self::TimeOff | Self::TimeOn => &[DataType::Time],
            Self::AdifVer
            | Self::CreatedTimestamp
            | Self::ProgramId
            | Self::ProgramVersion
            | Self::Call
            | Self::Check
            | Self::Class
            | Self::Comment
            | Self::ContactedOp
            | Self::ContestId
            | Self::Country
            | Self::Email
            | Self::EqCall
            | Self::GuestOp
            | Self::MorseKeyInfo
            | Self::MsShower
            | Self::MyAntenna
            | Self::MyCity
            | Self::MyCountry
            | Self::MyMorseKeyInfo
            | Self::MyName
            | Self::MyPostalCode
            | Self::MyRig
            | Self::MySig
            | Self::MySigInfo
            | Self::MyStreet
            | Self::Name
            | Self::Operator
            | Self::OwnerCallsign
            | Self::Pfx
            | Self::Precedence
            | Self::PublicKey
            | Self::QslVia
            | Self::Qth
            | Self::RstRcvd
            | Self::RstSent
            | Self::SatMode
            | Self::SatName
            | Self::Sig
            | Self::SigInfo
            | Self::Skcc
            | Self::SrxString
            | Self::StationCallsign
            | Self::StxString
            | Self::Submode
            | Self::VeProv
            | Self::Web => &[DataType::String],
        }
    }

    /// Returns the ADIF-supplied member set for an Enumeration field, if any.
    ///
    /// Fields with other data types and Enumeration fields with externally
    /// defined member sets return None.
    pub fn enumeration(self) -> Option<Enumeration> {
        match self {
            Self::AntPath => Some(Enumeration::AntPath),
            Self::ArrlSect | Self::MyArrlSect => Some(Enumeration::ArrlSection),
            Self::Band | Self::BandRx => Some(Enumeration::Band),
            Self::Cnty | Self::MyCnty => Some(Enumeration::SecondarySubdivision),
            Self::Cont => Some(Enumeration::Continent),
            Self::DclQslRcvd | Self::EqslQslRcvd | Self::LotwQslRcvd | Self::QslRcvd => {
                Some(Enumeration::QslRcvd)
            }
            Self::DclQslSent | Self::EqslQslSent | Self::LotwQslSent | Self::QslSent => {
                Some(Enumeration::QslSent)
            }
            Self::Dxcc | Self::MyDxcc => Some(Enumeration::DxccEntityCode),
            Self::EqslAg => Some(Enumeration::EqslAg),
            Self::ClublogQsoUploadStatus
            | Self::HamlogeuQsoUploadStatus
            | Self::HamqthQsoUploadStatus
            | Self::HrdlogQsoUploadStatus
            | Self::QrzcomQsoUploadStatus => Some(Enumeration::QsoUploadStatus),
            Self::Mode => Some(Enumeration::Mode),
            Self::MorseKeyType | Self::MyMorseKeyType => Some(Enumeration::MorseKeyType),
            Self::MyState | Self::State => Some(Enumeration::PrimarySubdivision),
            Self::PropMode => Some(Enumeration::PropagationMode),
            Self::QrzcomQsoDownloadStatus => Some(Enumeration::QsoDownloadStatus),
            Self::QslRcvdVia | Self::QslSentVia => Some(Enumeration::QslVia),
            Self::QsoComplete => Some(Enumeration::QsoComplete),
            Self::Region => Some(Enumeration::Region),
            _ => None,
        }
    }
}
