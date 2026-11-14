use strum::{EnumIter, EnumString, IntoStaticStr};

use super::Mode;

/// A specific form of an ADIF operating mode.
///
/// [`Submode::mode`] gives the required parent `MODE` for each `SUBMODE`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum Submode {
    /// 8PSK125 variant of phase-shift keying.
    #[strum(serialize = "8PSK125")]
    EightPsk125,
    /// 8PSK125F variant of phase-shift keying.
    #[strum(serialize = "8PSK125F")]
    EightPsk125f,
    /// 8PSK125FL variant of phase-shift keying.
    #[strum(serialize = "8PSK125FL")]
    EightPsk125fl,
    /// 8PSK250 variant of phase-shift keying.
    #[strum(serialize = "8PSK250")]
    EightPsk250,
    /// 8PSK250F variant of phase-shift keying.
    #[strum(serialize = "8PSK250F")]
    EightPsk250f,
    /// 8PSK250FL variant of phase-shift keying.
    #[strum(serialize = "8PSK250FL")]
    EightPsk250fl,
    /// 8PSK500 variant of phase-shift keying.
    #[strum(serialize = "8PSK500")]
    EightPsk500,
    /// 8PSK500F variant of phase-shift keying.
    #[strum(serialize = "8PSK500F")]
    EightPsk500f,
    /// 8PSK1000 variant of phase-shift keying.
    #[strum(serialize = "8PSK1000")]
    EightPsk1000,
    /// 8PSK1000F variant of phase-shift keying.
    #[strum(serialize = "8PSK1000F")]
    EightPsk1000f,
    /// 8PSK1200F variant of phase-shift keying.
    #[strum(serialize = "8PSK1200F")]
    EightPsk1200f,
    /// AMTOR forward-error-correction form of teleprinting over radio.
    #[strum(serialize = "AMTORFEC")]
    AmtorFec,
    /// ASCI form of radioteletype.
    #[strum(serialize = "ASCI")]
    Asci,
    /// C4FM four-level FSK digital voice.
    #[strum(serialize = "C4FM")]
    C4fm,
    /// CHIP64 form of CHIP.
    #[strum(serialize = "CHIP64")]
    Chip64,
    /// CHIP128 form of CHIP.
    #[strum(serialize = "CHIP128")]
    Chip128,
    /// Digital Mobile Radio digital voice.
    #[strum(serialize = "DMR")]
    Dmr,
    /// DOM-M form of Domino.
    #[strum(serialize = "DOM-M")]
    DomM,
    /// DOM4 form of Domino.
    #[strum(serialize = "DOM4")]
    Dom4,
    /// DOM5 form of Domino.
    #[strum(serialize = "DOM5")]
    Dom5,
    /// DOM8 form of Domino.
    #[strum(serialize = "DOM8")]
    Dom8,
    /// DOM11 form of Domino.
    #[strum(serialize = "DOM11")]
    Dom11,
    /// DOM16 form of Domino.
    #[strum(serialize = "DOM16")]
    Dom16,
    /// DOM22 form of Domino.
    #[strum(serialize = "DOM22")]
    Dom22,
    /// DOM44 form of Domino.
    #[strum(serialize = "DOM44")]
    Dom44,
    /// DOM88 form of Domino.
    #[strum(serialize = "DOM88")]
    Dom88,
    /// DominoEX form of Domino.
    #[strum(serialize = "DOMINOEX")]
    DominoEx,
    /// DominoF form of Domino.
    #[strum(serialize = "DOMINOF")]
    DominoF,
    /// Digital Smart Technologies for Amateur Radio digital voice.
    #[strum(serialize = "DSTAR")]
    Dstar,
    /// FM Hellschreiber.
    #[strum(serialize = "FMHELL")]
    FmHell,
    /// Data communications using Codec2 HF modems, in the dynamic mode family.
    #[strum(serialize = "FREEDATA")]
    FreeData,
    /// Open-source digital voice mode for HF radio.
    #[strum(serialize = "FREEDV")]
    FreeDv,
    /// FSK31 form of the PSK family.
    #[strum(serialize = "FSK31")]
    Fsk31,
    /// FSKH105 form of Hellschreiber.
    #[strum(serialize = "FSKH105")]
    Fskh105,
    /// FSKH245 form of Hellschreiber.
    #[strum(serialize = "FSKH245")]
    Fskh245,
    /// FSK Hellschreiber.
    #[strum(serialize = "FSKHELL")]
    FskHell,
    /// FSQCall protocol used with Fast Simple QSO transmission mode.
    #[strum(serialize = "FSQCALL")]
    FsqCall,
    /// WSJT-X FST4 digital mode, in the MFSK family.
    #[strum(serialize = "FST4")]
    Fst4,
    /// WSJT-X quasi-beacon mode carrying WSPR-style messages.
    #[strum(serialize = "FST4W")]
    Fst4w,
    /// FT2 form of the MFSK family.
    #[strum(serialize = "FT2")]
    Ft2,
    /// FT4 digital mode designed specifically for radio contesting.
    #[strum(serialize = "FT4")]
    Ft4,
    /// GTOR form of teleprinting over radio.
    #[strum(serialize = "GTOR")]
    Gtor,
    /// Hell80 form of Hellschreiber.
    #[strum(serialize = "HELL80")]
    Hell80,
    /// HellX5 form of Hellschreiber.
    #[strum(serialize = "HELLX5")]
    HellX5,
    /// HellX9 form of Hellschreiber.
    #[strum(serialize = "HELLX9")]
    HellX9,
    /// HFSK form of Hellschreiber.
    #[strum(serialize = "HFSK")]
    Hfsk,
    /// A variant of ISCAT.
    #[strum(serialize = "ISCAT-A")]
    IscatA,
    /// B variant of ISCAT.
    #[strum(serialize = "ISCAT-B")]
    IscatB,
    /// Jordan Sherer's eight-tone FSK design.
    #[strum(serialize = "JS8")]
    Js8,
    /// A variant of JT4.
    #[strum(serialize = "JT4A")]
    Jt4a,
    /// B variant of JT4.
    #[strum(serialize = "JT4B")]
    Jt4b,
    /// C variant of JT4.
    #[strum(serialize = "JT4C")]
    Jt4c,
    /// D variant of JT4.
    #[strum(serialize = "JT4D")]
    Jt4d,
    /// E variant of JT4.
    #[strum(serialize = "JT4E")]
    Jt4e,
    /// F variant of JT4.
    #[strum(serialize = "JT4F")]
    Jt4f,
    /// G variant of JT4.
    #[strum(serialize = "JT4G")]
    Jt4g,
    /// JT9-1 variant of JT9.
    #[strum(serialize = "JT9-1")]
    Jt9_1,
    /// JT9-2 variant of JT9.
    #[strum(serialize = "JT9-2")]
    Jt9_2,
    /// JT9-5 variant of JT9.
    #[strum(serialize = "JT9-5")]
    Jt9_5,
    /// JT9-10 variant of JT9.
    #[strum(serialize = "JT9-10")]
    Jt9_10,
    /// JT9-30 variant of JT9.
    #[strum(serialize = "JT9-30")]
    Jt9_30,
    /// A variant of JT9.
    #[strum(serialize = "JT9A")]
    Jt9a,
    /// B variant of JT9.
    #[strum(serialize = "JT9B")]
    Jt9b,
    /// C variant of JT9.
    #[strum(serialize = "JT9C")]
    Jt9c,
    /// D variant of JT9.
    #[strum(serialize = "JT9D")]
    Jt9d,
    /// E variant of JT9.
    #[strum(serialize = "JT9E")]
    Jt9e,
    /// Fast E variant of JT9.
    #[strum(serialize = "JT9E FAST")]
    Jt9eFast,
    /// F variant of JT9.
    #[strum(serialize = "JT9F")]
    Jt9f,
    /// Fast F variant of JT9.
    #[strum(serialize = "JT9F FAST")]
    Jt9fFast,
    /// G variant of JT9.
    #[strum(serialize = "JT9G")]
    Jt9g,
    /// Fast G variant of JT9.
    #[strum(serialize = "JT9G FAST")]
    Jt9gFast,
    /// H variant of JT9.
    #[strum(serialize = "JT9H")]
    Jt9h,
    /// Fast H variant of JT9.
    #[strum(serialize = "JT9H FAST")]
    Jt9hFast,
    /// A variant of JT65.
    #[strum(serialize = "JT65A")]
    Jt65a,
    /// B variant of JT65.
    #[strum(serialize = "JT65B")]
    Jt65b,
    /// B2 variant of JT65.
    #[strum(serialize = "JT65B2")]
    Jt65b2,
    /// C variant of JT65.
    #[strum(serialize = "JT65C")]
    Jt65c,
    /// C2 variant of JT65.
    #[strum(serialize = "JT65C2")]
    Jt65c2,
    /// JTMS form of the MFSK family.
    #[strum(serialize = "JTMS")]
    Jtms,
    /// JTTY form of the MFSK family.
    #[strum(serialize = "JTTY")]
    Jtty,
    /// Amplitude-modulated voice, lower sideband, suppressed carrier.
    #[strum(serialize = "LSB")]
    Lsb,
    /// Digital radio protocol using the Codec 2 voice encoder.
    #[strum(serialize = "M17")]
    M17,
    /// MFSK4 form of multiple-frequency-shift keying.
    #[strum(serialize = "MFSK4")]
    Mfsk4,
    /// MFSK8 form of multiple-frequency-shift keying.
    #[strum(serialize = "MFSK8")]
    Mfsk8,
    /// MFSK11 form of multiple-frequency-shift keying.
    #[strum(serialize = "MFSK11")]
    Mfsk11,
    /// MFSK16 form of multiple-frequency-shift keying.
    #[strum(serialize = "MFSK16")]
    Mfsk16,
    /// MFSK22 form of multiple-frequency-shift keying.
    #[strum(serialize = "MFSK22")]
    Mfsk22,
    /// MFSK31 form of multiple-frequency-shift keying.
    #[strum(serialize = "MFSK31")]
    Mfsk31,
    /// MFSK32 form of multiple-frequency-shift keying.
    #[strum(serialize = "MFSK32")]
    Mfsk32,
    /// MFSK64 form of multiple-frequency-shift keying.
    #[strum(serialize = "MFSK64")]
    Mfsk64,
    /// MFSK64L form of multiple-frequency-shift keying.
    #[strum(serialize = "MFSK64L")]
    Mfsk64l,
    /// MFSK128 form of multiple-frequency-shift keying.
    #[strum(serialize = "MFSK128")]
    Mfsk128,
    /// MFSK128L form of multiple-frequency-shift keying.
    #[strum(serialize = "MFSK128L")]
    Mfsk128l,
    /// NAVTEX form of teleprinting over radio.
    #[strum(serialize = "NAVTEX")]
    Navtex,
    /// Olivia 4/125 variant.
    #[strum(serialize = "OLIVIA 4/125")]
    Olivia4_125,
    /// Olivia 4/250 variant.
    #[strum(serialize = "OLIVIA 4/250")]
    Olivia4_250,
    /// Olivia 8/250 variant.
    #[strum(serialize = "OLIVIA 8/250")]
    Olivia8_250,
    /// Olivia 8/500 variant.
    #[strum(serialize = "OLIVIA 8/500")]
    Olivia8_500,
    /// Olivia 16/500 variant.
    #[strum(serialize = "OLIVIA 16/500")]
    Olivia16_500,
    /// Olivia 16/1000 variant.
    #[strum(serialize = "OLIVIA 16/1000")]
    Olivia16_1000,
    /// Olivia 32/1000 variant.
    #[strum(serialize = "OLIVIA 32/1000")]
    Olivia32_1000,
    /// Opera beacon operation.
    #[strum(serialize = "OPERA-BEACON")]
    OperaBeacon,
    /// Opera QSO operation.
    #[strum(serialize = "OPERA-QSO")]
    OperaQso,
    /// PACTOR II form of the PAC family.
    #[strum(serialize = "PAC2")]
    Pac2,
    /// PACTOR III form of the PAC family.
    #[strum(serialize = "PAC3")]
    Pac3,
    /// PACTOR IV form of the PAC family.
    #[strum(serialize = "PAC4")]
    Pac4,
    /// PAX2 form of PAX.
    #[strum(serialize = "PAX2")]
    Pax2,
    /// Coherent continuous-wave operation.
    #[strum(serialize = "PCW")]
    Pcw,
    /// PSK10 form of phase-shift keying.
    #[strum(serialize = "PSK10")]
    Psk10,
    /// PSK31 form of phase-shift keying.
    #[strum(serialize = "PSK31")]
    Psk31,
    /// PSK63 form of phase-shift keying.
    #[strum(serialize = "PSK63")]
    Psk63,
    /// PSK63F form of phase-shift keying.
    #[strum(serialize = "PSK63F")]
    Psk63f,
    /// PSK63RC10 form of phase-shift keying.
    #[strum(serialize = "PSK63RC10")]
    Psk63rc10,
    /// PSK63RC20 form of phase-shift keying.
    #[strum(serialize = "PSK63RC20")]
    Psk63rc20,
    /// PSK63RC32 form of phase-shift keying.
    #[strum(serialize = "PSK63RC32")]
    Psk63rc32,
    /// PSK63RC4 form of phase-shift keying.
    #[strum(serialize = "PSK63RC4")]
    Psk63rc4,
    /// PSK63RC5 form of phase-shift keying.
    #[strum(serialize = "PSK63RC5")]
    Psk63rc5,
    /// PSK125 form of phase-shift keying.
    #[strum(serialize = "PSK125")]
    Psk125,
    /// PSK125RC10 form of phase-shift keying.
    #[strum(serialize = "PSK125RC10")]
    Psk125rc10,
    /// PSK125RC12 form of phase-shift keying.
    #[strum(serialize = "PSK125RC12")]
    Psk125rc12,
    /// PSK125RC16 form of phase-shift keying.
    #[strum(serialize = "PSK125RC16")]
    Psk125rc16,
    /// PSK125RC4 form of phase-shift keying.
    #[strum(serialize = "PSK125RC4")]
    Psk125rc4,
    /// PSK125RC5 form of phase-shift keying.
    #[strum(serialize = "PSK125RC5")]
    Psk125rc5,
    /// PSK250 form of phase-shift keying.
    #[strum(serialize = "PSK250")]
    Psk250,
    /// PSK250RC2 form of phase-shift keying.
    #[strum(serialize = "PSK250RC2")]
    Psk250rc2,
    /// PSK250RC3 form of phase-shift keying.
    #[strum(serialize = "PSK250RC3")]
    Psk250rc3,
    /// PSK250RC5 form of phase-shift keying.
    #[strum(serialize = "PSK250RC5")]
    Psk250rc5,
    /// PSK250RC6 form of phase-shift keying.
    #[strum(serialize = "PSK250RC6")]
    Psk250rc6,
    /// PSK250RC7 form of phase-shift keying.
    #[strum(serialize = "PSK250RC7")]
    Psk250rc7,
    /// PSK500 form of phase-shift keying.
    #[strum(serialize = "PSK500")]
    Psk500,
    /// PSK500RC2 form of phase-shift keying.
    #[strum(serialize = "PSK500RC2")]
    Psk500rc2,
    /// PSK500RC3 form of phase-shift keying.
    #[strum(serialize = "PSK500RC3")]
    Psk500rc3,
    /// PSK500RC4 form of phase-shift keying.
    #[strum(serialize = "PSK500RC4")]
    Psk500rc4,
    /// PSK800RC2 form of phase-shift keying.
    #[strum(serialize = "PSK800RC2")]
    Psk800rc2,
    /// PSK1000 form of phase-shift keying.
    #[strum(serialize = "PSK1000")]
    Psk1000,
    /// PSK1000RC2 form of phase-shift keying.
    #[strum(serialize = "PSK1000RC2")]
    Psk1000rc2,
    /// PSKAM10 form of phase-shift keying.
    #[strum(serialize = "PSKAM10")]
    PskAm10,
    /// PSKAM31 form of phase-shift keying.
    #[strum(serialize = "PSKAM31")]
    PskAm31,
    /// PSKAM50 form of phase-shift keying.
    #[strum(serialize = "PSKAM50")]
    PskAm50,
    /// PSKFEC31 form of phase-shift keying.
    #[strum(serialize = "PSKFEC31")]
    PskFec31,
    /// PSK Hellschreiber.
    #[strum(serialize = "PSKHELL")]
    PskHell,
    /// QPSK31 form of the PSK family.
    #[strum(serialize = "QPSK31")]
    Qpsk31,
    /// Q65 form of the MFSK family.
    #[strum(serialize = "Q65")]
    Q65,
    /// QPSK63 form of the PSK family.
    #[strum(serialize = "QPSK63")]
    Qpsk63,
    /// QPSK125 form of the PSK family.
    #[strum(serialize = "QPSK125")]
    Qpsk125,
    /// QPSK250 form of the PSK family.
    #[strum(serialize = "QPSK250")]
    Qpsk250,
    /// QPSK500 form of the PSK family.
    #[strum(serialize = "QPSK500")]
    Qpsk500,
    /// A variant of QRA64.
    #[strum(serialize = "QRA64A")]
    Qra64a,
    /// B variant of QRA64.
    #[strum(serialize = "QRA64B")]
    Qra64b,
    /// C variant of QRA64.
    #[strum(serialize = "QRA64C")]
    Qra64c,
    /// D variant of QRA64.
    #[strum(serialize = "QRA64D")]
    Qra64d,
    /// E variant of QRA64.
    #[strum(serialize = "QRA64E")]
    Qra64e,
    /// Images transmitted using Ribbit, in the OFDM family.
    #[strum(serialize = "RIBBIT_PIX")]
    RibbitPix,
    /// Short text messages transmitted using Ribbit, in the OFDM family.
    #[strum(serialize = "RIBBIT_SMS")]
    RibbitSms,
    /// ROS for Earth–Moon–Earth operation.
    #[strum(serialize = "ROS-EME")]
    RosEme,
    /// ROS for HF operation.
    #[strum(serialize = "ROS-HF")]
    RosHf,
    /// ROS for MF operation.
    #[strum(serialize = "ROS-MF")]
    RosMf,
    /// SCAMP fast frequency shift keying.
    #[strum(serialize = "SCAMP_FAST")]
    ScampFast,
    /// SCAMP single modulated tone on/off keying.
    #[strum(serialize = "SCAMP_OO")]
    ScampOo,
    /// SCAMP single modulated tone slow on/off keying.
    #[strum(serialize = "SCAMP_OO_SLW")]
    ScampOoSlow,
    /// SCAMP slow frequency shift keying.
    #[strum(serialize = "SCAMP_SLOW")]
    ScampSlow,
    /// SCAMP very slow frequency shift keying.
    #[strum(serialize = "SCAMP_VSLOW")]
    ScampVerySlow,
    /// SIM31 form of the PSK family.
    #[strum(serialize = "SIM31")]
    Sim31,
    /// SITOR B form of teleprinting over radio.
    #[strum(serialize = "SITORB")]
    SitorB,
    /// Slow Hellschreiber.
    #[strum(serialize = "SLOWHELL")]
    SlowHell,
    /// THOR-M form of THOR.
    #[strum(serialize = "THOR-M")]
    ThorM,
    /// THOR4 form of THOR.
    #[strum(serialize = "THOR4")]
    Thor4,
    /// THOR5 form of THOR.
    #[strum(serialize = "THOR5")]
    Thor5,
    /// THOR8 form of THOR.
    #[strum(serialize = "THOR8")]
    Thor8,
    /// THOR11 form of THOR.
    #[strum(serialize = "THOR11")]
    Thor11,
    /// THOR16 form of THOR.
    #[strum(serialize = "THOR16")]
    Thor16,
    /// THOR22 form of THOR.
    #[strum(serialize = "THOR22")]
    Thor22,
    /// THOR25X4 form of THOR.
    #[strum(serialize = "THOR25X4")]
    Thor25x4,
    /// THOR50X1 form of THOR.
    #[strum(serialize = "THOR50X1")]
    Thor50x1,
    /// THOR50X2 form of THOR.
    #[strum(serialize = "THOR50X2")]
    Thor50x2,
    /// THOR100 form of THOR.
    #[strum(serialize = "THOR100")]
    Thor100,
    /// THRBX form of Throb.
    #[strum(serialize = "THRBX")]
    Thrbx,
    /// THRBX1 form of Throb.
    #[strum(serialize = "THRBX1")]
    Thrbx1,
    /// THRBX2 form of Throb.
    #[strum(serialize = "THRBX2")]
    Thrbx2,
    /// THRBX4 form of Throb.
    #[strum(serialize = "THRBX4")]
    Thrbx4,
    /// THROB1 form of Throb.
    #[strum(serialize = "THROB1")]
    Throb1,
    /// THROB2 form of Throb.
    #[strum(serialize = "THROB2")]
    Throb2,
    /// THROB4 form of Throb.
    #[strum(serialize = "THROB4")]
    Throb4,
    /// Amplitude-modulated voice, upper sideband, suppressed carrier.
    #[strum(serialize = "USB")]
    Usb,
    /// Channel-adaptive high-speed modem for HF.
    #[strum(serialize = "VARA HF")]
    VaraHf,
    /// Channel-adaptive high-speed modem for satellite operations.
    #[strum(serialize = "VARA SATELLITE")]
    VaraSatellite,
    /// VARA FM 1200 channel-adaptive high-speed modem for FM transceivers.
    #[strum(serialize = "VARA FM 1200")]
    VaraFm1200,
    /// VARA FM 9600 channel-adaptive high-speed modem for FM transceivers.
    #[strum(serialize = "VARA FM 9600")]
    VaraFm9600,
}

impl Submode {
    /// The canonical ADIF spelling, including significant spaces and punctuation.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// The parent `MODE` required by ADIF for this `SUBMODE`.
    pub const fn mode(self) -> Mode {
        match self {
            Self::EightPsk125
            | Self::EightPsk125f
            | Self::EightPsk125fl
            | Self::EightPsk250
            | Self::EightPsk250f
            | Self::EightPsk250fl
            | Self::EightPsk500
            | Self::EightPsk500f
            | Self::EightPsk1000
            | Self::EightPsk1000f
            | Self::EightPsk1200f
            | Self::Fsk31
            | Self::Psk10
            | Self::Psk31
            | Self::Psk63
            | Self::Psk63f
            | Self::Psk63rc10
            | Self::Psk63rc20
            | Self::Psk63rc32
            | Self::Psk63rc4
            | Self::Psk63rc5
            | Self::Psk125
            | Self::Psk125rc10
            | Self::Psk125rc12
            | Self::Psk125rc16
            | Self::Psk125rc4
            | Self::Psk125rc5
            | Self::Psk250
            | Self::Psk250rc2
            | Self::Psk250rc3
            | Self::Psk250rc5
            | Self::Psk250rc6
            | Self::Psk250rc7
            | Self::Psk500
            | Self::Psk500rc2
            | Self::Psk500rc3
            | Self::Psk500rc4
            | Self::Psk800rc2
            | Self::Psk1000
            | Self::Psk1000rc2
            | Self::PskAm10
            | Self::PskAm31
            | Self::PskAm50
            | Self::PskFec31
            | Self::Qpsk31
            | Self::Qpsk63
            | Self::Qpsk125
            | Self::Qpsk250
            | Self::Qpsk500
            | Self::Sim31 => Mode::Psk,
            Self::AmtorFec | Self::Gtor | Self::Navtex | Self::SitorB => Mode::Tor,
            Self::Asci => Mode::Rtty,
            Self::C4fm | Self::Dmr | Self::Dstar | Self::FreeDv | Self::M17 => Mode::DigitalVoice,
            Self::Chip64 | Self::Chip128 => Mode::Chip,
            Self::DomM
            | Self::Dom4
            | Self::Dom5
            | Self::Dom8
            | Self::Dom11
            | Self::Dom16
            | Self::Dom22
            | Self::Dom44
            | Self::Dom88
            | Self::DominoEx
            | Self::DominoF => Mode::Domino,
            Self::FmHell
            | Self::Fskh105
            | Self::Fskh245
            | Self::FskHell
            | Self::Hell80
            | Self::HellX5
            | Self::HellX9
            | Self::Hfsk
            | Self::PskHell
            | Self::SlowHell => Mode::Hell,
            Self::FreeData
            | Self::VaraHf
            | Self::VaraSatellite
            | Self::VaraFm1200
            | Self::VaraFm9600 => Mode::Dynamic,
            Self::FsqCall
            | Self::Fst4
            | Self::Fst4w
            | Self::Ft2
            | Self::Ft4
            | Self::Js8
            | Self::Jtms
            | Self::Jtty
            | Self::Mfsk4
            | Self::Mfsk8
            | Self::Mfsk11
            | Self::Mfsk16
            | Self::Mfsk22
            | Self::Mfsk31
            | Self::Mfsk32
            | Self::Mfsk64
            | Self::Mfsk64l
            | Self::Mfsk128
            | Self::Mfsk128l
            | Self::Q65 => Mode::Mfsk,
            Self::IscatA | Self::IscatB => Mode::Iscat,
            Self::Jt4a
            | Self::Jt4b
            | Self::Jt4c
            | Self::Jt4d
            | Self::Jt4e
            | Self::Jt4f
            | Self::Jt4g => Mode::Jt4,
            Self::Jt9_1
            | Self::Jt9_2
            | Self::Jt9_5
            | Self::Jt9_10
            | Self::Jt9_30
            | Self::Jt9a
            | Self::Jt9b
            | Self::Jt9c
            | Self::Jt9d
            | Self::Jt9e
            | Self::Jt9eFast
            | Self::Jt9f
            | Self::Jt9fFast
            | Self::Jt9g
            | Self::Jt9gFast
            | Self::Jt9h
            | Self::Jt9hFast => Mode::Jt9,
            Self::Jt65a | Self::Jt65b | Self::Jt65b2 | Self::Jt65c | Self::Jt65c2 => Mode::Jt65,
            Self::Lsb | Self::Usb => Mode::Ssb,
            Self::Olivia4_125
            | Self::Olivia4_250
            | Self::Olivia8_250
            | Self::Olivia8_500
            | Self::Olivia16_500
            | Self::Olivia16_1000
            | Self::Olivia32_1000 => Mode::Olivia,
            Self::OperaBeacon | Self::OperaQso => Mode::Opera,
            Self::Pac2 | Self::Pac3 | Self::Pac4 => Mode::Pac,
            Self::Pax2 => Mode::Pax,
            Self::Pcw => Mode::Cw,
            Self::Qra64a | Self::Qra64b | Self::Qra64c | Self::Qra64d | Self::Qra64e => Mode::Qra64,
            Self::RibbitPix | Self::RibbitSms => Mode::Ofdm,
            Self::RosEme | Self::RosHf | Self::RosMf => Mode::Ros,
            Self::ScampFast | Self::ScampSlow | Self::ScampVerySlow => Mode::Fsk,
            Self::ScampOo | Self::ScampOoSlow => Mode::Mtone,
            Self::ThorM
            | Self::Thor4
            | Self::Thor5
            | Self::Thor8
            | Self::Thor11
            | Self::Thor16
            | Self::Thor22
            | Self::Thor25x4
            | Self::Thor50x1
            | Self::Thor50x2
            | Self::Thor100 => Mode::Thor,
            Self::Thrbx
            | Self::Thrbx1
            | Self::Thrbx2
            | Self::Thrbx4
            | Self::Throb1
            | Self::Throb2
            | Self::Throb4 => Mode::Thrb,
        }
    }
}
