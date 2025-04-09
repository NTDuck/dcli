use std::marker::PhantomData;

use crate::{elements::{GivenFn, ThenFn, WhenFn, World}, utils::aliases::MaybeOwnedStr};

use super::{Step, StepLabel};

pub struct ScenarioThenState<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) given_steps: Vec<Step<GivenFnImpl>>,
    pub(crate) when_steps: Vec<Step<WhenFnImpl>>,
    pub(crate) then_steps: Vec<Step<ThenFnImpl>>,
    pub(crate) world: PhantomData<WorldImpl>,
}

impl<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl> ScenarioThenState<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl>
where
    GivenFnImpl: GivenFn<WorldImpl>,
    WhenFnImpl: WhenFn<WorldImpl>,
    ThenFnImpl: ThenFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(self, description: impl Into<MaybeOwnedStr>, callback: ThenFnImpl) -> Self {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback: callback.into(),
        };

        self.with_step(step)
    }

    pub fn but(self, description: impl Into<MaybeOwnedStr>, callback: ThenFnImpl) -> Self {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback: callback.into(),
        };

        self.with_step(step)
    }

    fn with_step(self, step: Step<ThenFnImpl>) -> Self {
        let mut then_steps = self.then_steps;
        then_steps.push(step);

        Self {
            then_steps,
            ..self
        }
    }
}

impl<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl> From<ScenarioThenState<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl>> for libtest::Trial
where
    GivenFnImpl: GivenFn<WorldImpl>,
    WhenFnImpl: WhenFn<WorldImpl>,
    ThenFnImpl: ThenFn<WorldImpl>,
    WorldImpl: World,
{    
    fn from(scenario: ScenarioThenState<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl>) -> Self {
        let ScenarioThenState {
            description,
            given_steps,
            when_steps,
            then_steps,
            ..
        } = scenario;

        let description = match description {
            Some(description) => description,
            None => compute_description(&given_steps, &when_steps, &then_steps),
        };
        let callback = compute_callback(given_steps, when_steps, then_steps);

        return libtest::Trial::test(description, callback);

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
                .join(" | ")
                .into()
        }

        fn compute_callback<GivenFnImpl, WhenFnImpl, ThenFnImpl, WorldImpl>(
            given_steps: Vec<Step<GivenFnImpl>>,
            when_steps: Vec<Step<WhenFnImpl>>,
            then_steps: Vec<Step<ThenFnImpl>>,
        ) -> impl FnOnce() -> Result<(), libtest::Failed> + Send + 'static
        where
            GivenFnImpl: GivenFn<WorldImpl>,
            WhenFnImpl: WhenFn<WorldImpl>,
            ThenFnImpl: ThenFn<WorldImpl>,
            WorldImpl: World,
        {
            move || {
                let mut world = WorldImpl::default();
    
                given_steps
                    .into_iter()
                    .map(|step| step.callback)
                    .try_for_each(|given| given(&mut world))?;
    
                when_steps
                    .into_iter()
                    .map(|step| step.callback)
                    .try_for_each(|when| when(&mut world))?;
    
                then_steps
                    .into_iter()
                    .map(|step| step.callback)
                    .try_for_each(|then| then(&world))?;
    
                Ok(())
            }
        }
    }
}
