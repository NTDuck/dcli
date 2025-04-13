pub use UnconfiguredScenario as Scenario;

use super::FeatureContext;
use super::RuleContext;
use crate::elements::BackgroundGivenStepFn;
use crate::elements::ScenarioGivenStepFn;
use crate::elements::ScenarioThenStepFn;
use crate::elements::ScenarioWhenStepFn;
use crate::elements::Step;
use crate::elements::StepLabel;
use crate::elements::StepMeta;
use crate::elements::Steps;
use crate::elements::World;
use crate::elements::STEP_DELIMITER;
use crate::utils::aliases::MaybeOwnedStr;

pub struct UnconfiguredScenario<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    ctx: Context<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl> From<FeatureContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>>
    for UnconfiguredScenario<
        FeatureBackgroundGivenStepFnImpl,
        NoOpFeatureBackgroundGivenStepFnImpl<WorldImpl>,
        WorldImpl,
    >
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(feature: FeatureContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        Self {
            ctx: Context::Feature(feature),
        }
    }
}

type NoOpFeatureBackgroundGivenStepFnImpl<WorldImpl> = fn(&mut WorldImpl) -> Result<(), libtest::Failed>;

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    From<RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>>
    for UnconfiguredScenario<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(rule: RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        Self {
            ctx: Context::Rule(rule),
        }
    }
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    UnconfiguredScenario<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn named(
        self,
        description: impl Into<MaybeOwnedStr>,
    ) -> ScenarioWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    {
        ScenarioWithDescriptionLastConfigured {
            description: Some(description.into()),

            ctx: self.ctx,
        }
    }

    pub fn unnamed(
        self,
    ) -> ScenarioWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    {
        ScenarioWithDescriptionLastConfigured {
            description: None,

            ctx: self.ctx,
        }
    }
}

pub struct ScenarioWithDescriptionLastConfigured<
    FeatureBackgroundGivenStepFnImpl,
    RuleBackgroundGivenStepFnImpl,
    WorldImpl,
> {
    description: Option<MaybeOwnedStr>,

    ctx: Context<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    ScenarioWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn ignored(
        self,
        ignored: impl Into<bool>,
    ) -> ScenarioWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    {
        ScenarioWithIgnoredLastConfigured {
            description: self.description,
            ignored: Some(ignored.into()),

            ctx: self.ctx,
        }
    }

    pub fn given<ScenarioGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: ScenarioGivenStepFnImpl,
    ) -> ScenarioWithGivenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    where
        ScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::Given,
                description: description.into(),
            },
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            ignored: None,

            given_steps: Steps::from(step),

            ctx: self.ctx,
        }
    }
}

pub struct ScenarioWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
{
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    ctx: Context<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    ScenarioWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn given<ScenarioGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: ScenarioGivenStepFnImpl,
    ) -> ScenarioWithGivenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    where
        ScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::Given,
                description: description.into(),
            },
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: Steps::from(step),

            ctx: self.ctx,
        }
    }
}

pub struct ScenarioWithGivenStepsLastConfigured<
    ScenarioGivenStepFnImpl,
    FeatureBackgroundGivenStepFnImpl,
    RuleBackgroundGivenStepFnImpl,
    WorldImpl,
> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: Steps<ScenarioGivenStepFnImpl>,

    ctx: Context<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<ScenarioGivenStepFnImpl, FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    ScenarioWithGivenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
where
    ScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherScenarioGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioGivenStepFnImpl,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl ScenarioGivenStepFn<WorldImpl>,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
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

            given_steps: self.given_steps.chain_scenario_given_step(step),

            ctx: self.ctx,
        }
    }

    pub fn but<OtherScenarioGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioGivenStepFnImpl,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl ScenarioGivenStepFn<WorldImpl>,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
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

            given_steps: self.given_steps.chain_scenario_given_step(step),

            ctx: self.ctx,
        }
    }

    pub fn when<ScenarioWhenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: ScenarioWhenStepFnImpl,
    ) -> ScenarioWithWhenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    where
        ScenarioWhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
    {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::When,
                description: description.into(),
            },
            callback,
        };

        ScenarioWithWhenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: Steps::from(step),

            ctx: self.ctx,
        }
    }
}

pub struct ScenarioWithWhenStepsLastConfigured<
    ScenarioGivenStepFnImpl,
    ScenarioWhenStepFnImpl,
    FeatureBackgroundGivenStepFnImpl,
    RuleBackgroundGivenStepFnImpl,
    WorldImpl,
> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: Steps<ScenarioGivenStepFnImpl>,
    when_steps: Steps<ScenarioWhenStepFnImpl>,

    ctx: Context<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    ScenarioWithWhenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
where
    ScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    ScenarioWhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherScenarioWhenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioWhenStepFnImpl,
    ) -> ScenarioWithWhenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        impl ScenarioWhenStepFn<WorldImpl>,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
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
            when_steps: self.when_steps.chain_scenario_when_step(step),

            ctx: self.ctx,
        }
    }

    pub fn but<OtherScenarioWhenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioWhenStepFnImpl,
    ) -> ScenarioWithWhenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        impl ScenarioWhenStepFn<WorldImpl>,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
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
            when_steps: self.when_steps.chain_scenario_when_step(step),

            ctx: self.ctx,
        }
    }

    pub fn then<ScenarioThenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: ScenarioThenStepFnImpl,
    ) -> ScenarioWithThenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        ScenarioThenStepFnImpl,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    where
        ScenarioThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
    {
        let step = Step {
            meta: StepMeta {
                label: StepLabel::Then,
                description: description.into(),
            },
            callback,
        };

        ScenarioWithThenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps: Steps::from(step),

            ctx: self.ctx,
        }
    }
}

pub struct ScenarioWithThenStepsLastConfigured<
    ScenarioGivenStepFnImpl,
    ScenarioWhenStepFnImpl,
    ScenarioThenStepFnImpl,
    FeatureBackgroundGivenStepFnImpl,
    RuleBackgroundGivenStepFnImpl,
    WorldImpl,
> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: Steps<ScenarioGivenStepFnImpl>,
    when_steps: Steps<ScenarioWhenStepFnImpl>,
    then_steps: Steps<ScenarioThenStepFnImpl>,

    ctx: Context<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        ScenarioThenStepFnImpl,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    ScenarioWithThenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        ScenarioThenStepFnImpl,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
where
    ScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    ScenarioWhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
    ScenarioThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherScenarioThenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioThenStepFnImpl,
    ) -> ScenarioWithThenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        impl ScenarioThenStepFn<WorldImpl>,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
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
            then_steps: self.then_steps.chain_scenario_then_step(step),

            ctx: self.ctx,
        }
    }

    pub fn but<OtherScenarioThenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioThenStepFnImpl,
    ) -> ScenarioWithThenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        impl ScenarioThenStepFn<WorldImpl>,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
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
            then_steps: self.then_steps.chain_scenario_then_step(step),

            ctx: self.ctx,
        }
    }
}

pub trait FinalizableScenario: Into<libtest::Trial> {}

impl<T> FinalizableScenario for T
where
    T: Into<libtest::Trial>,
{
}

impl<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        ScenarioThenStepFnImpl,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    From<
        ScenarioWithThenStepsLastConfigured<
            ScenarioGivenStepFnImpl,
            ScenarioWhenStepFnImpl,
            ScenarioThenStepFnImpl,
            FeatureBackgroundGivenStepFnImpl,
            RuleBackgroundGivenStepFnImpl,
            WorldImpl,
        >,
    > for libtest::Trial
where
    ScenarioGivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
    ScenarioWhenStepFnImpl: ScenarioWhenStepFn<WorldImpl>,
    ScenarioThenStepFnImpl: ScenarioThenStepFn<WorldImpl>,
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(
        scenario: ScenarioWithThenStepsLastConfigured<
            ScenarioGivenStepFnImpl,
            ScenarioWhenStepFnImpl,
            ScenarioThenStepFnImpl,
            FeatureBackgroundGivenStepFnImpl,
            RuleBackgroundGivenStepFnImpl,
            WorldImpl,
        >,
    ) -> Self {
        let scenario_description = match scenario.description {
            Some(description) => description,
            None => format!(
                "{}{}{}{}{}",
                scenario.given_steps, STEP_DELIMITER, scenario.when_steps, STEP_DELIMITER, scenario.then_steps
            )
            .into(),
        };

        match scenario.ctx {
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
                None => "".into(),
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
            .with_ignored_flag(
                rule.feature.ignored.unwrap_or(false)
                    || rule.ignored.unwrap_or(false)
                    || scenario.ignored.unwrap_or(false),
            )
            .with_kind(match (rule.feature.description, rule.description) {
                (Some(feature_description), Some(rule_description)) =>
                    format!("{}{}{}", feature_description, STEP_DELIMITER, rule_description).into(),
                (Some(feature_description), None) => feature_description,
                (None, Some(rule_description)) => rule_description,
                (None, None) => "".into(),
            }),
        }
    }
}

enum Context<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    Feature(FeatureContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>),
    Rule(RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>),
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepImpl, WorldImpl> Clone
    for Context<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn clone(&self) -> Self {
        match self {
            Self::Feature(feature) => Self::Feature(feature.clone()),
            Self::Rule(rule) => Self::Rule(rule.clone()),
        }
    }
}
