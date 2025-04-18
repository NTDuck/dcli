use std::marker::PhantomData;

pub use UnconfiguredBackground as Background;

use crate::elements::Step;
use crate::elements::StepLabel;
use crate::elements::World;
use crate::elements::ReusableGivenStepFn;
use crate::utils::aliases::Arc;
use crate::utils::aliases::MaybeOwnedStr;

use super::HookFn;
use super::ReusableHookedStepFn;
use super::ReusableHookedSteps;

pub struct UnconfiguredBackground<WorldImpl> {
    ctx: BackgroundContext<WorldImpl>,
}

impl<WorldImpl> From<BackgroundContext<WorldImpl>> for UnconfiguredBackground<WorldImpl>
where
    WorldImpl: World,
{
    fn from(ctx: BackgroundContext<WorldImpl>) -> Self {
        Self {
            ctx,
        }
    }
}

impl<WorldImpl> UnconfiguredBackground<WorldImpl>
where
    WorldImpl: World,
{
    pub fn named(self, description: impl Into<MaybeOwnedStr>) -> BackgroundWithDescriptionLastConfigured<WorldImpl> {
        BackgroundWithDescriptionLastConfigured {
            description: Some(description.into()),

            ctx: self.ctx,
        }
    }

    pub fn ignored(self, ignored: impl Into<bool>) -> BackgroundWithIgnoredLastConfigured<WorldImpl> {
        BackgroundWithIgnoredLastConfigured {
            description: None,
            ignored: Some(ignored.into()),

            ctx: self.ctx,
        }
    }

    pub fn given(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl ReusableGivenStepFn<WorldImpl>,
    ) -> BackgroundWithGivenStepsLastConfigured<impl ReusableHookedStepFn<WorldImpl>, WorldImpl> {
        let callback = self.ctx.hook(callback);

        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback,
        };

        BackgroundWithGivenStepsLastConfigured {
            description: None,
            ignored: None,

            steps: ReusableHookedSteps::from(step),

            ctx: self.ctx,
        }
    }
}

pub struct BackgroundWithDescriptionLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,

    ctx: BackgroundContext<WorldImpl>,
}

impl<WorldImpl> BackgroundWithDescriptionLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn ignored(self, ignored: impl Into<bool>) -> BackgroundWithIgnoredLastConfigured<WorldImpl> {
        BackgroundWithIgnoredLastConfigured {
            description: self.description,
            ignored: Some(ignored.into()),

            ctx: self.ctx,
        }
    }

    pub fn given(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl ReusableGivenStepFn<WorldImpl>,
    ) -> BackgroundWithGivenStepsLastConfigured<impl ReusableHookedStepFn<WorldImpl>, WorldImpl> {
        let callback = self.ctx.hook(callback);

        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback,
        };

        BackgroundWithGivenStepsLastConfigured {
            description: self.description,
            ignored: None,

            steps: ReusableHookedSteps::from(step),

            ctx: self.ctx,
        }
    }
}

pub struct BackgroundWithIgnoredLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    ctx: BackgroundContext<WorldImpl>,
}

impl<WorldImpl> BackgroundWithIgnoredLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn given(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl ReusableGivenStepFn<WorldImpl>,
    ) -> BackgroundWithGivenStepsLastConfigured<impl ReusableHookedStepFn<WorldImpl>, WorldImpl> {
        let callback = self.ctx.hook(callback);

        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback,
        };

        BackgroundWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            steps: ReusableHookedSteps::from(step),

            ctx: self.ctx,
        }
    }
}

pub struct BackgroundWithGivenStepsLastConfigured<HookedStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    steps: ReusableHookedSteps<HookedStepFnImpl>,

    ctx: BackgroundContext<WorldImpl>,
}

impl<HookedStepFnImpl, WorldImpl> BackgroundWithGivenStepsLastConfigured<HookedStepFnImpl, WorldImpl>
where
    HookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl ReusableGivenStepFn<WorldImpl>,
    ) -> BackgroundWithGivenStepsLastConfigured<impl ReusableHookedStepFn<WorldImpl>, WorldImpl> {
        let callback = self.ctx.hook(callback);

        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback,
        };

        BackgroundWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            steps: self.steps.chain(step),

            ctx: self.ctx,
        }
    }

    pub fn but(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl ReusableGivenStepFn<WorldImpl>,
    ) -> BackgroundWithGivenStepsLastConfigured<impl ReusableHookedStepFn<WorldImpl>, WorldImpl> {
        let callback = self.ctx.hook(callback);

        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback,
        };

        BackgroundWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            steps: self.steps.chain(step),

            ctx: self.ctx,
        }
    }
}

pub trait FinalizableBackground<HookedStepFnImpl, WorldImpl>:
    Into<BackgroundPayload<HookedStepFnImpl, WorldImpl>>
{
}

impl<T, HookedStepFnImpl, WorldImpl> FinalizableBackground<HookedStepFnImpl, WorldImpl> for T where
    T: Into<BackgroundPayload<HookedStepFnImpl, WorldImpl>>
{
}

impl<HookedStepFnImpl, WorldImpl>
    From<BackgroundWithGivenStepsLastConfigured<HookedStepFnImpl, WorldImpl>>
    for BackgroundPayload<HookedStepFnImpl, WorldImpl>
where
    HookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(background: BackgroundWithGivenStepsLastConfigured<HookedStepFnImpl, WorldImpl>) -> Self {
        Self {
            description: background.description,
            ignored: background.ignored,

            steps_callback: Arc::new(background.steps.callback),

            phantom: PhantomData,
        }
    }
}

pub struct BackgroundContext<WorldImpl> {
    pub(super) before_step_hooks_callback: Option<Arc<dyn HookFn<WorldImpl>>>,
    pub(super) after_step_hooks_callback: Option<Arc<dyn HookFn<WorldImpl>>>,
}

impl<WorldImpl> BackgroundContext<WorldImpl>
where
    WorldImpl: World,
{
    fn hook(&self, callback: impl ReusableGivenStepFn<WorldImpl>) -> impl ReusableHookedStepFn<WorldImpl> {
        let before_step_hooks_callback = self.before_step_hooks_callback.clone();
        let after_step_hooks_callback = self.after_step_hooks_callback.clone();

        move |world| {
            before_step_hooks_callback
                .as_ref()
                .map(|hook| (hook)(world));

            let result = (callback)(world);

            after_step_hooks_callback
                .as_ref()
                .map(|hook| (hook)(world));

            result
        }
    }
}

pub struct BackgroundPayload<HookedStepFnImpl, WorldImpl> {
    pub(super) description: Option<MaybeOwnedStr>,
    pub(super) ignored: Option<bool>,

    pub(super) steps_callback: Arc<HookedStepFnImpl>,

    phantom: PhantomData<WorldImpl>,
}

impl<HookedStepFnImpl, WorldImpl> Clone for BackgroundPayload<HookedStepFnImpl, WorldImpl> {
    fn clone(&self) -> Self {
        Self {
            description: self.description.clone(),
            ignored: self.ignored,

            steps_callback: self.steps_callback.clone(),

            phantom: PhantomData,
        }
    }
}
