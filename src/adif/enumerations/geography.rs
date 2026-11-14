//! Geographic codes from ADIF 3.1.8, including historical entries.
//!
//! Source: [ADIF 3.1.8, sections III.B.8 and III.B.21–23](https://www.adif.org.uk/318/ADIF_318.htm).

use strum::{EnumIter, EnumString, IntoStaticStr};

mod dxcc;
mod secondary;
mod secondary_alt;

pub use dxcc::DxccEntity;
pub use secondary::{AlaskaCounty, SecondarySubdivision};
pub use secondary_alt::{NzRegionDistrict, SecondarySubdivisionAlt};

/// Region identities used by CQ and Worked All Europe (WAE) awards.
///
/// `REGION` supplements DXCC when the award geography distinguishes regions
/// within a DXCC entity. ADIF 3.1.8 section III.B.21 publishes eight distinct
/// codes. Kosovo has three dated DXCC associations but one wire identity, `KO`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum Region {
    /// Not within a WAE or CQ region that is within a DXCC entity.
    #[strum(serialize = "NONE")]
    None,
    /// ITU Vienna; prefix 4U1V, DXCC 206, applicable to CQ and WAE.
    #[strum(serialize = "IV")]
    ItuVienna,
    /// African Italy; prefix IG9, DXCC 248, applicable to CQ.
    #[strum(serialize = "AI")]
    AfricanItaly,
    /// Sicily; prefix IT9, DXCC 248, applicable to CQ and WAE.
    #[strum(serialize = "SY")]
    Sicily,
    /// Bear Island; prefix JW/B, DXCC 259, applicable to CQ and WAE.
    #[strum(serialize = "BI")]
    BearIsland,
    /// Shetland Islands; prefix GM/S, DXCC 279, applicable to CQ and WAE.
    #[strum(serialize = "SI")]
    ShetlandIslands,
    /// Kosovo, applicable to CQ and WAE.
    ///
    /// ADIF associates prefix YU8 and DXCC 296 through 2012-09-11;
    /// prefix Z6 and DXCC 0 from 2012-09-12 through 2018-01-20; and
    /// prefix Z6 and DXCC 522 from 2018-01-21 onward.
    #[strum(serialize = "KO")]
    Kosovo,
    /// European Turkey; prefix TA1, DXCC 390, applicable to CQ and WAE.
    #[strum(serialize = "ET")]
    EuropeanTurkey,
}

impl Region {
    /// Returns the canonical ADIF region code.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}
