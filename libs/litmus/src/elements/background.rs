use std::marker::PhantomData;
use std::sync::Arc;

use crate::elements::step::World;
use crate::elements::step::BackgroundGivenStepFn;
use crate::elements::step::StepMeta;
use crate::elements::step::Step;
use crate::elements::step::Steps;
use crate::elements::step::StepLabel;
use crate::utils::aliases::MaybeOwnedStr;

pub use UnconfiguredBackground as Background;

pub struct UnconfiguredBackground<WorldImpl> {
    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> UnconfiguredBackground<WorldImpl>
where
    WorldImpl: World,
{
    pub fn named(description: impl Into<MaybeOwnedStr>) -> BackgroundWithDescriptionLastConfigured<WorldImpl> {
        BackgroundWithDescriptionLastConfigured {
            description: description.into(),

            phantom: PhantomData,
        }
    }
}

pub struct BackgroundWithDescriptionLastConfigured<WorldImpl> {
    description: MaybeOwnedStr,

    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> BackgroundWithDescriptionLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn ignored(self, ignored: impl Into<bool>) -> BackgroundWithIgnoredLastConfigured<WorldImpl> {
        BackgroundWithIgnoredLastConfigured {
            description: self.description,
            ignored: Some(ignored.into()),

            phantom: PhantomData,
        }
    }

    pub fn given<BackgroundGivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: BackgroundGivenStepFnImpl) -> BackgroundWithGivenStepsLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        BackgroundWithGivenStepsLastConfigured {
            description: self.description,
            ignored: None,

            given_steps: Steps {
                metas: vec![StepMeta {
                    label: StepLabel::Given,
                    description: description.into(),
                }],
                callback,
            },

            phantom: PhantomData,
        }
    }
}

pub struct BackgroundWithIgnoredLastConfigured<WorldImpl> {
    description: MaybeOwnedStr,
    ignored: Option<bool>,

    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> BackgroundWithIgnoredLastConfigured<WorldImpl> 
where
    WorldImpl: World,
{
    pub fn given<BackgroundGivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: BackgroundGivenStepFnImpl) -> BackgroundWithGivenStepsLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        BackgroundWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: Steps {
                metas: vec![StepMeta {
                    label: StepLabel::Given,
                    description: description.into(),
                }],
                callback,
            },

            phantom: PhantomData,
        }
    }
}

pub struct BackgroundWithGivenStepsLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
    description: MaybeOwnedStr,
    ignored: Option<bool>,

    given_steps: Steps<BackgroundGivenStepFnImpl>,

    phantom: PhantomData<WorldImpl>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> BackgroundWithGivenStepsLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherBackgroundGivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherBackgroundGivenStepFnImpl) -> BackgroundWithGivenStepsLastConfigured<impl BackgroundGivenStepFn<WorldImpl>, WorldImpl>
    where
        OtherBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::And,
                description: description.into(),
            },
            callback,
        };

        BackgroundWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps.chain_given_background(step),

            phantom: PhantomData,
        }
    }

    pub fn but<OtherBackgroundGivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherBackgroundGivenStepFnImpl) -> BackgroundWithGivenStepsLastConfigured<impl BackgroundGivenStepFn<WorldImpl>, WorldImpl>
    where
        OtherBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::But,
                description: description.into(),
            },
            callback,
        };

        BackgroundWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps.chain_given_background(step),

            phantom: PhantomData,
        }
    }
}

impl<BackgroundGivenStepFnImpl, WorldImpl> From<BackgroundWithGivenStepsLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>> for FinalizedBackground<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(background: BackgroundWithGivenStepsLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        let BackgroundWithGivenStepsLastConfigured {
            description,
            ignored,

            given_steps,
            ..
        } = background;

        Self {
            description,
            ignored: match ignored {
                Some(ignored) => ignored,
                None => false,
            },

            given_steps_callback: Arc::new(given_steps.callback),

            phantom: PhantomData,
        }
    }
}

pub struct FinalizedBackground<BackgroundGivenStepFnImpl, WorldImpl> {
    pub(super) description: MaybeOwnedStr,
    pub(super) ignored: bool,

    pub(super) given_steps_callback: Arc<BackgroundGivenStepFnImpl>,

    phantom: PhantomData<WorldImpl>,
}
