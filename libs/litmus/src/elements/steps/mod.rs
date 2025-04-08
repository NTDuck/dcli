mod given;
mod when;
mod then;

pub use self::given::*;
pub use self::when::*;
pub use self::then::*;

use crate::utils::aliases::MaybeOwnedStr;

pub(crate) struct Step<Body> {
    pub label: StepLabel,
    pub description: MaybeOwnedStr,
    pub body: Body,
}

impl<Body> std::fmt::Display for Step<Body> {
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

pub(crate) trait StepBody: Send + 'static {}

impl<T> StepBody for T
where
    T: Send + 'static
{
}

pub(crate) trait World: Default {}

impl<T> World for T
where
    T: Default,
{
}