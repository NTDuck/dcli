use crate::utils::aliases::MaybeOwnedStr;

pub(crate) struct Step<StepFnImpl> {
    pub(crate) metas: Vec<StepMeta>,
    pub(crate) callback: StepFnImpl,
}

impl<StepFnImpl> Step<StepFnImpl> {
    pub fn with<OtherStepFnImpl, Args, R>(self, meta: StepMeta, callback: OtherStepFnImpl) -> Step<OtherStepFnImpl>
    where
        StepFnImpl: FnOnce(Args) -> R,
        OtherStepFnImpl: FnOnce(Args) -> R,
    {
        let mut metas = self.metas;
        metas.push(meta);

        let callback = move |args: Args| {
            self.callback(args);
            callback(args);
        };

        Step {
            metas,
            callback,
        }
    }
}

impl<StepFnImpl> std::fmt::Display for Step<StepFnImpl> {
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

pub(crate) trait VecExt<T> {
    fn with(self, item: T) -> Self;
}

impl<T> VecExt<T> for Vec<T> {
    fn with(self, item: T) -> Self {
        let mut this = self;
        this.push(item);
        this
    }
}
