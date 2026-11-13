//! Confirmation, transfer and propagation codes from ADIF 3.1.8 III.B.

use strum::{EnumIter, EnumString, IntoStaticStr};

/// The continent associated with a station or an IOTA reference.
///
/// See ADIF III.B.6.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum Continent {
    /// North America (`NA`).
    #[strum(serialize = "NA")]
    NorthAmerica,
    /// South America (`SA`).
    #[strum(serialize = "SA")]
    SouthAmerica,
    /// Europe (`EU`).
    #[strum(serialize = "EU")]
    Europe,
    /// Africa (`AF`).
    #[strum(serialize = "AF")]
    Africa,
    /// Oceania (`OC`).
    #[strum(serialize = "OC")]
    Oceania,
    /// Asia (`AS`).
    #[strum(serialize = "AS")]
    Asia,
    /// Antarctica (`AN`).
    #[strum(serialize = "AN")]
    Antarctica,
}

impl Continent {
    /// Returns the canonical ADIF abbreviation.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// Whether an eQSL callsign holds Authenticity Guaranteed status (III.B.9).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum EqslAg {
    /// The callsign holds Authenticity Guaranteed status (`Y`).
    #[strum(serialize = "Y")]
    Yes,
    /// The callsign does not hold Authenticity Guaranteed status (`N`).
    #[strum(serialize = "N")]
    No,
    /// Status was not specified; the ADIF default (`U`).
    #[strum(serialize = "U")]
    Unspecified,
}

impl EqslAg {
    /// Returns the canonical ADIF status.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// The medium used to confirm a QSO, including within a CreditList (III.B.14).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum QslMedium {
    /// A paper QSL card (`CARD`).
    #[strum(serialize = "CARD")]
    Card,
    /// Confirmation through eQSL.cc (`EQSL`).
    #[strum(serialize = "EQSL")]
    Eqsl,
    /// Confirmation through ARRL Logbook of the World (`LOTW`).
    #[strum(serialize = "LOTW")]
    Lotw,
}

impl QslMedium {
    /// Returns the canonical ADIF medium.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// Receipt or online confirmation status of a QSL (III.B.15).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum QslRcvd {
    /// A card was received or an online service confirmed the QSO (`Y`).
    #[strum(serialize = "Y")]
    Yes,
    /// No card or online confirmation has been received (`N`).
    #[strum(serialize = "N")]
    No,
    /// A card or online confirmation has been requested (`R`).
    #[strum(serialize = "R")]
    Requested,
    /// DXCC credit was granted for a card or LoTW confirmation (`V`, import-only).
    #[strum(serialize = "V")]
    Verified,
    /// Ignore the QSL or treat it as invalid (`I`).
    #[strum(serialize = "I")]
    Ignore,
}

impl QslRcvd {
    /// Returns the canonical ADIF receipt status.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// Reports whether this status is accepted only for importing old data.
    pub fn import_only(self) -> bool {
        self == Self::Verified
    }
}

/// Sending or online submission status of a QSL (III.B.16).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum QslSent {
    /// A card was sent or the online service accepted the uploaded QSO (`Y`).
    #[strum(serialize = "Y")]
    Yes,
    /// Do not send a card or upload the QSO (`N`).
    #[strum(serialize = "N")]
    No,
    /// The contacted station requested a card or upload (`R`).
    #[strum(serialize = "R")]
    Requested,
    /// The card or upload has been selected for sending (`Q`).
    #[strum(serialize = "Q")]
    Queued,
    /// Ignore the QSL or treat it as invalid (`I`).
    #[strum(serialize = "I")]
    Ignore,
}

impl QslSent {
    /// Returns the canonical ADIF sending status.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// The route by which a QSL was sent or received (III.B.17).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum QslVia {
    /// Through a QSL bureau (`B`).
    #[strum(serialize = "B")]
    Bureau,
    /// Directly (`D`).
    #[strum(serialize = "D")]
    Direct,
    /// Electronically (`E`).
    #[strum(serialize = "E")]
    Electronic,
    /// Through a QSL manager (`M`, import-only).
    #[strum(serialize = "M")]
    Manager,
}

impl QslVia {
    /// Returns the canonical ADIF route.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// Reports whether this route is accepted only for importing old data.
    pub fn import_only(self) -> bool {
        self == Self::Manager
    }
}

/// Whether the QSO was completed (III.B.18).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum QsoComplete {
    /// The QSO was completed (`Y`).
    #[strum(serialize = "Y")]
    Yes,
    /// The QSO was not completed (`N`).
    #[strum(serialize = "N")]
    No,
    /// The other station was not heard (`NIL`).
    #[strum(serialize = "NIL")]
    NotHeard,
    /// Completion is uncertain (`?`).
    #[strum(serialize = "?")]
    Uncertain,
}

impl QsoComplete {
    /// Returns the canonical ADIF completion status.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// Download state of a QSO at an online service (III.B.19).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum QsoDownloadStatus {
    /// The QSO has been downloaded (`Y`).
    #[strum(serialize = "Y")]
    Downloaded,
    /// The QSO has not been downloaded (`N`).
    #[strum(serialize = "N")]
    NotDownloaded,
    /// Ignore the QSO or treat it as invalid (`I`).
    #[strum(serialize = "I")]
    Ignore,
}

impl QsoDownloadStatus {
    /// Returns the canonical ADIF download status.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// Upload state of a QSO at an online service (III.B.20).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum QsoUploadStatus {
    /// The online service accepted the uploaded QSO (`Y`).
    #[strum(serialize = "Y")]
    Uploaded,
    /// Do not upload the QSO (`N`).
    #[strum(serialize = "N")]
    DoNotUpload,
    /// The QSO changed after its last upload (`M`).
    #[strum(serialize = "M")]
    Modified,
}

impl QsoUploadStatus {
    /// Returns the canonical ADIF upload status.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// The mechanism by which radio signals propagated (III.B.13).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PropagationMode {
    /// Aircraft scatter (`AS`).
    #[strum(serialize = "AS")]
    AircraftScatter,
    /// Auroral propagation (`AUR`).
    #[strum(serialize = "AUR")]
    Aurora,
    /// Auroral-E propagation (`AUE`).
    #[strum(serialize = "AUE")]
    AuroralE,
    /// Back scatter (`BS`).
    #[strum(serialize = "BS")]
    BackScatter,
    /// Earth-Moon-Earth (`EME`).
    #[strum(serialize = "EME")]
    EarthMoonEarth,
    /// Short-path F2 reflection (`F2`).
    #[strum(serialize = "F2")]
    F2,
    /// Field-aligned irregularities (`FAI`).
    #[strum(serialize = "FAI")]
    FieldAlignedIrregularities,
    /// Ground wave (`GWAVE`).
    #[strum(serialize = "GWAVE")]
    GroundWave,
    /// Internet-assisted communication (`INTERNET`).
    #[strum(serialize = "INTERNET")]
    Internet,
    /// Ionospheric scatter (`ION`).
    #[strum(serialize = "ION")]
    IonosphericScatter,
    /// Meteor scatter (`MS`).
    #[strum(serialize = "MS")]
    MeteorScatter,
    /// A terrestrial or atmospheric repeater or transponder (`RPT`).
    #[strum(serialize = "RPT")]
    Repeater,
    /// Rain scatter (`RS`).
    #[strum(serialize = "RS")]
    RainScatter,
    /// A satellite (`SAT`).
    #[strum(serialize = "SAT")]
    Satellite,
    /// Sporadic-E reflection (`ES`).
    #[strum(serialize = "ES")]
    SporadicE,
    /// Transequatorial propagation (`TEP`).
    #[strum(serialize = "TEP")]
    Transequatorial,
    /// Tropospheric ducting (`TR`).
    #[strum(serialize = "TR")]
    Tropospheric,
    /// EchoLink-assisted communication (`ECH`).
    #[strum(serialize = "ECH")]
    Echolink,
    /// IRLP-assisted communication (`IRL`).
    #[strum(serialize = "IRL")]
    Irlp,
    /// Line of sight, including transmission through obstacles (`LOS`).
    #[strum(serialize = "LOS")]
    LineOfSight,
}

impl PropagationMode {
    /// Returns the canonical ADIF propagation code.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// How a Morse key produces elements and composes characters (III.B.11).
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum MorseKeyType {
    /// Straight key: the operator forms every element (`SK`).
    #[strum(serialize = "SK")]
    StraightKey,
    /// Sideswiper: hand-formed elements using alternating sides (`SS`).
    #[strum(serialize = "SS")]
    Sideswiper,
    /// Single paddle operating an electronic keyer (`SP`).
    #[strum(serialize = "SP")]
    SinglePaddle,
    /// Dual paddles operating an electronic keyer (`DP`).
    #[strum(serialize = "DP")]
    DualPaddle,
    /// Semi-automatic mechanical key with automatic dots (`BUG`).
    #[strum(serialize = "BUG")]
    Bug,
    /// Fully automatic mechanical bug (`FAB`).
    #[strum(serialize = "FAB")]
    FullyAutomaticBug,
    /// Computer-generated Morse (`CPU`).
    #[strum(serialize = "CPU")]
    Computer,
}

impl MorseKeyType {
    /// Returns the canonical ADIF key abbreviation.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}
