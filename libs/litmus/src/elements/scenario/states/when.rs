use crate::{elements::{GivenFn, ThenFn, WhenFn, World}, utils::aliases::MaybeOwnedStr};

use super::{ScenarioThenState, Step, StepLabel};

pub struct ScenarioWhenState<GivenFnImpl, WhenFnImpl> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) given_steps: Vec<Step<GivenFnImpl>>,
    pub(crate) when_steps: Vec<Step<WhenFnImpl>>,
}

impl<GivenFnImpl, WhenFnImpl> ScenarioWhenState<GivenFnImpl, WhenFnImpl> {
    pub fn and<WorldImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<WhenFnImpl>) -> Self
    where
        GivenFnImpl: GivenFn<WorldImpl>,
        WhenFnImpl: WhenFn<WorldImpl>,
        WorldImpl: World,
    {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback: callback.into(),
        };

        self.with_step(step)
    }

    pub fn but<WorldImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<WhenFnImpl>) -> Self
    where
        GivenFnImpl: GivenFn<WorldImpl>,
        WhenFnImpl: WhenFn<WorldImpl>,
        WorldImpl: World,
    {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback: callback.into(),
        };

        self.with_step(step)
    }

    pub fn then<ThenFnImpl, WorldImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<ThenFnImpl>) -> ScenarioThenState<GivenFnImpl, WhenFnImpl, ThenFnImpl>
    where
        GivenFnImpl: GivenFn<WorldImpl>,
        WhenFnImpl: WhenFn<WorldImpl>,
        ThenFnImpl: ThenFn<WorldImpl>,
        WorldImpl: World,
    {
        let step = Step {
            label: StepLabel::Then,
            description: description.into(),
            callback: callback.into(),
        };

        let then_steps = vec![step];

        ScenarioThenState {
            description: self.description,
            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps,
        }
    }

    fn with_step<WorldImpl>(self, step: impl Into<Step<WhenFnImpl>>) -> Self
    where
        GivenFnImpl: GivenFn<WorldImpl>,
        WhenFnImpl: WhenFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut when_steps = self.when_steps;
        when_steps.push(step.into());

        Self {
            when_steps,
            ..self
        }
    }
}
