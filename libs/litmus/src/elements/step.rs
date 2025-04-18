use crate::elements::World;
use crate::utils::aliases::{Arc, MaybeOwnedStr};

use super::HookFn;

pub(super) struct ReusableHookedSteps<StepFnImpl> {
    pub(super) metas: Vec<StepMeta>,
    pub(super) callback: StepFnImpl,
}

impl<StepFnImpl> ReusableHookedSteps<StepFnImpl> {
    pub(super) fn chain<WorldImpl>(
        self,
        step: Step<impl ReusableHookedStepFn<WorldImpl>>,
    ) -> ReusableHookedSteps<impl ReusableGivenStepFn<WorldImpl>>
    where
        StepFnImpl: ReusableHookedStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };
        metas.push(meta);

        let callback = self.callback.chain(step.callback);

        ReusableHookedSteps { metas, callback }
    }
}

impl<StepFnImpl> From<Step<StepFnImpl>> for ReusableHookedSteps<StepFnImpl> {
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

impl<StepFnImpl> std::fmt::Display for ReusableHookedSteps<StepFnImpl> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.metas
            .iter()
            .map(|meta| format!("{}", meta))
            .collect::<Vec<_>>()
            .join(" ");
        write!(formatter, "{}", joined)
    }
}

pub(super) struct HookedSteps<StepFnImpl> {
    pub(super) metas: Vec<StepMeta>,
    pub(super) callback: StepFnImpl,
}

impl<StepFnImpl> HookedSteps<StepFnImpl> {
    pub(super) fn chain<WorldImpl>(
        self,
        step: Step<impl HookedStepFn<WorldImpl>>,
    ) -> HookedSteps<impl HookedStepFn<WorldImpl>>
    where
        StepFnImpl: HookedStepFn<WorldImpl>,
        WorldImpl: World,
    {
        let mut metas = self.metas;
        let meta = StepMeta {
            label: step.label,
            description: step.description,
        };
        metas.push(meta);

        let callback = self.callback.chain(step.callback);

        HookedSteps { metas, callback }
    }
}

impl<StepFnImpl> From<Step<StepFnImpl>> for HookedSteps<StepFnImpl> {
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

impl<StepFnImpl> std::fmt::Display for HookedSteps<StepFnImpl> {
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

    fn chain_before(self, hook: Arc<dyn HookFn<WorldImpl>>) -> impl GivenStepFn<WorldImpl>
    where
        Self: Sized,
        WorldImpl: World,
    {
        move |world| {
            (hook)(world);
            (self)(world)?;

            Ok(())
        }
    }

    fn chain_after(self, hook: Arc<dyn HookFn<WorldImpl>>) -> impl GivenStepFn<WorldImpl>
    where
        Self: Sized,
        WorldImpl: World,
    {
        move |world| {
            let result = (self)(world);
            (hook)(world);

            result
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

pub trait ReusableHookedStepFn<WorldImpl>:
    Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
    fn chain(self, other: impl ReusableHookedStepFn<WorldImpl>) -> impl ReusableHookedStepFn<WorldImpl>
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

impl<T, WorldImpl> ReusableHookedStepFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait HookedStepFn<WorldImpl>:
    FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
    fn chain(self, other: impl HookedStepFn<WorldImpl>) -> impl HookedStepFn<WorldImpl>
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

impl<T, WorldImpl> HookedStepFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub(super) type NoOpGivenStepFn<WorldImpl> = fn(&mut WorldImpl) -> Result<(), libtest::Failed>;
pub(super) type NoOpWhenStepFn<WorldImpl> = fn(&mut WorldImpl) -> Result<(), libtest::Failed>;
pub(super) type NoOpThenStepFn<WorldImpl> = fn(&WorldImpl) -> Result<(), libtest::Failed>;
