//! India subdivisions in ADIF III.B.12, DXCC entity 324.

use strum::{EnumIter, EnumString, IntoStaticStr};

/// A state or Union territory in ADIF's India group.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Andhra Pradesh (state).
    #[strum(serialize = "AP")]
    AndhraPradesh,
    /// Arunāchal Pradesh (state).
    #[strum(serialize = "AR")]
    ArunachalPradesh,
    /// Assam (state).
    #[strum(serialize = "AS")]
    Assam,
    /// Bihār (state).
    #[strum(serialize = "BR")]
    Bihar,
    /// Chandīgarh (Union territory).
    #[strum(serialize = "CH")]
    Chandigarh,
    /// Chhattīsgarh (state).
    #[strum(serialize = "CG")]
    Chhattisgarh,
    /// Damān and Diu (Union territory).
    #[strum(serialize = "DD")]
    DamanAndDiu,
    /// Delhi (Union territory).
    #[strum(serialize = "DL")]
    Delhi,
    /// Dādra and Nagar Haveli (Union territory).
    #[strum(serialize = "DN")]
    DadraAndNagarHaveli,
    /// Goa (state).
    #[strum(serialize = "GA")]
    Goa,
    /// Gujarāt (state).
    #[strum(serialize = "GJ")]
    Gujarat,
    /// Haryāna (state).
    #[strum(serialize = "HR")]
    Haryana,
    /// Himāchal Pradesh (state).
    #[strum(serialize = "HP")]
    HimachalPradesh,
    /// Jammu and Kashmīr (Union territory).
    #[strum(serialize = "JK")]
    JammuAndKashmir,
    /// Jhārkhand (state).
    #[strum(serialize = "JH")]
    Jharkhand,
    /// Karnātaka (state).
    #[strum(serialize = "KA")]
    Karnataka,
    /// Kerala (state).
    #[strum(serialize = "KL")]
    Kerala,
    /// Ladākh (Union territory).
    #[strum(serialize = "LA")]
    Ladakh,
    /// Madhya Pradesh (state).
    #[strum(serialize = "MP")]
    MadhyaPradesh,
    /// Mahārāshtra (state).
    #[strum(serialize = "MH")]
    Maharashtra,
    /// Manipur (state).
    #[strum(serialize = "MN")]
    Manipur,
    /// Meghālaya (state).
    #[strum(serialize = "ML")]
    Meghalaya,
    /// Mizoram (state).
    #[strum(serialize = "MZ")]
    Mizoram,
    /// Nāgāland (state).
    #[strum(serialize = "NL")]
    Nagaland,
    /// Odisha (state).
    #[strum(serialize = "OD")]
    Odisha,
    /// Puducherry (Union territory).
    #[strum(serialize = "PY")]
    Puducherry,
    /// Punjab (state).
    #[strum(serialize = "PB")]
    Punjab,
    /// Rājasthān (state).
    #[strum(serialize = "RJ")]
    Rajasthan,
    /// Sikkim (state).
    #[strum(serialize = "SK")]
    Sikkim,
    /// Tamil Nādu (state).
    #[strum(serialize = "TN")]
    TamilNadu,
    /// Telangāna (state).
    #[strum(serialize = "TG")]
    Telangana,
    /// Tripura (state).
    #[strum(serialize = "TR")]
    Tripura,
    /// Uttar Pradesh (state).
    #[strum(serialize = "UP")]
    UttarPradesh,
    /// Uttarākhand (state).
    #[strum(serialize = "UK")]
    Uttarakhand,
    /// West Bengal (state).
    #[strum(serialize = "WB")]
    WestBengal,
}
