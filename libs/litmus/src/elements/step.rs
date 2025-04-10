use crate::utils::aliases::MaybeOwnedStr;

pub(crate) struct Steps<StepFnImpl> {
    pub(crate) metas: Vec<StepMeta>,
    pub(crate) callback: StepFnImpl,
}

impl<StepFnImpl> Steps<StepFnImpl> {
    pub(crate) fn chain_given_background<WorldImpl>(self, step: Step<impl BackgroundGivenStepFn<WorldImpl>>) -> Steps<impl BackgroundGivenStepFn<WorldImpl>>
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

        fn chain_callback<WorldImpl>(lhs: impl BackgroundGivenStepFn<WorldImpl>, rhs: impl BackgroundGivenStepFn<WorldImpl>) -> impl BackgroundGivenStepFn<WorldImpl>
        where
            WorldImpl: World,
        {
            move |world| {
                lhs(world)?;
                rhs(world)
            }
        }
    }

    pub(crate) fn chain_given<WorldImpl>(self, step: Step<impl GivenStepFn<WorldImpl>>) -> Steps<impl GivenStepFn<WorldImpl>>
    where
        StepFnImpl: GivenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        metas.push(step.meta);

        let callback = chain_callback(self.callback, step.callback);

        return Steps {
            metas,
            callback,
        };

        fn chain_callback<WorldImpl>(lhs: impl GivenStepFn<WorldImpl>, rhs: impl GivenStepFn<WorldImpl>) -> impl GivenStepFn<WorldImpl>
        where
            WorldImpl: World,
        {
            move |world| {
                lhs(world)?;
                rhs(world)
            }
        }
    }

    pub(crate) fn chain_when<WorldImpl>(self, step: Step<impl WhenStepFn<WorldImpl>>) -> Steps<impl WhenStepFn<WorldImpl>>
    where
        StepFnImpl: WhenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        metas.push(step.meta);

        let callback = chain_callback(self.callback, step.callback);

        return Steps {
            metas,
            callback,
        };

        fn chain_callback<WorldImpl>(lhs: impl WhenStepFn<WorldImpl>, rhs: impl WhenStepFn<WorldImpl>) -> impl WhenStepFn<WorldImpl>
        where
            WorldImpl: World,
        {
            move |world| {
                lhs(world)?;
                rhs(world)
            }
        }
    }

    pub(crate) fn chain_then<WorldImpl>(self, step: Step<impl ThenStepFn<WorldImpl>>) -> Steps<impl ThenStepFn<WorldImpl>>
    where
        StepFnImpl: ThenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        metas.push(step.meta);

        let callback = chain_callback(self.callback, step.callback);

        return Steps {
            metas,
            callback,
        };

        fn chain_callback<WorldImpl>(lhs: impl ThenStepFn<WorldImpl>, rhs: impl ThenStepFn<WorldImpl>) -> impl ThenStepFn<WorldImpl>
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
        let joined = self.metas
            .iter()
            .map(|meta| format!("{}", meta))
            .collect::<Vec<_>>()
            .join(STEP_DELIMITER);

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

pub trait BackgroundGivenStepFn<WorldImpl>: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static {}

impl<WorldImpl, T> BackgroundGivenStepFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait GivenStepFn<WorldImpl>: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static {}

impl<WorldImpl, T> GivenStepFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait WhenStepFn<WorldImpl>: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static {}

impl<WorldImpl, T> WhenStepFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ThenStepFn<WorldImpl>: FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static {}

impl<WorldImpl, T> ThenStepFn<WorldImpl> for T
where
    T: FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait World: Default + Send + Sync + 'static {}

impl<T> World for T
where
    T: Default + Send + Sync + 'static,
{
}
