mod aliases;
mod states;

use std::marker::PhantomData;

use crate::utils::aliases::MaybeOwnedStr;

pub use self::aliases::*;
pub use self::states::*;

pub struct Scenario<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl> {
    pub(crate) description: MaybeOwnedStr,
    pub(crate) given_steps: Vec<Step<GivenFnImpl>>,
    pub(crate) when_steps: Vec<Step<WhenFnImpl>>,
    pub(crate) then_steps: Vec<Step<ThenFnImpl>>,
    pub(crate) _phantom: PhantomData<WorldImpl>,
}

impl<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl> Scenario<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl> {
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

impl<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl> From<Scenario<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl>> for libtest::Trial
where
    GivenFnImpl: GivenFn<WorldImpl>,
    WhenFnImpl: WhenFn<WorldImpl>,
    ThenFnImpl: ThenFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(scenario: Scenario<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl>) -> Self {
        Self::test(scenario.description, move || {
            let mut world = WorldImpl::default();

            scenario.given_steps
                .into_iter()
                .map(|step| step.callback)
                .try_for_each(|given| given(&mut world))?;

            scenario.when_steps
                .into_iter()
                .map(|step| step.callback)
                .try_for_each(|when| when(&mut world))?;

            scenario.then_steps
                .into_iter()
                .map(|step| step.callback)
                .try_for_each(|then| then(&world))?;

            Ok(())
        })
    }
}
