use crate::utils::aliases::MaybeOwnedStr;

pub(crate) struct Steps<StepFnImpl> {
    pub(crate) metas: Vec<StepMeta>,
    pub(crate) callback: StepFnImpl,
}

impl<StepFnImpl> Steps<StepFnImpl> {
    pub(crate) fn chain_background_given_step<WorldImpl>(self, step: Step<impl BackgroundGivenStepFn<WorldImpl>>) -> Steps<impl BackgroundGivenStepFn<WorldImpl>>
    where
        StepFnImpl: BackgroundGivenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        metas.push(step.meta);

        let callback = self.callback.chain(step.callback);

        Steps {
            metas,
            callback,
        }
    }

    pub(crate) fn chain_scenario_given_step<WorldImpl>(self, step: Step<impl ScenarioGivenStepFn<WorldImpl>>) -> Steps<impl ScenarioGivenStepFn<WorldImpl>>
    where
        StepFnImpl: ScenarioGivenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        metas.push(step.meta);

        let callback = self.callback.chain(step.callback);

        Steps {
            metas,
            callback,
        }
    }

    pub(crate) fn chain_scenario_when_step<WorldImpl>(self, step: Step<impl ScenarioWhenStepFn<WorldImpl>>) -> Steps<impl ScenarioWhenStepFn<WorldImpl>>
    where
        StepFnImpl: ScenarioWhenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        metas.push(step.meta);

        let callback = self.callback.chain(step.callback);

        Steps {
            metas,
            callback,
        }
    }

    pub(crate) fn chain_scenario_then_step<WorldImpl>(self, step: Step<impl ScenarioThenStepFn<WorldImpl>>) -> Steps<impl ScenarioThenStepFn<WorldImpl>>
    where
        StepFnImpl: ScenarioThenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        metas.push(step.meta);

        let callback = self.callback.chain(step.callback);

        Steps {
            metas,
            callback,
        }
    }
}

impl<StepFnImpl> From<Step<StepFnImpl>> for Steps<StepFnImpl> {
    fn from(step: Step<StepFnImpl>) -> Self {
        Self {
            metas: vec![step.meta],
            callback: step.callback,
        }
    }
}

impl<StepFnImpl> std::fmt::Display for Steps<StepFnImpl> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.metas.iter().map(|meta| format!("{}", meta)).collect::<Vec<_>>().join(STEP_DELIMITER);

        write!(formatter, "{}", joined)
    }
}

pub(crate) struct Step<StepFnImpl> {
    pub(crate) meta: StepMeta,
    pub(crate) callback: StepFnImpl,
}

pub(crate) struct StepMeta {
    pub(crate) label: StepLabel,
    pub(crate) description: MaybeOwnedStr,
}

impl std::fmt::Display for StepMeta {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} {}", self.label, self.description)
    }
}

#[allow(dead_code)]
#[derive(strum::Display)]
pub(crate) enum StepLabel {
    // Flavor A
    Suite,
    Context,
    Example,

    // Flavor B
    Describe,
    Specify,
    It,

    // Flavor C
    Given,
    When,
    Then,

    // Succession
    And,
    But,
}

pub(crate) const STEP_DELIMITER: &str = " | ";

pub trait BackgroundGivenStepFn<WorldImpl>:
    Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
    fn chain(self, other: impl BackgroundGivenStepFn<WorldImpl>) -> impl BackgroundGivenStepFn<WorldImpl>
    where
        Self: Sized,
        WorldImpl: World,
    {
        move |world| {
            (self)(world)?;
            (other)(world)?;

            Ok(())
        }
    }
}

impl<T, WorldImpl> BackgroundGivenStepFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ScenarioGivenStepFn<WorldImpl>:
    FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
    fn chain(self, other: impl ScenarioGivenStepFn<WorldImpl>) -> impl ScenarioGivenStepFn<WorldImpl>
    where
        Self: Sized,
        WorldImpl: World,
    {
        move |world| {
            (self)(world)?;
            (other)(world)?;

            Ok(())
        }
    }
}

impl<T, WorldImpl> ScenarioGivenStepFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ScenarioWhenStepFn<WorldImpl>:
    FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
    fn chain(self, other: impl ScenarioWhenStepFn<WorldImpl>) -> impl ScenarioWhenStepFn<WorldImpl>
    where
        Self: Sized,
        WorldImpl: World,
    {
        move |world| {
            (self)(world)?;
            (other)(world)?;

            Ok(())
        }
    }
}

impl<T, WorldImpl> ScenarioWhenStepFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ScenarioThenStepFn<WorldImpl>:
    FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
    fn chain(self, other: impl ScenarioThenStepFn<WorldImpl>) -> impl ScenarioThenStepFn<WorldImpl>
    where
        Self: Sized,
        WorldImpl: World,
    {
        move |world| {
            (self)(world)?;
            (other)(world)?;

            Ok(())
        }
    }
}

impl<T, WorldImpl> ScenarioThenStepFn<WorldImpl> for T
where
    T: FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait World: Default + Send + Sync + 'static {}

impl<T> World for T where T: Default + Send + Sync + 'static {}
