use crate::{elements::{GivenFn, World}, utils::aliases::MaybeOwnedStr};

use super::{ScenarioGivenState, Step, StepLabel};

pub struct ScenarioEmptyState {
    pub(crate) description: Option<MaybeOwnedStr>,
}

impl ScenarioEmptyState {
    pub fn given<GivenFnImpl, WorldImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<GivenFnImpl>) -> ScenarioGivenState<GivenFnImpl>
    where
        GivenFnImpl: GivenFn<WorldImpl>,
        WorldImpl: World,
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
        }
    }
}
