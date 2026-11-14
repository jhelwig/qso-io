//! Radio operation and award reference types defined by ADIF 3.1.8.
//!
//! These maintained types follow the [ADIF enumeration tables](https://adif.org/318/ADIF_318.htm#Enumerations).
//! Parsing accepts ASCII case differences, but preserves distinctions such as
//! embedded spaces, punctuation, and legacy import-only values.

mod band;
mod contest;
mod credit;
mod mode;
mod submode;

pub use band::Band;
pub use contest::ContestId;
pub use credit::{AwardSponsor, Credit};
pub use mode::Mode;
pub use submode::Submode;

#[cfg(test)]
mod tests {
    use super::{ContestId, Mode, Submode};

    #[test]
    fn legacy_mode_import_preserves_its_distinction_from_a_submode() {
        let legacy = "c4fm"
            .parse::<Mode>()
            .expect("C4FM should identify its historical Mode entry");
        let submode = "c4fm"
            .parse::<Submode>()
            .expect("C4FM should identify its current Submode entry");

        assert_eq!(legacy, Mode::C4fm);
        assert!(legacy.is_import_only());
        assert_eq!(submode.mode(), Mode::DigitalVoice);
        assert!(!submode.mode().is_import_only());
    }

    #[test]
    fn historical_contest_aliases_remain_distinct_on_import() {
        let legacy = "virginia qso party"
            .parse::<ContestId>()
            .expect("the published contest identifier should retain its own identity");
        let current = "va-qso-party"
            .parse::<ContestId>()
            .expect("the published contest identifier should retain its own identity");

        assert_ne!(legacy, current);
        assert!(legacy.is_import_only());
        assert!(!current.is_import_only());
        assert_eq!(legacy.as_str(), "VIRGINIA QSO PARTY");
    }
}
