mod aliases;
mod states;

use crate::utils::aliases::MaybeOwnedStr;

pub use self::aliases::*;
pub use self::states::*;

pub struct Scenario;

impl Scenario {
    pub fn named(description: impl Into<MaybeOwnedStr>) -> ScenarioEmptyState {
        ScenarioEmptyState {
            description: Some(description.into()),
        }
    }

    pub fn unnamed() -> ScenarioEmptyState {
        ScenarioEmptyState {
            description: None,
        }
    }
}
