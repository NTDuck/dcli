use std::marker::PhantomData;

use crate::elements::step::World;
use crate::elements::step::ScenarioGivenStepFn;
use crate::elements::step::ScenarioThenStepFn;
use crate::elements::step::ScenarioWhenStepFn;
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
        GivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
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
        GivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
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
    GivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherGivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherGivenStepFnImpl) -> ScenarioWithGivenStepsLastConfigured<impl ScenarioGivenStepFn<WorldImpl>, WorldImpl>
    where
        OtherGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::And,
                description: description.into(),
            },
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps.chain_scenario_given(step),

            phantom: PhantomData,
        }
    }

    pub fn but<OtherGivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherGivenStepFnImpl) -> ScenarioWithGivenStepsLastConfigured<impl ScenarioGivenStepFn<WorldImpl>, WorldImpl>
    where
        OtherGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::But,
                description: description.into(),
            },
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps.chain_scenario_given(step),

            phantom: PhantomData,
        }
    }

    pub fn when<WhenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: WhenStepFnImpl) -> ScenarioWithWhenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, WorldImpl>
    where
        WhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
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
    GivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    WhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherWhenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherWhenStepFnImpl) -> ScenarioWithWhenStepsLastConfigured<GivenStepFnImpl, impl ScenarioWhenStepFn<WorldImpl>, WorldImpl>
    where
        OtherWhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
    {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::And,
                description: description.into(),
            },
            callback,
        };

        ScenarioWithWhenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: self.when_steps.chain_scenario_when(step),

            phantom: PhantomData,
        }
    }

    pub fn but<OtherWhenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherWhenStepFnImpl) -> ScenarioWithWhenStepsLastConfigured<GivenStepFnImpl, impl ScenarioWhenStepFn<WorldImpl>, WorldImpl>
    where
        OtherWhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
    {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::But,
                description: description.into(),
            },
            callback,
        };

        ScenarioWithWhenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: self.when_steps.chain_scenario_when(step),

            phantom: PhantomData,
        }
    }

    pub fn then<ThenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: ThenStepFnImpl) -> ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
    where
        ThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
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
    GivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    WhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
    ThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherThenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherThenStepFnImpl) -> ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, impl ScenarioThenStepFn<WorldImpl>, WorldImpl>
    where
        OtherThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
    {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::And,
                description: description.into(),
            },
            callback,
        };

        ScenarioWithThenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps: self.then_steps.chain_scenario_then(step),

            phantom: PhantomData,
        }
    }

    pub fn but<OtherThenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherThenStepFnImpl) -> ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, impl ScenarioThenStepFn<WorldImpl>, WorldImpl>
    where
        OtherThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
    {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::But,
                description: description.into(),
            },
            callback,
        };

        ScenarioWithThenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps: self.then_steps.chain_scenario_then(step),

            phantom: PhantomData,
        }
    }
}

impl<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> From<ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>> for FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
where
    GivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    WhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
    ThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
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

        Self {
            description: match description {
                Some(description) => description,
                None => format!("{}{}{}{}{}", given_steps, STEP_DELIMITER, when_steps, STEP_DELIMITER, then_steps).into(),
            },
            ignored: match ignored {
                Some(ignored) => ignored,
                None => false,
            },

            given_steps_callback: given_steps.callback,
            when_steps_callback: when_steps.callback,
            then_steps_callback: then_steps.callback,

            phantom: PhantomData,
        }
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
