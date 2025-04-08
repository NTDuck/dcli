use crate::elements::GivenFn;
use crate::elements::WhenFn;
use crate::elements::World;
use crate::utils::aliases::MaybeOwnedStr;

use super::Step;
use super::StepLabel;
use super::ScenarioWhenState;

pub struct ScenarioGivenState<GivenFnImpl> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) given_steps: Vec<Step<GivenFnImpl>>,
}

impl<GivenFnImpl> ScenarioGivenState<GivenFnImpl> {
    pub fn and<WorldImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<GivenFnImpl>) -> Self
    where
        GivenFnImpl: GivenFn<WorldImpl>,
        WorldImpl: World,
    {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback: callback.into(),
        };

        self.with_step(step)
    }

    pub fn but<WorldImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<GivenFnImpl>) -> Self
    where
        GivenFnImpl: GivenFn<WorldImpl>,
        WorldImpl: World,
    {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback: callback.into(),
        };

        self.with_step(step)
    }

    pub fn when<WhenFnImpl, WorldImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<WhenFnImpl>) -> ScenarioWhenState<GivenFnImpl, WhenFnImpl>
    where
        GivenFnImpl: GivenFn<WorldImpl>,
        WhenFnImpl: WhenFn<WorldImpl>,
        WorldImpl: World,
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
        }
    }

    fn with_step<WorldImpl>(self, step: impl Into<Step<GivenFnImpl>>) -> Self
    where
        GivenFnImpl: GivenFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut given_steps = self.given_steps;
        given_steps.push(step.into());

        Self {
            given_steps,
            ..self
        }
    }
}
