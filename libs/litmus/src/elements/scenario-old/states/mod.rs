mod empty;
mod given;
mod when;
mod then;

pub use self::empty::*;
pub use self::given::*;
pub use self::when::*;
pub use self::then::*;

use crate::utils::aliases::MaybeOwnedStr;

pub(crate) struct Step<FnImpl> {
    pub label: StepLabel,
    pub description: MaybeOwnedStr,
    pub callback: FnImpl,
}

impl<FnImpl> std::fmt::Display for Step<FnImpl> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} {}", self.label, self.description)
    }
}

#[derive(strum::Display)]
pub(crate) enum StepLabel {
    Given,
    When,
    Then,
    And,
    But,
}
