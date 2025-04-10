use crate::utils::aliases::MaybeOwnedStr;

pub(crate) struct Steps<StepFnImpl> {
    pub(crate) metas: Vec<StepMeta>,
    pub(crate) callback: StepFnImpl,
}

impl<StepFnImpl> Steps<StepFnImpl> {
    pub fn with<'w, OtherStepFnImpl, WorldImpl>(self, step: Step<OtherStepFnImpl>) -> Steps<impl FnOnce(&'w mut WorldImpl) -> Result<(), libtest::Failed>>
    where
        StepFnImpl: FnOnce(&'w mut WorldImpl) -> Result<(), libtest::Failed>,
        OtherStepFnImpl: FnOnce(&'w mut WorldImpl) -> Result<(), libtest::Failed>,
        WorldImpl: 'w,
    {
        let mut metas = self.metas;
        metas.push(step.meta);

        let callback = move |world: &mut World| {
            (self.callback)(world)?;
            (step.callback)(world)?;
            Ok(())
        };

        Steps {
            metas,
            callback,
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

pub trait BackgroundStepFn<WorldImpl>: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static {}

impl<WorldImpl, T> BackgroundStepFn<WorldImpl> for T
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
