use crate::utils::aliases::MaybeOwnedStr;

pub(crate) struct Steps<StepFnImpl> {
    pub(crate) metas: Vec<StepMeta>,
    pub(crate) callback: StepFnImpl,
}

impl<StepFnImpl> Steps<StepFnImpl> {
    pub(crate) fn chain_background_given<WorldImpl>(
        self,
        step: Step<impl BackgroundGivenStepFn<WorldImpl>>,
    ) -> Steps<impl BackgroundGivenStepFn<WorldImpl>>
    where
        StepFnImpl: BackgroundGivenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        metas.push(step.meta);

        let callback = chain_callback(self.callback, step.callback);

        return Steps {
            metas,
            callback,
        };

        fn chain_callback<WorldImpl>(
            lhs: impl BackgroundGivenStepFn<WorldImpl>,
            rhs: impl BackgroundGivenStepFn<WorldImpl>,
        ) -> impl BackgroundGivenStepFn<WorldImpl>
        where
            WorldImpl: World,
        {
            move |world| {
                lhs(world)?;
                rhs(world)
            }
        }
    }

    pub(crate) fn chain_scenario_given<WorldImpl>(
        self,
        step: Step<impl ScenarioGivenStepFn<WorldImpl>>,
    ) -> Steps<impl ScenarioGivenStepFn<WorldImpl>>
    where
        StepFnImpl: ScenarioGivenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        metas.push(step.meta);

        let callback = chain_callback(self.callback, step.callback);

        return Steps {
            metas,
            callback,
        };

        fn chain_callback<WorldImpl>(
            lhs: impl ScenarioGivenStepFn<WorldImpl>,
            rhs: impl ScenarioGivenStepFn<WorldImpl>,
        ) -> impl ScenarioGivenStepFn<WorldImpl>
        where
            WorldImpl: World,
        {
            move |world| {
                lhs(world)?;
                rhs(world)
            }
        }
    }

    pub(crate) fn chain_scenario_when<WorldImpl>(
        self,
        step: Step<impl ScenarioWhenStepFn<WorldImpl>>,
    ) -> Steps<impl ScenarioWhenStepFn<WorldImpl>>
    where
        StepFnImpl: ScenarioWhenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        metas.push(step.meta);

        let callback = chain_callback(self.callback, step.callback);

        return Steps {
            metas,
            callback,
        };

        fn chain_callback<WorldImpl>(
            lhs: impl ScenarioWhenStepFn<WorldImpl>,
            rhs: impl ScenarioWhenStepFn<WorldImpl>,
        ) -> impl ScenarioWhenStepFn<WorldImpl>
        where
            WorldImpl: World,
        {
            move |world| {
                lhs(world)?;
                rhs(world)
            }
        }
    }

    pub(crate) fn chain_scenario_then<WorldImpl>(
        self,
        step: Step<impl ScenarioThenStepFn<WorldImpl>>,
    ) -> Steps<impl ScenarioThenStepFn<WorldImpl>>
    where
        StepFnImpl: ScenarioThenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        metas.push(step.meta);

        let callback = chain_callback(self.callback, step.callback);

        return Steps {
            metas,
            callback,
        };

        fn chain_callback<WorldImpl>(
            lhs: impl ScenarioThenStepFn<WorldImpl>,
            rhs: impl ScenarioThenStepFn<WorldImpl>,
        ) -> impl ScenarioThenStepFn<WorldImpl>
        where
            WorldImpl: World,
        {
            move |world| {
                lhs(world)?;
                rhs(world)
            }
        }
    }
}

pub(crate) struct Step<StepFnImpl> {
    pub(crate) meta: StepMeta,
    pub(crate) callback: StepFnImpl,
}

impl<StepFnImpl> std::fmt::Display for Steps<StepFnImpl> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.metas.iter().map(|meta| format!("{}", meta)).collect::<Vec<_>>().join(STEP_DELIMITER);

        write!(formatter, "{}", joined)
    }
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

#[derive(strum::Display)]
pub(crate) enum StepLabel {
    Given,
    When,
    Then,
    And,
    But,
}

pub(crate) const STEP_DELIMITER: &str = " | ";

pub trait BackgroundGivenStepFn<WorldImpl>:
    Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
}

impl<WorldImpl, T> BackgroundGivenStepFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ScenarioGivenStepFn<WorldImpl>:
    FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
}

impl<WorldImpl, T> ScenarioGivenStepFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ScenarioWhenStepFn<WorldImpl>:
    FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
}

impl<WorldImpl, T> ScenarioWhenStepFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ScenarioThenStepFn<WorldImpl>:
    FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
}

impl<WorldImpl, T> ScenarioThenStepFn<WorldImpl> for T
where
    T: FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait World: Default + Send + Sync + 'static {}

impl<T> World for T where T: Default + Send + Sync + 'static {}
