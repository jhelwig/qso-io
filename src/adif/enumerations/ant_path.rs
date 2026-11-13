use strum::{EnumIter, EnumString, IntoStaticStr};

/// ADIF antenna-path codes describing how the signal propagated.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(ascii_case_insensitive)]
pub enum AntPath {
    /// Grayline
    #[strum(serialize = "G")]
    G,
    /// Other
    #[strum(serialize = "O")]
    O,
    /// Short path
    #[strum(serialize = "S")]
    S,
    /// Long path
    #[strum(serialize = "L")]
    L,
}

impl AntPath {
    /// Returns the canonical ADIF antenna-path code.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}
