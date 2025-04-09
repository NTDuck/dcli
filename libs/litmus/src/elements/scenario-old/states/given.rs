use std::marker::PhantomData;

use crate::elements::GivenFn;
use crate::elements::WhenFn;
use crate::elements::World;
use crate::utils::aliases::MaybeOwnedStr;

use super::Step;
use super::StepLabel;
use super::ScenarioWhenState;

pub struct ScenarioGivenState<GivenFnImpl, WorldImpl> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) given_steps: Vec<Step<GivenFnImpl>>,
    pub(crate) world: PhantomData<WorldImpl>,
}

impl<GivenFnImpl, WorldImpl> ScenarioGivenState<GivenFnImpl, WorldImpl>
where
    GivenFnImpl: GivenFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(self, description: impl Into<MaybeOwnedStr>, callback: GivenFnImpl) -> Self {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback: callback.into(),
        };

        self.with_step(step)
    }

    pub fn but(self, description: impl Into<MaybeOwnedStr>, callback: GivenFnImpl) -> Self {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback: callback.into(),
        };

        self.with_step(step)
    }

    pub fn when<WhenFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: WhenFnImpl) -> ScenarioWhenState<GivenFnImpl, WhenFnImpl, WorldImpl>
    where
        WhenFnImpl: WhenFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::When,
            description: description.into(),
            callback: callback.into(),
        };

        let when_steps = vec![step];

        ScenarioWhenState {
            description: self.description,
            given_steps: self.given_steps,
            when_steps,
            world: self.world,
        }
    }

    fn with_step(self, step: Step<GivenFnImpl>) -> Self {
        let mut given_steps = self.given_steps;
        given_steps.push(step);

        Self {
            given_steps,
            ..self
        }
    }
}
