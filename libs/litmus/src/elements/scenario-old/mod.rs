mod aliases;
mod states;

use std::marker::PhantomData;

use crate::utils::aliases::MaybeOwnedStr;

pub use self::aliases::*;
pub use self::states::*;

pub struct Scenario<WorldImpl> {
    pub(crate) world: PhantomData<WorldImpl>,
}

impl<WorldImpl> Scenario<WorldImpl>
where
    WorldImpl: World,
{
    pub fn named(description: impl Into<MaybeOwnedStr>) -> ScenarioEmptyState<WorldImpl> {
        ScenarioEmptyState {
            description: Some(description.into()),
            world: PhantomData,
        }
    }

    pub fn unnamed() -> ScenarioEmptyState<WorldImpl> {
        ScenarioEmptyState {
            description: None,
            world: PhantomData,
        }
    }
}
