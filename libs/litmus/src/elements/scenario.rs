pub use UnconfiguredScenario as Scenario;

use crate::elements::ReusableHookedStepFn;
use crate::elements::GivenStepFn;
use crate::elements::ThenStepFn;
use crate::elements::WhenStepFn;
use crate::elements::Step;
use crate::elements::StepLabel;
use crate::elements::GivenSteps;
use crate::elements::WhenSteps;
use crate::elements::ThenSteps;
use crate::elements::World;
use crate::utils::aliases::Arc;
use crate::utils::aliases::MaybeOwnedStr;

use super::BackgroundPayload;
use super::HookFn;
use super::HookedStepFn;
use super::HookedSteps;
use super::Hooks;
use super::Tag;
use super::Tags;

pub struct UnconfiguredScenario<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl> {
    ctx: ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl> From<ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>> for UnconfiguredScenario<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(ctx: ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>) -> Self {
        Self {
            ctx,
        }
    }
}

impl<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    UnconfiguredScenario<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn named(
        self,
        description: impl Into<MaybeOwnedStr>,
    ) -> ScenarioWithDescriptionLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    {
        ScenarioWithDescriptionLastConfigured {
            description: Some(description.into()),

            ctx: self.ctx,
        }
    }

    pub fn unnamed(
        self,
    ) -> ScenarioWithDescriptionLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    {
        ScenarioWithDescriptionLastConfigured {
            description: None,

            ctx: self.ctx,
        }
    }
}

pub struct ScenarioWithDescriptionLastConfigured<
    FeatureBackgroundHookedStepFnImpl,
    RuleBackgroundHookedStepFnImpl,
    WorldImpl,
> {
    description: Option<MaybeOwnedStr>,
    
    ctx: ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    ScenarioWithDescriptionLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn ignored(
        self,
        ignored: impl Into<bool>,
    ) -> ScenarioWithIgnoredLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    {
        let scenario_ignored = ignored.into();

        ScenarioWithIgnoredLastConfigured {
            description: self.description,

            ctx: ScenarioContext {
                ignored: Some(match self.ctx.ignored {
                    Some(feature_or_rule_ignored) => feature_or_rule_ignored || scenario_ignored,
                    None => scenario_ignored,
                }),
                ..self.ctx
            },
        }
    }

    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> ScenarioWithTagLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        U: Into<Tag>,
    {
        let scenario_tags = Tags::from_iter(tags);

        ScenarioWithTagLastConfigured {
            description: self.description,

            ctx: ScenarioContext {
                tags: self.ctx.tags.union(scenario_tags),
                ..self.ctx
            }.resolve(),
        }
    }

    pub fn given(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl GivenStepFn<WorldImpl>,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl GivenStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let callback = self.hook_given_step(callback);

        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,

            steps: GivenSteps::from(step),

            ctx: 
        }
    }

    fn hook_given_step(ctx: &ResolvedScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>, callback: impl GivenStepFn<WorldImpl>) -> impl GivenStepFn<WorldImpl>
    where
        WorldImpl: World,
    {
        let before_step_hooks_callback = ctx.before_step_hooks_callback.clone();
        let after_step_hooks_callback = ctx.after_step_hooks_callback.clone();

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

pub struct ScenarioWithIgnoredLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
{
    description: Option<MaybeOwnedStr>,
    
    ctx: ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    ScenarioWithIgnoredLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> ScenarioWithTagLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        U: Into<Tag>,
    {
        ScenarioWithTagLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::from_iter(tags),

            feature: self.feature,
            rule: self.rule,
        }
    }

    pub fn given(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl GivenStepFn<WorldImpl>,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl GivenStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let callback = self.hook_given_step(callback);

        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::default(),

            steps: GivenSteps::from(step),

            feature: self.feature,
            rule: self.rule,
        }
    }

    fn hook_given_step(&self, callback: impl GivenStepFn<WorldImpl>) -> impl GivenStepFn<WorldImpl>
    where
        WorldImpl: World,
    {
        let before_step_hooks_callback = self.feature.before_step_hooks.to_callback(self.tags());
        let after_step_hooks_callback = self.feature.after_step_hooks.to_callback(self.tags());

        move |world| {
            (before_step_hooks_callback)(world);
            let result = (callback)(world);
            (after_step_hooks_callback)(world);

            result
        }
    }

    fn tags(&self) -> impl Iterator<Item = &Tag> {
        self.feature.tags.iter()
            .chain(self.rule.iter()
                .flat_map(|rule| rule.tags.iter()))
    }
}

pub struct ScenarioWithTagLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    
    ctx: ResolvedScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl> ScenarioWithTagLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn given<ScenarioGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: ScenarioGivenStepFnImpl,
    ) -> ScenarioWithGivenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
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
            tags: self.tags,

            steps: GivenSteps::from(step),

            feature: self.feature,
            rule: self.rule,
        }
    }

    fn hook_given_step(&self, callback: impl GivenStepFn<WorldImpl>) -> impl GivenStepFn<WorldImpl>
    where
        WorldImpl: World,
    {
        let before_step_hooks_callback = self.feature.before_step_hooks.to_callback(self.tags());
        let after_step_hooks_callback = self.feature.after_step_hooks.to_callback(self.tags());

        move |world| {
            (before_step_hooks_callback)(world);
            let result = (callback)(world);
            (after_step_hooks_callback)(world);

            result
        }
    }

    fn tags(&self) -> impl Iterator<Item = &Tag> {
        self.tags.iter()
            .chain(self.feature.tags.iter())
            .chain(self.rule.iter()
                .flat_map(|rule| rule.tags.iter()))
    }
}

pub struct ScenarioWithGivenStepsLastConfigured<
    ScenarioHookedStepFnImpl,
    FeatureBackgroundHookedStepFnImpl,
    RuleBackgroundHookedStepFnImpl,
    WorldImpl,
> {
    description: Option<MaybeOwnedStr>,
    steps: HookedSteps<ScenarioHookedStepFnImpl>,
    
    ctx: ResolvedScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>,
}

impl<ScenarioGivenStepFnImpl, FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    ScenarioWithGivenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
where
    ScenarioGivenStepFnImpl: GivenStepFn<WorldImpl>,
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherScenarioGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioGivenStepFnImpl,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl GivenStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
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
            tags: self.tags,

            steps: self.steps.chain(step),

            feature: self.feature,
            rule: self.rule,
        }
    }

    pub fn but<OtherScenarioGivenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioGivenStepFnImpl,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl GivenStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
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
            tags: self.tags,

            steps: self.steps.chain(step),

            feature: self.feature,
            rule: self.rule,
        }
    }

    pub fn when<ScenarioWhenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: ScenarioWhenStepFnImpl,
    ) -> ScenarioWithWhenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
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
            tags: self.tags,

            given_steps: self.steps,
            when_steps: WhenSteps::from(step),

            feature: self.feature,
            rule: self.rule,
        }
    }
}

pub struct ScenarioWithWhenStepsLastConfigured<
    ScenarioHookedStepFnImpl,
    FeatureBackgroundHookedStepFnImpl,
    RuleBackgroundHookedStepFnImpl,
    WorldImpl,
> {
    description: Option<MaybeOwnedStr>,
    steps: HookedSteps<ScenarioHookedStepFnImpl>,
    
    ctx: ResolvedScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>,
}

impl<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    ScenarioWithWhenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
where
    ScenarioGivenStepFnImpl: GivenStepFn<WorldImpl>,
    ScenarioWhenStepFnImpl: WhenStepFn<WorldImpl>,
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and<OtherScenarioWhenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioWhenStepFnImpl,
    ) -> ScenarioWithWhenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        impl WhenStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
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
            tags: self.tags,

            given_steps: self.given_steps,
            when_steps: self.when_steps.chain(step),

            feature: self.feature,
            rule: self.rule,
        }
    }

    pub fn but<OtherScenarioWhenStepFnImpl>(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: OtherScenarioWhenStepFnImpl,
    ) -> ScenarioWithWhenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        impl WhenStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
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
            tags: self.tags,

            given_steps: self.given_steps,
            when_steps: self.when_steps.chain(step),

            feature: self.feature,
            rule: self.rule,
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
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
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
            tags: self.tags,

            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps: ThenSteps::from(step),

            feature: self.feature,
            rule: self.rule,
        }
    }
}

pub struct ScenarioWithThenStepsLastConfigured<
    ScenarioHookedStepFnImpl,
    FeatureBackgroundHookedStepFnImpl,
    RuleBackgroundHookedStepFnImpl,
    WorldImpl,
> {
    description: Option<MaybeOwnedStr>,
    steps: HookedSteps<ScenarioHookedStepFnImpl>,
    
    ctx: ResolvedScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>,
}

impl<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        ScenarioThenStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    ScenarioWithThenStepsLastConfigured<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        ScenarioThenStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
where
    ScenarioGivenStepFnImpl: GivenStepFn<WorldImpl>,
    ScenarioWhenStepFnImpl: WhenStepFn<WorldImpl>,
    ScenarioThenStepFnImpl: ThenStepFn<WorldImpl>,
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
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
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
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
            tags: self.tags,

            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps: self.then_steps.chain(step),

            feature: self.feature,
            rule: self.rule,
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
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
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
            tags: self.tags,

            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps: self.then_steps.chain(step),

            feature: self.feature,
            rule: self.rule,
        }
    }
}

pub trait FinalizableScenario: Into<libtest::Trial> {}

impl<T> FinalizableScenario for T where T: Into<libtest::Trial> {}

impl<
        ScenarioGivenStepFnImpl,
        ScenarioWhenStepFnImpl,
        ScenarioThenStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    From<
        ScenarioWithThenStepsLastConfigured<
            ScenarioGivenStepFnImpl,
            ScenarioWhenStepFnImpl,
            ScenarioThenStepFnImpl,
            FeatureBackgroundHookedStepFnImpl,
            RuleBackgroundHookedStepFnImpl,
            WorldImpl,
        >,
    > for libtest::Trial
where
    ScenarioGivenStepFnImpl: GivenStepFn<WorldImpl>,
    ScenarioWhenStepFnImpl: WhenStepFn<WorldImpl>,
    ScenarioThenStepFnImpl: ThenStepFn<WorldImpl>,
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(
        scenario: ScenarioWithThenStepsLastConfigured<
            ScenarioGivenStepFnImpl,
            ScenarioWhenStepFnImpl,
            ScenarioThenStepFnImpl,
            FeatureBackgroundHookedStepFnImpl,
            RuleBackgroundHookedStepFnImpl,
            WorldImpl,
        >,
    ) -> Self {
        let description = match scenario.description {
            Some(description) => description,
            None => format!("{} | {} | {}", scenario.given_steps, scenario.when_steps, scenario.then_steps).into(),
        };

        let ignored = scenario.feature.ignored.unwrap_or(false)
            || scenario.rule.as_ref().and_then(|rule| rule.ignored).unwrap_or(false)
            || scenario.ignored.unwrap_or(false);

        // ORDERING WILL NEED TO BE REVIEWED AGAIN !!!
        libtest::Trial::test(description, move || {
            let mut world = WorldImpl::default();

            scenario.feature.before_scenario_hooks.untagged
                .map(|hook| (hook)(&mut world));

            scenario.tags.iter()
                .chain(scenario.feature.tags.iter())
                .chain(scenario.rule.clone().iter().flat_map(|rule| rule.tags.iter()))
                .filter_map(|tag| scenario.feature.before_scenario_hooks.tagged.get(tag))
                .for_each(|hook| hook(&mut world));
            
            scenario.feature.background
                .filter(|background| !background.ignored)
                .map(|background| (background.given_steps_callback)(&mut world))
                .transpose()?;
            
            scenario.rule.clone()
                .and_then(|rule| rule.background)
                .filter(|background| !background.ignored)
                .map(|background| (background.given_steps_callback)(&mut world))
                .transpose()?;

            (scenario.given_steps.callback)(&mut world)?;
            (scenario.when_steps.callback)(&mut world)?;
            (scenario.then_steps.callback)(&world)?;

            scenario.feature.after_scenario_hooks.untagged
                .map(|hook| (hook)(&mut world));

            scenario.tags.iter()
                .chain(scenario.feature.tags.iter())
                .chain(scenario.rule.iter().flat_map(|rule| rule.tags.iter()))
                .filter_map(|tag| scenario.feature.after_scenario_hooks.tagged.get(tag))
                .for_each(|hook| hook(&mut world));

            Ok(())
        })
            .with_ignored_flag(ignored)
    }
}

pub struct ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl> {
    pub(super) feature_description: Option<MaybeOwnedStr>,
    pub(super) rule_description: Option<MaybeOwnedStr>,
    pub(super) ignored: Option<bool>,
    pub(super) tags: Tags,

    pub(super) before_scenario_hooks: Hooks<WorldImpl>,
    pub(super) after_scenario_hooks: Hooks<WorldImpl>,
    pub(super) before_step_hooks: Hooks<WorldImpl>,
    pub(super) after_step_hooks: Hooks<WorldImpl>,

    pub(super) feature_background: Option<BackgroundPayload<FeatureBackgroundHookedStepFnImpl, WorldImpl>>,
    pub(super) rule_background: Option<BackgroundPayload<RuleBackgroundHookedStepFnImpl, WorldImpl>>,
}

impl<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl> ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn resolve(self) -> ResolvedScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl> {
        self.into()
    }
}

struct ResolvedScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl> {
    feature_description: Option<MaybeOwnedStr>,
    rule_description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    before_scenario_hooks_callback: Option<Arc<dyn HookFn<WorldImpl>>>,
    after_scenario_hooks_callback: Option<Arc<dyn HookFn<WorldImpl>>>,
    before_step_hooks_callback: Option<Arc<dyn HookFn<WorldImpl>>>,
    after_step_hooks_callback: Option<Arc<dyn HookFn<WorldImpl>>>,

    feature_background: Option<BackgroundPayload<FeatureBackgroundHookedStepFnImpl, WorldImpl>>,
    rule_background: Option<BackgroundPayload<RuleBackgroundHookedStepFnImpl, WorldImpl>>,
}

impl<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl> From<ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>> for ResolvedScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(ctx: ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>) -> Self {
        Self {
            feature_description: ctx.feature_description,
            rule_description: ctx.rule_description,
            ignored: ctx.ignored,
            
            before_scenario_hooks_callback: ctx.before_scenario_hooks.to_callback(ctx.tags.iter()),
            after_scenario_hooks_callback: ctx.after_scenario_hooks.to_callback(ctx.tags.iter()),
            before_step_hooks_callback: ctx.before_step_hooks.to_callback(ctx.tags.iter()),
            after_step_hooks_callback: ctx.after_step_hooks.to_callback(ctx.tags.iter()),
            
            tags: ctx.tags, // Placed here to avoid cloning

            feature_background: ctx.feature_background,
            rule_background: ctx.rule_background,
        }
    }
}

impl<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl> ResolvedScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn hook_given_step(&self, callback: impl GivenStepFn<WorldImpl>) -> impl HookedStepFn<WorldImpl> {
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

    fn hook_when_step(&self, callback: impl WhenStepFn<WorldImpl>) -> impl HookedStepFn<WorldImpl> {
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
    
    fn hook_then_step(&self, callback: impl ThenStepFn<WorldImpl>) -> impl HookedStepFn<WorldImpl> {
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