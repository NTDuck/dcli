mod aliases;
mod states;

use std::marker::PhantomData;

use crate::utils::aliases::MaybeOwnedStr;

pub use self::aliases::*;
pub use self::states::*;

pub struct Scenario<ScenarioFnImpl, WorldImpl> {
    pub(crate) description: MaybeOwnedStr,
    pub(crate) callback: ScenarioFnImpl,
    pub(crate) _phantom: PhantomData<WorldImpl>,
}

impl<ScenarioFnImpl, WorldImpl> Scenario<ScenarioFnImpl, WorldImpl> {
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

impl<ScenarioFnImpl, WorldImpl> From<Scenario<ScenarioFnImpl, WorldImpl>> for libtest::Trial
where
    ScenarioFnImpl: ScenarioFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(scenario: Scenario<ScenarioFnImpl, WorldImpl>) -> Self {
        Self::test(scenario.description, scenario.callback)
    }
}
