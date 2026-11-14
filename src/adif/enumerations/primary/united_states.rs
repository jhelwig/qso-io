//! United States subdivisions in ADIF III.B.12, DXCC entity 291.

use strum::{EnumIter, EnumString, IntoStaticStr};

/// A primary subdivision in ADIF's United States group.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum PrimarySubdivision {
    /// Connecticut.
    #[strum(serialize = "CT")]
    Connecticut,
    /// Maine.
    #[strum(serialize = "ME")]
    Maine,
    /// Massachusetts.
    #[strum(serialize = "MA")]
    Massachusetts,
    /// New Hampshire.
    #[strum(serialize = "NH")]
    NewHampshire,
    /// Rhode Island.
    #[strum(serialize = "RI")]
    RhodeIsland,
    /// Vermont.
    #[strum(serialize = "VT")]
    Vermont,
    /// New Jersey.
    #[strum(serialize = "NJ")]
    NewJersey,
    /// New York.
    #[strum(serialize = "NY")]
    NewYork,
    /// Delaware.
    #[strum(serialize = "DE")]
    Delaware,
    /// District of Columbia.
    #[strum(serialize = "DC")]
    DistrictOfColumbia,
    /// Maryland.
    #[strum(serialize = "MD")]
    Maryland,
    /// Pennsylvania.
    #[strum(serialize = "PA")]
    Pennsylvania,
    /// Alabama.
    #[strum(serialize = "AL")]
    Alabama,
    /// Florida.
    #[strum(serialize = "FL")]
    Florida,
    /// Georgia.
    #[strum(serialize = "GA")]
    Georgia,
    /// Kentucky.
    #[strum(serialize = "KY")]
    Kentucky,
    /// North Carolina.
    #[strum(serialize = "NC")]
    NorthCarolina,
    /// South Carolina.
    #[strum(serialize = "SC")]
    SouthCarolina,
    /// Tennessee.
    #[strum(serialize = "TN")]
    Tennessee,
    /// Virginia.
    #[strum(serialize = "VA")]
    Virginia,
    /// Arkansas.
    #[strum(serialize = "AR")]
    Arkansas,
    /// Louisiana.
    #[strum(serialize = "LA")]
    Louisiana,
    /// Mississippi.
    #[strum(serialize = "MS")]
    Mississippi,
    /// New Mexico.
    #[strum(serialize = "NM")]
    NewMexico,
    /// Oklahoma.
    #[strum(serialize = "OK")]
    Oklahoma,
    /// Texas.
    #[strum(serialize = "TX")]
    Texas,
    /// California.
    #[strum(serialize = "CA")]
    California,
    /// Arizona.
    #[strum(serialize = "AZ")]
    Arizona,
    /// Idaho.
    #[strum(serialize = "ID")]
    Idaho,
    /// Montana.
    #[strum(serialize = "MT")]
    Montana,
    /// Nevada.
    #[strum(serialize = "NV")]
    Nevada,
    /// Oregon.
    #[strum(serialize = "OR")]
    Oregon,
    /// Utah.
    #[strum(serialize = "UT")]
    Utah,
    /// Washington.
    #[strum(serialize = "WA")]
    Washington,
    /// Wyoming.
    #[strum(serialize = "WY")]
    Wyoming,
    /// Michigan.
    #[strum(serialize = "MI")]
    Michigan,
    /// Ohio.
    #[strum(serialize = "OH")]
    Ohio,
    /// West Virginia.
    #[strum(serialize = "WV")]
    WestVirginia,
    /// Illinois.
    #[strum(serialize = "IL")]
    Illinois,
    /// Indiana.
    #[strum(serialize = "IN")]
    Indiana,
    /// Wisconsin.
    #[strum(serialize = "WI")]
    Wisconsin,
    /// Colorado.
    #[strum(serialize = "CO")]
    Colorado,
    /// Iowa.
    #[strum(serialize = "IA")]
    Iowa,
    /// Kansas.
    #[strum(serialize = "KS")]
    Kansas,
    /// Minnesota.
    #[strum(serialize = "MN")]
    Minnesota,
    /// Missouri.
    #[strum(serialize = "MO")]
    Missouri,
    /// Nebraska.
    #[strum(serialize = "NE")]
    Nebraska,
    /// North Dakota.
    #[strum(serialize = "ND")]
    NorthDakota,
    /// South Dakota.
    #[strum(serialize = "SD")]
    SouthDakota,
}
