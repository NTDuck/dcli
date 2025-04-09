use std::marker::PhantomData;

use crate::{elements::{GivenFn, ThenFn, WhenFn, World}, utils::aliases::MaybeOwnedStr};

use super::{ScenarioThenState, Step, StepLabel};

pub struct ScenarioWhenState<GivenFnImpl, WhenFnImpl, WorldImpl> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) given_steps: Vec<Step<GivenFnImpl>>,
    pub(crate) when_steps: Vec<Step<WhenFnImpl>>,
    pub(crate) world: PhantomData<WorldImpl>,
}

impl<GivenFnImpl, WhenFnImpl, WorldImpl> ScenarioWhenState<GivenFnImpl, WhenFnImpl, WorldImpl>
where
    GivenFnImpl: GivenFn<WorldImpl>,
    WhenFnImpl: WhenFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(self, description: impl Into<MaybeOwnedStr>, callback: WhenFnImpl) -> Self {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback: callback.into(),
        };

        self.with_step(step)
    }

    pub fn but(self, description: impl Into<MaybeOwnedStr>, callback: WhenFnImpl) -> Self {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback: callback.into(),
        };

        self.with_step(step)
    }

    pub fn then<ThenFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: ThenFnImpl) -> ScenarioThenState<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl>
    where
        ThenFnImpl: ThenFn<WorldImpl>,
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
            world: self.world,
        }
    }

    fn with_step(self, step: Step<WhenFnImpl>) -> Self {
        let mut when_steps = self.when_steps;
        when_steps.push(step);

        Self {
            when_steps,
            ..self
        }
    }
}
