use std::{borrow::Cow, marker::PhantomData};

use crate::utils::aliases::MaybeOwnedStr;

use super::{ScenarioThenState, Step, StepBody, World};

pub struct ScenarioWhenState<GivenBody, WhenBody, World> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) given_steps: Vec<Step<GivenBody>>,
    pub(crate) when_steps: Vec<Step<WhenBody>>,
    pub(crate) _phantom: PhantomData<World>,
}

impl<GivenBody, WhenBody, World> ScenarioWhenState<GivenBody, WhenBody, World>
where

    World: World,
{
}

pub(crate) trait WhenStepBody<World>: StepBody + FnOnce(&mut World) {}

impl<T, World> WhenStepBody<World> for T
where
    T: StepBody + FnOnce(&mut World),
{
}
