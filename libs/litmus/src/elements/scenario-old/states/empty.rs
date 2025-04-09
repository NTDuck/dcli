use std::marker::PhantomData;

use crate::{elements::{GivenFn, World}, utils::aliases::MaybeOwnedStr};

use super::{ScenarioGivenState, Step, StepLabel};

pub struct ScenarioEmptyState<WorldImpl> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) world: PhantomData<WorldImpl>,
}

impl<WorldImpl> ScenarioEmptyState<WorldImpl>
where
    WorldImpl: World,
{
    pub fn given<GivenFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: GivenFnImpl) -> ScenarioGivenState<GivenFnImpl, WorldImpl>
    where
        GivenFnImpl: GivenFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback: callback.into(),
        };

        let given_steps = vec![step];

        ScenarioGivenState {
            description: self.description,
            given_steps,
            world: self.world,
        }
    }
}
