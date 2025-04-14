pub use UnconfiguredScenario as Scenario;

use super::FeatureContext;
use super::RuleContext;
use crate::elements::ReusableGivenStepFn;
use crate::elements::GivenStepFn;
use crate::elements::ThenStepFn;
use crate::elements::WhenStepFn;
use crate::elements::Step;
use crate::elements::StepLabel;
use crate::elements::GivenSteps;
use crate::elements::WhenSteps;
use crate::elements::ThenSteps;
use crate::elements::World;
use crate::utils::aliases::MaybeOwnedStr;

pub struct UnconfiguredScenario<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    ctx: ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> From<ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>> for UnconfiguredScenario<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    fn from(ctx: ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        Self {
            ctx,
        }
    }
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    UnconfiguredScenario<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
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

    ctx: ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    ScenarioWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
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
        ScenarioGivenStepFnImpl: GivenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            ignored: None,

            given_steps: GivenSteps::from(step),

            ctx: self.ctx,
        }
    }
}

pub struct ScenarioWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
{
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    ctx: ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    ScenarioWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
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
        ScenarioGivenStepFnImpl: GivenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: GivenSteps::from(step),

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

    given_steps: GivenSteps<ScenarioGivenStepFnImpl>,

    ctx: ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<ScenarioGivenStepFnImpl, FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    ScenarioWithGivenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
where
    ScenarioGivenStepFnImpl: GivenStepFn<WorldImpl>,
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherScenarioGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioGivenStepFnImpl,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl GivenStepFn<WorldImpl>,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    where
        OtherScenarioGivenStepFnImpl: GivenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps.chain(step),

            ctx: self.ctx,
        }
    }

    pub fn but<OtherScenarioGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioGivenStepFnImpl,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl GivenStepFn<WorldImpl>,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    where
        OtherScenarioGivenStepFnImpl: GivenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps.chain(step),

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
        ScenarioWhenStepFnImpl: WhenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::When,
            description: description.into(),
            callback,
        };

        ScenarioWithWhenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: WhenSteps::from(step),

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

    given_steps: GivenSteps<ScenarioGivenStepFnImpl>,
    when_steps: WhenSteps<ScenarioWhenStepFnImpl>,

    ctx: ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
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
    ScenarioGivenStepFnImpl: GivenStepFn<WorldImpl>,
    ScenarioWhenStepFnImpl: WhenStepFn<WorldImpl>,
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherScenarioWhenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioWhenStepFnImpl,
    ) -> ScenarioWithWhenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        impl WhenStepFn<WorldImpl>,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    where
        OtherScenarioWhenStepFnImpl: WhenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback,
        };

        ScenarioWithWhenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: self.when_steps.chain(step),

            ctx: self.ctx,
        }
    }

    pub fn but<OtherScenarioWhenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioWhenStepFnImpl,
    ) -> ScenarioWithWhenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        impl WhenStepFn<WorldImpl>,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    where
        OtherScenarioWhenStepFnImpl: WhenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback,
        };

        ScenarioWithWhenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: self.when_steps.chain(step),

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
        ScenarioThenStepFnImpl: ThenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::Then,
            description: description.into(),
            callback,
        };

        ScenarioWithThenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps: ThenSteps::from(step),

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

    given_steps: GivenSteps<ScenarioGivenStepFnImpl>,
    when_steps: WhenSteps<ScenarioWhenStepFnImpl>,
    then_steps: ThenSteps<ScenarioThenStepFnImpl>,

    ctx: ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
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
    ScenarioGivenStepFnImpl: GivenStepFn<WorldImpl>,
    ScenarioWhenStepFnImpl: WhenStepFn<WorldImpl>,
    ScenarioThenStepFnImpl: ThenStepFn<WorldImpl>,
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherScenarioThenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioThenStepFnImpl,
    ) -> ScenarioWithThenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        impl ThenStepFn<WorldImpl>,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    where
        OtherScenarioThenStepFnImpl: ThenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback,
        };

        ScenarioWithThenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps: self.then_steps.chain(step),

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
        impl ThenStepFn<WorldImpl>,
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
        WorldImpl,
    >
    where
        OtherScenarioThenStepFnImpl: ThenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback,
        };

        ScenarioWithThenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps: self.then_steps.chain(step),

            ctx: self.ctx,
        }
    }
}

pub trait FinalizableScenario: Into<libtest::Trial> {}

impl<T> FinalizableScenario for T where T: Into<libtest::Trial> {}

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
    ScenarioGivenStepFnImpl: GivenStepFn<WorldImpl>,
    ScenarioWhenStepFnImpl: WhenStepFn<WorldImpl>,
    ScenarioThenStepFnImpl: ThenStepFn<WorldImpl>,
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
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
        let description = match scenario.description {
            Some(description) => description,
            None => format!("{} | {} | {}", scenario.given_steps, scenario.when_steps, scenario.then_steps).into(),
        };

        let ignored = scenario.ctx.feature.ignored.unwrap_or(false)
            || scenario.ctx.rule.as_ref().and_then(|rule| rule.ignored).unwrap_or(false)
            || scenario.ignored.unwrap_or(false);

        libtest::Trial::test(description, move || {
            let mut world = WorldImpl::default();
            
            scenario.ctx.feature.background
                .filter(|background| !background.ignored)
                .map(|background| (background.given_steps_callback)(&mut world))
                .transpose()?;
            
            scenario.ctx.rule
                .and_then(|rule| rule.background)
                .filter(|background| !background.ignored)
                .map(|background| (background.given_steps_callback)(&mut world))
                .transpose()?;

            (scenario.given_steps.callback)(&mut world)?;
            (scenario.when_steps.callback)(&mut world)?;
            (scenario.then_steps.callback)(&world)?;

            Ok(())
        })
            .with_ignored_flag(ignored)
    }
}

pub struct ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    feature: FeatureContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
    rule: Option<RuleContext<RuleBackgroundGivenStepFnImpl, WorldImpl>>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> Clone for ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where 
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn clone(&self) -> Self {
        Self {
            feature: self.feature.clone(),
            rule: self.rule.clone(),
        }
    }
}
