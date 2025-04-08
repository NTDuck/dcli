use std::marker::PhantomData;

use crate::{elements::{GivenFn, Scenario, ScenarioFn, ThenFn, WhenFn, World}, utils::aliases::MaybeOwnedStr};
use crate::utils::extensions::InfallibleExt;

use super::{Step, StepLabel};

pub struct ScenarioThenState<GivenFnImpl, WhenFnImpl, ThenFnImpl> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) given_steps: Vec<Step<GivenFnImpl>>,
    pub(crate) when_steps: Vec<Step<WhenFnImpl>>,
    pub(crate) then_steps: Vec<Step<ThenFnImpl>>,
}

impl<GivenFnImpl, WhenFnImpl, ThenFnImpl> ScenarioThenState<GivenFnImpl, WhenFnImpl, ThenFnImpl> {
    pub fn and<WorldImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<ThenFnImpl>) -> Self
    where
        GivenFnImpl: GivenFn<WorldImpl>,
        WhenFnImpl: WhenFn<WorldImpl>,
        ThenFnImpl: ThenFn<WorldImpl>,
        WorldImpl: World,
    {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback: callback.into(),
        };

        self.with_step(step)
    }

    pub fn but<WorldImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<ThenFnImpl>) -> Self
    where
        GivenFnImpl: GivenFn<WorldImpl>,
        WhenFnImpl: WhenFn<WorldImpl>,
        ThenFnImpl: ThenFn<WorldImpl>,
        WorldImpl: World,
    {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback: callback.into(),
        };

        self.with_step(step)
    }

    fn with_step<WorldImpl>(self, step: impl Into<Step<ThenFnImpl>>) -> Self
    where
        GivenFnImpl: GivenFn<WorldImpl>,
        WhenFnImpl: WhenFn<WorldImpl>,
        ThenFnImpl: ThenFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut then_steps = self.then_steps;
        then_steps.push(step.into());

        Self {
            then_steps,
            ..self
        }
    }
}

impl<GivenFnImpl, WhenFnImpl, ThenFnImpl, ScenarioFnImpl, WorldImpl> From<ScenarioThenState<GivenFnImpl, WhenFnImpl, ThenFnImpl>> for Scenario<ScenarioFnImpl, WorldImpl>
where
    GivenFnImpl: GivenFn<WorldImpl>,
    WhenFnImpl: WhenFn<WorldImpl>,
    ThenFnImpl: ThenFn<WorldImpl>,
    ScenarioFnImpl: ScenarioFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(state: ScenarioThenState<GivenFnImpl, WhenFnImpl, ThenFnImpl>) -> Self {
        fn compute_description<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl>(
            given_steps: &[Step<GivenFnImpl>],
            when_steps: &[Step<WhenFnImpl>],
            then_steps: &[Step<ThenFnImpl>],
        ) -> MaybeOwnedStr
        where
            GivenFnImpl: GivenFn<WorldImpl>,
            WhenFnImpl: WhenFn<WorldImpl>,
            ThenFnImpl: ThenFn<WorldImpl>,
            WorldImpl: World,
        {
            given_steps
                .iter()
                .map(|step| format!("{}", step))
                .chain(
                    when_steps
                        .iter()
                        .map(|step| format!("{}", step))
                )
                .chain(
                    then_steps
                        .iter()
                        .map(|step| format!("{}", step))
                )
                .collect::<Vec<_>>()
                .join(" ")
                .into()
            }

        fn compute_callback<GivenFnImpl, WhenFnImpl, ThenFnImpl, ScenarioFnImpl, WorldImpl>(
            given_steps: &[Step<GivenFnImpl>],
            when_steps: &[Step<WhenFnImpl>],
            then_steps: &[Step<ThenFnImpl>],
        ) -> impl ScenarioFn<WorldImpl>
        where
            GivenFnImpl: GivenFn<WorldImpl>,
            WhenFnImpl: WhenFn<WorldImpl>,
            ThenFnImpl: ThenFn<WorldImpl>,
            WorldImpl: World,
        {
            move || {
                let mut world = WorldImpl::default();
    
                given_steps
                    .iter()
                    .map(|step| step.callback)
                    .try_for_each(|given| given(&mut world));
    
                when_steps
                    .iter()
                    .map(|step| step.callback)
                    .try_for_each(|when| when(&mut world));
    
                then_steps
                    .iter()
                    .map(|step| step.callback)
                    .try_for_each(|then| then(&world));
    
                Ok(())
            }
        }

        let ScenarioThenState {
            description,
            given_steps,
            when_steps,
            then_steps,
        } = state;

        Self {
            description: match description {
                Some(description) => description,
                None => compute_description(&given_steps, &when_steps, &then_steps),
            },
            callback: compute_callback(&given_steps, &when_steps, &then_steps),
            _phantom: PhantomData,
        }
    }
}
