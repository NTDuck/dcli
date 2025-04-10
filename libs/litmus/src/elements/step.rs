use crate::utils::aliases::MaybeOwnedStr;

pub(crate) struct Step<StepFnImpl> {
    pub(crate) label: StepLabel,
    pub(crate) description: MaybeOwnedStr,
    pub(crate) callback: StepFnImpl,
}

impl<StepFnImpl> std::fmt::Display for Step<StepFnImpl> {
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

pub trait GivenStepFn<WorldImpl>: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static {}

impl<WorldImpl, T> GivenStepFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
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

pub(crate) trait VecStepExt<StepFnImpl> {
    fn format(&self) -> impl IntoIterator<Item = String>;
    fn callbacks(self) -> Vec<StepFnImpl>;
}

impl<StepFnImpl> VecStepExt<StepFnImpl> for Vec<Step<StepFnImpl>> {
    fn format(&self) -> impl IntoIterator<Item = String> {
        self
            .iter()
            .map(|step| format!("{}", step))
    }
    
    fn callbacks(self) -> Vec<StepFnImpl> {
        self
            .into_iter()
            .map(|step| step.callback)
            .collect()
    }
}
