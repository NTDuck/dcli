use std::marker::PhantomData;

use crate::utils::aliases::MaybeOwnedStr;

use super::Step;
use super::StepBody;
use super::StepLabel;
use super::WhenStepBody;
use super::ScenarioWhenState;
use super::World;

pub struct ScenarioGivenState<GivenBody, World> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) given_steps: Vec<Step<GivenBody>>,
    pub(crate) _phantom: PhantomData<World>,
}

impl<GivenBodyImpl, WorldImpl> ScenarioGivenState<GivenBodyImpl, WorldImpl>
where
    GivenBodyImpl: GivenBody<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(self, description: impl Into<MaybeOwnedStr>, body: GivenBodyImpl) -> Self {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            body,
        };

        self.with_step(step)
    }

    pub fn but(self, description: impl Into<MaybeOwnedStr>, body: GivenBodyImpl) -> Self {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            body,
        };

        self.with_step(step)
    }

    pub fn when<WhenBody>(self, description: impl Into<MaybeOwnedStr>, body: WhenBody) -> ScenarioWhenState<GivenBodyImpl, WhenBody, WorldImpl>
    where
        WhenBody: WhenStepBody<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::When,
            description: description.into(),
            body,
        };

        let when_steps = vec![step];

        ScenarioWhenState {
            description: self.description,
            given_steps: self.given_steps,
            when_steps,
            _phantom: self._phantom,
        }
    }

    fn with_step(self, step: impl Into<Step<GivenBodyImpl>>) -> Self {
        let mut given_steps = self.given_steps;
        given_steps.push(step.into());

        Self {
            given_steps,
            ..self
        }
    }
}

pub(crate) trait GivenBody<WorldImpl>: StepBody + FnOnce(&mut WorldImpl) {}

impl<T, WorldImpl> GivenBody<WorldImpl> for T
where
    T: StepBody + FnOnce(&mut WorldImpl),
    WorldImpl: World,
{
}
