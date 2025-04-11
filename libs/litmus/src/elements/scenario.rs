use crate::elements::step::World;
use crate::elements::step::ScenarioGivenStepFn;
use crate::elements::step::ScenarioThenStepFn;
use crate::elements::step::ScenarioWhenStepFn;
use crate::elements::step::BackgroundGivenStepFn;
use crate::elements::step::StepMeta;
use crate::elements::step::Step;
use crate::elements::step::Steps;
use crate::elements::step::StepLabel;
use crate::elements::STEP_DELIMITER;
use crate::utils::aliases::MaybeOwnedStr;

pub use UnconfiguredScenario as Scenario;

use super::FeatureContext;
use super::RuleContext;

pub struct UnconfiguredScenario<BackgroundGivenStepFnImpl, WorldImpl> {
    context: Context<BackgroundGivenStepFnImpl, WorldImpl>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> From<FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>> for UnconfiguredScenario<BackgroundGivenStepFnImpl, WorldImpl> {
    fn from(feature: FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        Self {
            context: Context::Feature(feature),
        }
    }
}

impl<BackgroundGivenStepFnImpl, WorldImpl> From<RuleContext<BackgroundGivenStepFnImpl, WorldImpl>> for UnconfiguredScenario<BackgroundGivenStepFnImpl, WorldImpl> {
    fn from(rule: RuleContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        Self {
            context: Context::Rule(rule),
        }
    }
}

impl<BackgroundGivenStepFnImpl, WorldImpl> UnconfiguredScenario<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn named(self, description: impl Into<MaybeOwnedStr>) -> ScenarioWithDescriptionLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
        ScenarioWithDescriptionLastConfigured {
            description: Some(description.into()),

            context: self.context,
        }
    }

    pub fn unnamed(self) -> ScenarioWithDescriptionLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
        ScenarioWithDescriptionLastConfigured {
            description: None,

            context: self.context,
        }
    }
}

pub struct ScenarioWithDescriptionLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,

    context: Context<BackgroundGivenStepFnImpl, WorldImpl>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> ScenarioWithDescriptionLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn ignored(self, ignored: impl Into<bool>) -> ScenarioWithIgnoredLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
        ScenarioWithIgnoredLastConfigured {
            description: self.description,
            ignored: Some(ignored.into()),

            context: self.context,
        }
    }

    pub fn given<ScenarioGivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: ScenarioGivenStepFnImpl) -> ScenarioWithGivenStepsLastConfigured<ScenarioGivenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl>
    where
        ScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
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

            context: self.context,
        }
    }
}

pub struct ScenarioWithIgnoredLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    context: Context<BackgroundGivenStepFnImpl, WorldImpl>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> ScenarioWithIgnoredLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> 
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn given<ScenarioGivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: ScenarioGivenStepFnImpl) -> ScenarioWithGivenStepsLastConfigured<ScenarioGivenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl>
    where
        ScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
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

            context: self.context,
        }
    }
}

pub struct ScenarioWithGivenStepsLastConfigured<ScenarioGivenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: Steps<ScenarioGivenStepFnImpl>,

    context: Context<BackgroundGivenStepFnImpl, WorldImpl>,
}

impl<ScenarioGivenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl> ScenarioWithGivenStepsLastConfigured<ScenarioGivenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl>
where
    ScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherScenarioGivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherScenarioGivenStepFnImpl) -> ScenarioWithGivenStepsLastConfigured<impl ScenarioGivenStepFn<WorldImpl>, BackgroundGivenStepFnImpl, WorldImpl>
    where
        OtherScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
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

            context: self.context,
        }
    }

    pub fn but<OtherScenarioGivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherScenarioGivenStepFnImpl) -> ScenarioWithGivenStepsLastConfigured<impl ScenarioGivenStepFn<WorldImpl>, BackgroundGivenStepFnImpl, WorldImpl>
    where
        OtherScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
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

            context: self.context,
        }
    }

    pub fn when<ScenarioWhenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: ScenarioWhenStepFnImpl) -> ScenarioWithWhenStepsLastConfigured<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl>
    where
        ScenarioWhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
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

            context: self.context,
        }
    }
}

pub struct ScenarioWithWhenStepsLastConfigured<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: Steps<ScenarioGivenStepFnImpl>,
    when_steps: Steps<ScenarioWhenStepFnImpl>,

    context: Context<BackgroundGivenStepFnImpl, WorldImpl>,
}

impl<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl> ScenarioWithWhenStepsLastConfigured<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl>
where
    ScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    ScenarioWhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherScenarioWhenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherScenarioWhenStepFnImpl) -> ScenarioWithWhenStepsLastConfigured<ScenarioGivenStepFnImpl, impl ScenarioWhenStepFn<WorldImpl>, BackgroundGivenStepFnImpl, WorldImpl>
    where
        OtherScenarioWhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
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

            context: self.context,
        }
    }

    pub fn but<OtherScenarioWhenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherScenarioWhenStepFnImpl) -> ScenarioWithWhenStepsLastConfigured<ScenarioGivenStepFnImpl, impl ScenarioWhenStepFn<WorldImpl>, BackgroundGivenStepFnImpl, WorldImpl>
    where
        OtherScenarioWhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
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

            context: self.context,
        }
    }

    pub fn then<ScenarioThenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: ScenarioThenStepFnImpl) -> ScenarioWithThenStepsLastConfigured<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, ScenarioThenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl>
    where
        ScenarioThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
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

            context: self.context,
        }
    }
}

pub struct ScenarioWithThenStepsLastConfigured<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, ScenarioThenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: Steps<ScenarioGivenStepFnImpl>,
    when_steps: Steps<ScenarioWhenStepFnImpl>,
    then_steps: Steps<ScenarioThenStepFnImpl>,

    context: Context<BackgroundGivenStepFnImpl, WorldImpl>,
}

impl<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, ScenarioThenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl> ScenarioWithThenStepsLastConfigured<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, ScenarioThenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl>
where
    ScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    ScenarioWhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
    ScenarioThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherScenarioThenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherScenarioThenStepFnImpl) -> ScenarioWithThenStepsLastConfigured<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, impl ScenarioThenStepFn<WorldImpl>, BackgroundGivenStepFnImpl, WorldImpl>
    where
        OtherScenarioThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
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

            context: self.context,
        }
    }

    pub fn but<OtherScenarioThenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: OtherScenarioThenStepFnImpl) -> ScenarioWithThenStepsLastConfigured<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, impl ScenarioThenStepFn<WorldImpl>, BackgroundGivenStepFnImpl, WorldImpl>
    where
        OtherScenarioThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
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

            context: self.context,
        }
    }
}

impl<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, ScenarioThenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl> From<ScenarioWithThenStepsLastConfigured<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, ScenarioThenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl>> for libtest::Trial
where
    ScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    ScenarioWhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
    ScenarioThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(scenario: ScenarioWithThenStepsLastConfigured<ScenarioGivenStepFnImpl, ScenarioWhenStepFnImpl, ScenarioThenStepFnImpl, BackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        let scenario_description = match scenario.description {
            Some(description) => description,
            None => format!("{}{}{}{}{}", scenario.given_steps, STEP_DELIMITER, scenario.when_steps, STEP_DELIMITER, scenario.then_steps).into(),
        };

        match scenario.context {
            Context::Feature(feature) => libtest::Trial::test(scenario_description, move || {
                let mut world = WorldImpl::default();

                if let Some(feature_background) = feature.background {
                    if !feature_background.ignored {
                        (feature_background.given_steps_callback)(&mut world)?;
                    }
                }

                (scenario.given_steps.callback)(&mut world)?;
                (scenario.when_steps.callback)(&mut world)?;
                (scenario.then_steps.callback)(&world)?;

                Ok(())
            })
                .with_ignored_flag(feature.ignored.unwrap_or(false) || scenario.ignored.unwrap_or(false))
                .with_kind(match feature.description {
                    Some(description) => description,
                    None => "".into()
                }),
            Context::Rule(rule) => libtest::Trial::test(scenario_description, move || {
                let mut world = WorldImpl::default();

                if let Some(feature_background) = rule.feature.background {
                    if !feature_background.ignored {
                        (feature_background.given_steps_callback)(&mut world)?;
                    }
                }

                if let Some(rule_background) = rule.background {
                    if !rule_background.ignored {
                        (rule_background.given_steps_callback)(&mut world)?;
                    }
                }

                (scenario.given_steps.callback)(&mut world)?;
                (scenario.when_steps.callback)(&mut world)?;
                (scenario.then_steps.callback)(&world)?;

                Ok(())
            })
                .with_ignored_flag(rule.feature.ignored.unwrap_or(false) || rule.ignored.unwrap_or(false) || scenario.ignored.unwrap_or(false))
                .with_kind(match (rule.feature.description, rule.description) {
                    (Some(feature_description), Some(rule_description)) => format!("{}{}{}", feature_description, STEP_DELIMITER, rule_description).into(),
                    (Some(feature_description), None) => feature_description,
                    (None, Some(rule_description)) => rule_description,
                    (None, None) => "".into(),
                }),
        }
    }
}

enum Context<BackgroundGivenStepFnImpl, WorldImpl> {
    Feature(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>),
    Rule(RuleContext<BackgroundGivenStepFnImpl, WorldImpl>),
}

impl<BackgroundGivenStepFnImpl, WorldImpl> Clone for Context<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn clone(&self) -> Self {
        match self {
            Self::Feature(feature) => Self::Feature(feature.clone()),
            Self::Rule(rule) => Self::Rule(rule.clone()),
        }
    }
}