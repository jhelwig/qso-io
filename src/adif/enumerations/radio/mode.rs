use strum::{EnumIter, EnumString, IntoStaticStr};

/// An ADIF operating mode, including legacy import-only mode values.
///
/// Modern records express a mode family here and its more specific form in
/// [`super::Submode`]. [`Mode::is_import_only`] identifies historical values
/// accepted for import but no longer permitted for export as `MODE`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum Mode {
    /// Amplitude modulation.
    #[strum(serialize = "AM")]
    Am,
    /// Amateur Radio Digital Open Protocol.
    #[strum(serialize = "ARDOP")]
    Ardop,
    /// Amateur television.
    #[strum(serialize = "ATV")]
    Atv,
    /// CHIP digital mode family, with CHIP64 and CHIP128 submodes.
    #[strum(serialize = "CHIP")]
    Chip,
    /// CLO operating mode; ADIF supplies no further description.
    #[strum(serialize = "CLO")]
    Clo,
    /// Contestia digital mode.
    #[strum(serialize = "CONTESTI")]
    Contesti,
    /// Continuous-wave Morse operation, including coherent CW.
    #[strum(serialize = "CW")]
    Cw,
    /// Digital voice family, including C4FM, DMR, DSTAR, FreeDV and M17.
    #[strum(serialize = "DIGITALVOICE")]
    DigitalVoice,
    /// Domino digital mode family.
    #[strum(serialize = "DOMINO")]
    Domino,
    /// Adaptive digital mode family, including FreeData and VARA.
    #[strum(serialize = "DYNAMIC")]
    Dynamic,
    /// Facsimile transmission.
    #[strum(serialize = "FAX")]
    Fax,
    /// Frequency modulation.
    #[strum(serialize = "FM")]
    Fm,
    /// FSK441 digital mode.
    #[strum(serialize = "FSK441")]
    Fsk441,
    /// Frequency shift keying, including SCAMP FSK submodes.
    #[strum(serialize = "FSK")]
    Fsk,
    /// Franke–Taylor design using eight-tone frequency shift keying.
    #[strum(serialize = "FT8")]
    Ft8,
    /// Hellschreiber family, including its FSK, FM and PSK variants.
    #[strum(serialize = "HELL")]
    Hell,
    /// ISCAT digital mode family.
    #[strum(serialize = "ISCAT")]
    Iscat,
    /// JT4 digital mode family.
    #[strum(serialize = "JT4")]
    Jt4,
    /// JT6M digital mode.
    #[strum(serialize = "JT6M")]
    Jt6m,
    /// JT9 digital mode family, including its fast variants.
    #[strum(serialize = "JT9")]
    Jt9,
    /// JT44 digital mode.
    #[strum(serialize = "JT44")]
    Jt44,
    /// JT65 digital mode family.
    #[strum(serialize = "JT65")]
    Jt65,
    /// Multiple-frequency-shift-keying family, including FT4, JS8 and Q65.
    #[strum(serialize = "MFSK")]
    Mfsk,
    /// MSK144 digital mode.
    #[strum(serialize = "MSK144")]
    Msk144,
    /// Single modulated tone, including SCAMP on/off keying.
    #[strum(serialize = "MTONE")]
    Mtone,
    /// MT63 digital mode.
    #[strum(serialize = "MT63")]
    Mt63,
    /// Orthogonal frequency-division multiplexing, including COFDM.
    #[strum(serialize = "OFDM")]
    Ofdm,
    /// Olivia digital mode family.
    #[strum(serialize = "OLIVIA")]
    Olivia,
    /// Opera digital mode family, for beacon and QSO operation.
    #[strum(serialize = "OPERA")]
    Opera,
    /// PACTOR family, with PAC2, PAC3 and PAC4 submodes.
    #[strum(serialize = "PAC")]
    Pac,
    /// PAX digital mode family.
    #[strum(serialize = "PAX")]
    Pax,
    /// Packet radio.
    #[strum(serialize = "PKT")]
    Pkt,
    /// Phase-shift-keying family, including binary and quadrature variants.
    #[strum(serialize = "PSK")]
    Psk,
    /// PSK2K digital mode.
    #[strum(serialize = "PSK2K")]
    Psk2k,
    /// Q15 digital mode; ADIF supplies no further description.
    #[strum(serialize = "Q15")]
    Q15,
    /// QRA64 digital mode family.
    #[strum(serialize = "QRA64")]
    Qra64,
    /// ROS digital mode family for EME, HF and MF operation.
    #[strum(serialize = "ROS")]
    Ros,
    /// Radioteletype, including the ASCI submode.
    #[strum(serialize = "RTTY")]
    Rtty,
    /// RTTYM digital mode; ADIF supplies no further description.
    #[strum(serialize = "RTTYM")]
    Rttym,
    /// Single-sideband voice, with lower and upper sideband submodes.
    #[strum(serialize = "SSB")]
    Ssb,
    /// Slow-scan television.
    #[strum(serialize = "SSTV")]
    Sstv,
    /// Tonal 10 digital mode, emphasizing sensitivity, band capacity and
    /// resistance to HF Doppler frequency spread.
    #[strum(serialize = "T10")]
    T10,
    /// THOR digital mode family.
    #[strum(serialize = "THOR")]
    Thor,
    /// Throb digital mode family.
    #[strum(serialize = "THRB")]
    Thrb,
    /// Teleprinting-over-radio family, including AMTOR FEC, GTOR and SITOR B.
    #[strum(serialize = "TOR")]
    Tor,
    /// V4 digital mode.
    #[strum(serialize = "V4")]
    V4,
    /// VOI operating mode; ADIF supplies no further description.
    #[strum(serialize = "VOI")]
    Voi,
    /// WINMOR digital mode.
    #[strum(serialize = "WINMOR")]
    Winmor,
    /// Weak Signal Propagation Reporter mode.
    #[strum(serialize = "WSPR")]
    Wspr,
    /// Legacy AMTOR forward-error-correction mode; import-only, now a TOR submode.
    #[strum(serialize = "AMTORFEC")]
    AmtorFec,
    /// Legacy ASCI radioteletype mode; import-only, now an RTTY submode.
    #[strum(serialize = "ASCI")]
    Asci,
    /// Legacy C4FM four-level FSK voice; import-only, now a digital voice submode.
    #[strum(serialize = "C4FM")]
    C4fm,
    /// Legacy CHIP64 mode; import-only, now a CHIP submode.
    #[strum(serialize = "CHIP64")]
    Chip64,
    /// Legacy CHIP128 mode; import-only, now a CHIP submode.
    #[strum(serialize = "CHIP128")]
    Chip128,
    /// Legacy Domino F mode; import-only, now a Domino submode.
    #[strum(serialize = "DOMINOF")]
    DominoF,
    /// Digital Smart Technologies for Amateur Radio; import-only as a mode,
    /// now a digital voice submode.
    #[strum(serialize = "DSTAR")]
    Dstar,
    /// Legacy FM Hellschreiber; import-only, now a Hell submode.
    #[strum(serialize = "FMHELL")]
    FmHell,
    /// Legacy FSK31 mode; import-only, now a PSK submode.
    #[strum(serialize = "FSK31")]
    Fsk31,
    /// Legacy GTOR mode; import-only, now a TOR submode.
    #[strum(serialize = "GTOR")]
    Gtor,
    /// Legacy Hell80 mode; import-only, now a Hell submode.
    #[strum(serialize = "HELL80")]
    Hell80,
    /// Legacy HFSK Hellschreiber; import-only, now a Hell submode.
    #[strum(serialize = "HFSK")]
    Hfsk,
    /// Legacy JT4 A variant; import-only, now a JT4 submode.
    #[strum(serialize = "JT4A")]
    Jt4a,
    /// Legacy JT4 B variant; import-only, now a JT4 submode.
    #[strum(serialize = "JT4B")]
    Jt4b,
    /// Legacy JT4 C variant; import-only, now a JT4 submode.
    #[strum(serialize = "JT4C")]
    Jt4c,
    /// Legacy JT4 D variant; import-only, now a JT4 submode.
    #[strum(serialize = "JT4D")]
    Jt4d,
    /// Legacy JT4 E variant; import-only, now a JT4 submode.
    #[strum(serialize = "JT4E")]
    Jt4e,
    /// Legacy JT4 F variant; import-only, now a JT4 submode.
    #[strum(serialize = "JT4F")]
    Jt4f,
    /// Legacy JT4 G variant; import-only, now a JT4 submode.
    #[strum(serialize = "JT4G")]
    Jt4g,
    /// Legacy JT65 A variant; import-only, now a JT65 submode.
    #[strum(serialize = "JT65A")]
    Jt65a,
    /// Legacy JT65 B variant; import-only, now a JT65 submode.
    #[strum(serialize = "JT65B")]
    Jt65b,
    /// Legacy JT65 C variant; import-only, now a JT65 submode.
    #[strum(serialize = "JT65C")]
    Jt65c,
    /// Legacy MFSK8 mode; import-only, now an MFSK submode.
    #[strum(serialize = "MFSK8")]
    Mfsk8,
    /// Legacy MFSK16 mode; import-only, now an MFSK submode.
    #[strum(serialize = "MFSK16")]
    Mfsk16,
    /// Legacy PACTOR II mode; import-only, now a PAC submode.
    #[strum(serialize = "PAC2")]
    Pac2,
    /// Legacy PACTOR III mode; import-only, now a PAC submode.
    #[strum(serialize = "PAC3")]
    Pac3,
    /// Legacy PAX2 mode; import-only, now a PAX submode.
    #[strum(serialize = "PAX2")]
    Pax2,
    /// Legacy coherent CW mode; import-only, now a CW submode.
    #[strum(serialize = "PCW")]
    Pcw,
    /// Legacy PSK10 mode; import-only, now a PSK submode.
    #[strum(serialize = "PSK10")]
    Psk10,
    /// Legacy PSK31 mode; import-only, now a PSK submode.
    #[strum(serialize = "PSK31")]
    Psk31,
    /// Legacy PSK63 mode; import-only, now a PSK submode.
    #[strum(serialize = "PSK63")]
    Psk63,
    /// Legacy PSK63F mode; import-only, now a PSK submode.
    #[strum(serialize = "PSK63F")]
    Psk63f,
    /// Legacy PSK125 mode; import-only, now a PSK submode.
    #[strum(serialize = "PSK125")]
    Psk125,
    /// Legacy PSKAM10 mode; import-only, now a PSK submode.
    #[strum(serialize = "PSKAM10")]
    PskAm10,
    /// Legacy PSKAM31 mode; import-only, now a PSK submode.
    #[strum(serialize = "PSKAM31")]
    PskAm31,
    /// Legacy PSKAM50 mode; import-only, now a PSK submode.
    #[strum(serialize = "PSKAM50")]
    PskAm50,
    /// Legacy PSKFEC31 mode; import-only, now a PSK submode.
    #[strum(serialize = "PSKFEC31")]
    PskFec31,
    /// Legacy PSK Hellschreiber; import-only, now a Hell submode.
    #[strum(serialize = "PSKHELL")]
    PskHell,
    /// Legacy QPSK31 mode; import-only, now a PSK submode.
    #[strum(serialize = "QPSK31")]
    Qpsk31,
    /// Legacy QPSK63 mode; import-only, now a PSK submode.
    #[strum(serialize = "QPSK63")]
    Qpsk63,
    /// Legacy QPSK125 mode; import-only, now a PSK submode.
    #[strum(serialize = "QPSK125")]
    Qpsk125,
    /// Legacy THRBX mode; import-only, now a Throb submode.
    #[strum(serialize = "THRBX")]
    Thrbx,
}

impl Mode {
    /// The canonical ADIF spelling.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// Whether ADIF permits this value only on import, not as an exported mode.
    pub const fn is_import_only(self) -> bool {
        matches!(
            self,
            Self::AmtorFec
                | Self::Asci
                | Self::C4fm
                | Self::Chip64
                | Self::Chip128
                | Self::DominoF
                | Self::Dstar
                | Self::FmHell
                | Self::Fsk31
                | Self::Gtor
                | Self::Hell80
                | Self::Hfsk
                | Self::Jt4a
                | Self::Jt4b
                | Self::Jt4c
                | Self::Jt4d
                | Self::Jt4e
                | Self::Jt4f
                | Self::Jt4g
                | Self::Jt65a
                | Self::Jt65b
                | Self::Jt65c
                | Self::Mfsk8
                | Self::Mfsk16
                | Self::Pac2
                | Self::Pac3
                | Self::Pax2
                | Self::Pcw
                | Self::Psk10
                | Self::Psk31
                | Self::Psk63
                | Self::Psk63f
                | Self::Psk125
                | Self::PskAm10
                | Self::PskAm31
                | Self::PskAm50
                | Self::PskFec31
                | Self::PskHell
                | Self::Qpsk31
                | Self::Qpsk63
                | Self::Qpsk125
                | Self::Thrbx
        )
    }
}
