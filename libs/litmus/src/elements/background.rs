use std::marker::PhantomData;

pub use UnconfiguredBackground as Background;

use crate::elements::Step;
use crate::elements::StepLabel;
use crate::elements::World;
use crate::elements::ReusableGivenStepFn;
use crate::elements::ReusableGivenSteps;
use crate::utils::aliases::Arc;
use crate::utils::aliases::MaybeOwnedStr;

use super::Hooks;

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

    pub fn unnamed(self) -> BackgroundWithDescriptionLastConfigured<WorldImpl> {
        BackgroundWithDescriptionLastConfigured {
            description: None,

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

    pub fn given<BackgroundGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: BackgroundGivenStepFnImpl,
    ) -> BackgroundWithGivenStepsLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback,
        };

        BackgroundWithGivenStepsLastConfigured {
            description: self.description,
            ignored: None,

            given_steps: ReusableGivenSteps::from(step),

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
    pub fn given<BackgroundGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: BackgroundGivenStepFnImpl,
    ) -> BackgroundWithGivenStepsLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback,
        };

        BackgroundWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: ReusableGivenSteps::from(step),

            ctx: self.ctx,
        }
    }
}

pub struct BackgroundWithGivenStepsLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: ReusableGivenSteps<BackgroundGivenStepFnImpl>,

    ctx: BackgroundContext<WorldImpl>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> BackgroundWithGivenStepsLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherBackgroundGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherBackgroundGivenStepFnImpl,
    ) -> BackgroundWithGivenStepsLastConfigured<impl ReusableGivenStepFn<WorldImpl>, WorldImpl>
    where
        OtherBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback,
        };

        BackgroundWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps.chain(step),

            ctx: self.ctx,
        }
    }

    pub fn but<OtherBackgroundGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherBackgroundGivenStepFnImpl,
    ) -> BackgroundWithGivenStepsLastConfigured<impl ReusableGivenStepFn<WorldImpl>, WorldImpl>
    where
        OtherBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback,
        };

        BackgroundWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps.chain(step),

            ctx: self.ctx,
        }
    }
}

pub trait FinalizableBackground<BackgroundGivenStepFnImpl, WorldImpl>:
    Into<BackgroundPayload<BackgroundGivenStepFnImpl, WorldImpl>>
{
}

impl<T, BackgroundGivenStepFnImpl, WorldImpl> FinalizableBackground<BackgroundGivenStepFnImpl, WorldImpl> for T where
    T: Into<BackgroundPayload<BackgroundGivenStepFnImpl, WorldImpl>>
{
}

impl<BackgroundGivenStepFnImpl, WorldImpl>
    From<BackgroundWithGivenStepsLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>>
    for BackgroundPayload<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(background: BackgroundWithGivenStepsLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        Self {
            description: background.description,
            ignored: background.ignored,

            given_steps_callback: Arc::new(background.given_steps.callback),

            phantom: PhantomData,
        }
    }
}

pub struct BackgroundContext<WorldImpl> {
    pub(super) before_step_hooks: Hooks<WorldImpl>,
    pub(super) after_step_hooks: Hooks<WorldImpl>,
}

pub struct BackgroundPayload<BackgroundGivenStepFnImpl, WorldImpl> {
    pub(super) description: Option<MaybeOwnedStr>,
    pub(super) ignored: Option<bool>,

    pub(super) given_steps_callback: Arc<BackgroundGivenStepFnImpl>,

    phantom: PhantomData<WorldImpl>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> Clone for BackgroundPayload<BackgroundGivenStepFnImpl, WorldImpl> {
    fn clone(&self) -> Self {
        Self {
            description: self.description.clone(),
            ignored: self.ignored,

            given_steps_callback: self.given_steps_callback.clone(),

            phantom: PhantomData,
        }
    }
}
