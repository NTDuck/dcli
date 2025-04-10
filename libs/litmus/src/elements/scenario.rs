use std::marker::PhantomData;

use crate::elements::step::World;
use crate::elements::step::GivenStepFn;
use crate::elements::step::ThenStepFn;
use crate::elements::step::WhenStepFn;
use crate::elements::step::StepMeta;
use crate::elements::step::Step;
use crate::elements::step::Steps;
use crate::elements::step::StepLabel;
use crate::elements::STEP_DELIMITER;
use crate::utils::aliases::MaybeOwnedStr;

pub use UnconfiguredScenario as Scenario;

pub struct UnconfiguredScenario<WorldImpl> {
    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> UnconfiguredScenario<WorldImpl>
where
    WorldImpl: World,
{
    pub fn named(description: impl Into<MaybeOwnedStr>) -> ScenarioWithDescriptionLastConfigured<WorldImpl> {
        ScenarioWithDescriptionLastConfigured {
            description: Some(description.into()),

            phantom: PhantomData,
        }
    }

    pub fn unnamed() -> ScenarioWithDescriptionLastConfigured<WorldImpl> {
        ScenarioWithDescriptionLastConfigured {
            description: None,

            phantom: PhantomData,
        }
    }
}

pub struct ScenarioWithDescriptionLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,

    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> ScenarioWithDescriptionLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn ignored(self, ignored: impl Into<bool>) -> ScenarioWithIgnoredLastConfigured<WorldImpl> {
        ScenarioWithIgnoredLastConfigured {
            description: self.description,
            ignored: Some(ignored.into()),

            phantom: PhantomData,
        }
    }

    pub fn given<GivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: GivenStepFnImpl) -> ScenarioWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl>
    where
        GivenStepFnImpl: GivenStepFn<WorldImpl>,
    {
        ScenarioWithGivenStepsLastConfigured {
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

pub struct ScenarioWithIgnoredLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> ScenarioWithIgnoredLastConfigured<WorldImpl> 
where
    WorldImpl: World,
{
    pub fn given<GivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: GivenStepFnImpl) -> ScenarioWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl>
    where
        GivenStepFnImpl: GivenStepFn<WorldImpl>,
    {
        ScenarioWithGivenStepsLastConfigured {
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

pub struct ScenarioWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: Steps<GivenStepFnImpl>,

    phantom: PhantomData<WorldImpl>,
}

impl<GivenStepFnImpl, WorldImpl> ScenarioWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl>
where
    GivenStepFnImpl: GivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(self, description: impl Into<MaybeOwnedStr>, callback: GivenStepFnImpl) -> Self {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::And,
                description: description.into(),
            },
            callback,
        };

        Self {
            given_steps: self.given_steps.with(step),
            ..self
        }
    }

    pub fn but(self, description: impl Into<MaybeOwnedStr>, callback: GivenStepFnImpl) -> Self {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::But,
                description: description.into(),
            },
            callback,
        };

        Self {
            given_steps: self.given_steps.with(step),
            ..self
        }
    }

    pub fn when<WhenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: WhenStepFnImpl) -> ScenarioWithWhenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, WorldImpl>
    where
        WhenStepFnImpl: WhenStepFn<WorldImpl>,
    {
        ScenarioWithWhenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: Steps {
                metas: vec![StepMeta {
                    label: StepLabel::When,
                    description: description.into(),
                }],
                callback,
            },

            phantom: PhantomData,
        }
    }
}

pub struct ScenarioWithWhenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: Steps<GivenStepFnImpl>,
    when_steps: Steps<WhenStepFnImpl>,

    phantom: PhantomData<WorldImpl>,
}

impl<GivenStepFnImpl, WhenStepFnImpl, WorldImpl> ScenarioWithWhenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, WorldImpl>
where
    GivenStepFnImpl: GivenStepFn<WorldImpl>,
    WhenStepFnImpl: WhenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(self, description: impl Into<MaybeOwnedStr>, callback: WhenStepFnImpl) -> Self {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::And,
                description: description.into(),
            },
            callback,
        };

        Self {
            when_steps: self.when_steps.with(step),
            ..self
        }
    }

    pub fn but(self, description: impl Into<MaybeOwnedStr>, callback: WhenStepFnImpl) -> Self {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::But,
                description: description.into(),
            },
            callback,
        };

        Self {
            when_steps: self.when_steps.with(step),
            ..self
        }
    }

    pub fn then<ThenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: ThenStepFnImpl) -> ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
    where
        ThenStepFnImpl: ThenStepFn<WorldImpl>,
    {
        ScenarioWithThenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps: Steps {
                metas: vec![StepMeta {
                    label: StepLabel::Then,
                    description: description.into(),
                }],
                callback,
            },

            phantom: PhantomData,
        }
    }
}

pub struct ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: Steps<GivenStepFnImpl>,
    when_steps: Steps<WhenStepFnImpl>,
    then_steps: Steps<ThenStepFnImpl>,

    phantom: PhantomData<WorldImpl>,
}

impl<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
where
    GivenStepFnImpl: GivenStepFn<WorldImpl>,
    WhenStepFnImpl: WhenStepFn<WorldImpl>,
    ThenStepFnImpl: ThenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(self, description: impl Into<MaybeOwnedStr>, callback: ThenStepFnImpl) -> Self {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::And,
                description: description.into(),
            },
            callback,
        };

        Self {
            then_steps: self.then_steps.with(step),
            ..self
        }
    }

    pub fn but(self, description: impl Into<MaybeOwnedStr>, callback: ThenStepFnImpl) -> Self {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::But,
                description: description.into(),
            },
            callback,
        };

        Self {
            then_steps: self.then_steps.with(step),
            ..self
        }
    }
}

impl<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> From<ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>> for FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
where
    GivenStepFnImpl: GivenStepFn<WorldImpl>,
    WhenStepFnImpl: WhenStepFn<WorldImpl>,
    ThenStepFnImpl: ThenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(scenario: ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>) -> Self {
        let ScenarioWithThenStepsLastConfigured {
            description,
            ignored,

            given_steps,
            when_steps,
            then_steps,
            ..
        } = scenario;

        return Self {
            description: match description {
                Some(description) => description,
                None => [&given_steps, &when_steps, &then_steps].join(STEP_DELIMITER),
            },
            ignored: match ignored {
                Some(ignored) => ignored,
                None => false,
            },

            given_steps_callback: given_steps.callback,
            when_steps_callback: when_steps.callback,
            then_steps_callback: then_steps.callback,

            phantom: PhantomData,
        };
    }
}

pub struct FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> {
    pub(super) description: MaybeOwnedStr,
    pub(super) ignored: bool,

    pub(super) given_steps_callback: GivenStepFnImpl,
    pub(super) when_steps_callback: WhenStepFnImpl,
    pub(super) then_steps_callback: ThenStepFnImpl,

    phantom: PhantomData<WorldImpl>,
}
