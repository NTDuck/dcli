use crate::utils::aliases::MaybeOwnedStr;

pub(super) struct GivenSteps<StepFnImpl> {
    pub(super) metas: Vec<StepMeta>,
    pub(super) callback: StepFnImpl,
}

impl<StepFnImpl> GivenSteps<StepFnImpl> {
    pub(super) fn chain<WorldImpl>(self, step: Step<impl GivenStepFn<WorldImpl>>) -> GivenSteps<impl GivenStepFn<WorldImpl>>
    where
        StepFnImpl: GivenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };
        metas.push(meta);

        let callback = self.callback.chain(step.callback);

        GivenSteps {
            metas,
            callback,
        }
    }
}

impl<StepFnImpl> From<Step<StepFnImpl>> for GivenSteps<StepFnImpl> {
    fn from(step: Step<StepFnImpl>) -> Self {
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };

        Self {
            metas: vec![meta],
            callback: step.callback,
        }
    }
}

impl<StepFnImpl> std::fmt::Display for GivenSteps<StepFnImpl> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.metas
            .iter()
            .map(|meta| format!("{}", meta))
            .collect::<Vec<_>>()
            .join(" ");

        write!(formatter, "{}", joined)
    }
}

pub(super) struct WhenSteps<StepFnImpl> {
    pub(super) metas: Vec<StepMeta>,
    pub(super) callback: StepFnImpl,
}

impl<StepFnImpl> WhenSteps<StepFnImpl> {
    pub(super) fn chain<WorldImpl>(self, step: Step<impl WhenStepFn<WorldImpl>>) -> WhenSteps<impl WhenStepFn<WorldImpl>>
    where
        StepFnImpl: WhenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };
        metas.push(meta);

        let callback = self.callback.chain(step.callback);

        WhenSteps {
            metas,
            callback,
        }
    }
}

impl<StepFnImpl> From<Step<StepFnImpl>> for WhenSteps<StepFnImpl> {
    fn from(step: Step<StepFnImpl>) -> Self {
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };

        Self {
            metas: vec![meta],
            callback: step.callback,
        }
    }
}

impl<StepFnImpl> std::fmt::Display for WhenSteps<StepFnImpl> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.metas
            .iter()
            .map(|meta| format!("{}", meta))
            .collect::<Vec<_>>()
            .join(" ");

        write!(formatter, "{}", joined)
    }
}

pub(super) struct ThenSteps<StepFnImpl> {
    pub(super) metas: Vec<StepMeta>,
    pub(super) callback: StepFnImpl,
}

impl<StepFnImpl> ThenSteps<StepFnImpl> {
    pub(super) fn chain<WorldImpl>(self, step: Step<impl ThenStepFn<WorldImpl>>) -> ThenSteps<impl ThenStepFn<WorldImpl>>
    where
        StepFnImpl: ThenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };
        metas.push(meta);

        let callback = self.callback.chain(step.callback);

        ThenSteps {
            metas,
            callback,
        }
    }
}

impl<StepFnImpl> From<Step<StepFnImpl>> for ThenSteps<StepFnImpl> {
    fn from(step: Step<StepFnImpl>) -> Self {
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };

        Self {
            metas: vec![meta],
            callback: step.callback,
        }
    }
}

impl<StepFnImpl> std::fmt::Display for ThenSteps<StepFnImpl> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.metas
            .iter()
            .map(|meta| format!("{}", meta))
            .collect::<Vec<_>>()
            .join(" ");

        write!(formatter, "{}", joined)
    }
}

pub(super) struct ReusableGivenSteps<StepFnImpl> {
    pub(super) metas: Vec<StepMeta>,
    pub(super) callback: StepFnImpl,
}

impl<StepFnImpl> ReusableGivenSteps<StepFnImpl> {
    pub(super) fn chain<WorldImpl>(
        self,
        step: Step<impl ReusableGivenStepFn<WorldImpl>>,
    ) -> ReusableGivenSteps<impl ReusableGivenStepFn<WorldImpl>>
    where
        StepFnImpl: ReusableGivenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };
        metas.push(meta);

        let callback = self.callback.chain(step.callback);

        ReusableGivenSteps { metas, callback }
    }
}

impl<StepFnImpl> From<Step<StepFnImpl>> for ReusableGivenSteps<StepFnImpl> {
    fn from(step: Step<StepFnImpl>) -> Self {
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };

        Self {
            metas: vec![meta],
            callback: step.callback,
        }
    }
}

impl<StepFnImpl> std::fmt::Display for ReusableGivenSteps<StepFnImpl> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.metas
            .iter()
            .map(|meta| format!("{}", meta))
            .collect::<Vec<_>>()
            .join(" ");
        write!(formatter, "{}", joined)
    }
}

pub(super) struct ReusableWhenSteps<StepFnImpl> {
    pub(super) metas: Vec<StepMeta>,
    pub(super) callback: StepFnImpl,
}

impl<StepFnImpl> ReusableWhenSteps<StepFnImpl> {
    pub(super) fn chain<WorldImpl>(
        self,
        step: Step<impl ReusableWhenStepFn<WorldImpl>>,
    ) -> ReusableWhenSteps<impl ReusableWhenStepFn<WorldImpl>>
    where
        StepFnImpl: ReusableWhenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };
        metas.push(meta);

        let callback = self.callback.chain(step.callback);

        ReusableWhenSteps { metas, callback }
    }
}

impl<StepFnImpl> From<Step<StepFnImpl>> for ReusableWhenSteps<StepFnImpl> {
    fn from(step: Step<StepFnImpl>) -> Self {
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };

        Self {
            metas: vec![meta],
            callback: step.callback,
        }
    }
}

impl<StepFnImpl> std::fmt::Display for ReusableWhenSteps<StepFnImpl> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.metas
            .iter()
            .map(|meta| format!("{}", meta))
            .collect::<Vec<_>>()
            .join(" ");
        write!(formatter, "{}", joined)
    }
}

pub(super) struct ReusableThenSteps<StepFnImpl> {
    pub(super) metas: Vec<StepMeta>,
    pub(super) callback: StepFnImpl,
}

impl<StepFnImpl> ReusableThenSteps<StepFnImpl> {
    pub(super) fn chain<WorldImpl>(
        self,
        step: Step<impl ReusableThenStepFn<WorldImpl>>,
    ) -> ReusableThenSteps<impl ReusableThenStepFn<WorldImpl>>
    where
        StepFnImpl: ReusableThenStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };
        metas.push(meta);

        let callback = self.callback.chain(step.callback);

        ReusableThenSteps { metas, callback }
    }
}

impl<StepFnImpl> From<Step<StepFnImpl>> for ReusableThenSteps<StepFnImpl> {
    fn from(step: Step<StepFnImpl>) -> Self {
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };

        Self {
            metas: vec![meta],
            callback: step.callback,
        }
    }
}

impl<StepFnImpl> std::fmt::Display for ReusableThenSteps<StepFnImpl> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.metas
            .iter()
            .map(|meta| format!("{}", meta))
            .collect::<Vec<_>>()
            .join(" ");
        write!(formatter, "{}", joined)
    }
}

pub(super) struct Step<StepFnImpl> {
    pub(super) label: StepLabel,
    pub(super) description: MaybeOwnedStr,
    pub(super) callback: StepFnImpl,
}

pub(super) struct StepMeta {
    pub(super) label: StepLabel,
    pub(super) description: MaybeOwnedStr,
}

impl std::fmt::Display for StepMeta {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} {}", self.label, self.description)
    }
}

#[derive(strum::Display)]
pub(super) enum StepLabel {
    Given,
    When,
    Then,

    And,
    But,
}

pub trait ReusableGivenStepFn<WorldImpl>:
    Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
    fn chain(self, other: impl ReusableGivenStepFn<WorldImpl>) -> impl ReusableGivenStepFn<WorldImpl>
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

impl<T, WorldImpl> ReusableGivenStepFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ReusableWhenStepFn<WorldImpl>:
    Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
    fn chain(self, other: impl ReusableWhenStepFn<WorldImpl>) -> impl ReusableWhenStepFn<WorldImpl>
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

impl<T, WorldImpl> ReusableWhenStepFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ReusableThenStepFn<WorldImpl>:
    Fn(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
    fn chain(self, other: impl ReusableThenStepFn<WorldImpl>) -> impl ReusableThenStepFn<WorldImpl>
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

impl<T, WorldImpl> ReusableThenStepFn<WorldImpl> for T
where
    T: Fn(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait GivenStepFn<WorldImpl>:
    FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
    fn chain(self, other: impl GivenStepFn<WorldImpl>) -> impl GivenStepFn<WorldImpl>
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

impl<T, WorldImpl> GivenStepFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait WhenStepFn<WorldImpl>:
    FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
    fn chain(self, other: impl WhenStepFn<WorldImpl>) -> impl WhenStepFn<WorldImpl>
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

impl<T, WorldImpl> WhenStepFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ThenStepFn<WorldImpl>:
    FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
    fn chain(self, other: impl ThenStepFn<WorldImpl>) -> impl ThenStepFn<WorldImpl>
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

impl<T, WorldImpl> ThenStepFn<WorldImpl> for T
where
    T: FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait World: Default + Send + Sync + 'static {}

impl<T> World for T where T: Default + Send + Sync + 'static {}
