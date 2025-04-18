pub use UnconfiguredScenario as Scenario;

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
use crate::utils::aliases::Arc;
use crate::utils::aliases::MaybeOwnedStr;

use super::BackgroundPayload;
use super::HookFn;
use super::Hooks;
use super::Tag;
use super::Tags;

pub struct UnconfiguredScenario<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    ctx: ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> From<ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>> for UnconfiguredScenario<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
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

    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> ScenarioWithTagLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
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
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
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

            given_steps: GivenSteps::from(step),

            ctx: 
        }
    }

    fn hook_given_step(ctx: &ResolvedScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>, callback: impl GivenStepFn<WorldImpl>) -> impl GivenStepFn<WorldImpl>
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

pub struct ScenarioWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
{
    description: Option<MaybeOwnedStr>,
    
    ctx: ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    ScenarioWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> ScenarioWithTagLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
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
        FeatureBackgroundGivenStepFnImpl,
        RuleBackgroundGivenStepFnImpl,
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

            given_steps: GivenSteps::from(step),

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

pub struct ScenarioWithTagLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    
    ctx: ResolvedScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> ScenarioWithTagLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
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
            tags: self.tags,

            given_steps: GivenSteps::from(step),

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
    ScenarioGivenStepFnImpl,
    FeatureBackgroundGivenStepFnImpl,
    RuleBackgroundGivenStepFnImpl,
    WorldImpl,
> {
    description: Option<MaybeOwnedStr>,

    given_steps: GivenSteps<ScenarioGivenStepFnImpl>,
    
    ctx: ResolvedScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
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
            tags: self.tags,

            given_steps: self.given_steps.chain(step),

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
            tags: self.tags,

            given_steps: self.given_steps.chain(step),

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
            tags: self.tags,

            given_steps: self.given_steps,
            when_steps: WhenSteps::from(step),

            feature: self.feature,
            rule: self.rule,
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

    given_steps: GivenSteps<ScenarioGivenStepFnImpl>,
    when_steps: WhenSteps<ScenarioWhenStepFnImpl>,
    
    ctx: ResolvedScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
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
    ScenarioGivenStepFnImpl,
    ScenarioWhenStepFnImpl,
    ScenarioThenStepFnImpl,
    FeatureBackgroundGivenStepFnImpl,
    RuleBackgroundGivenStepFnImpl,
    WorldImpl,
> {
    description: Option<MaybeOwnedStr>,
    
    given_steps: GivenSteps<ScenarioGivenStepFnImpl>,
    when_steps: WhenSteps<ScenarioWhenStepFnImpl>,
    then_steps: ThenSteps<ScenarioThenStepFnImpl>,
    
    ctx: ResolvedScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>,
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

pub struct ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    pub(super) feature_description: Option<MaybeOwnedStr>,
    pub(super) rule_description: Option<MaybeOwnedStr>,
    pub(super) ignored: Option<bool>,
    pub(super) tags: Tags,

    pub(super) before_scenario_hooks: Hooks<WorldImpl>,
    pub(super) after_scenario_hooks: Hooks<WorldImpl>,
    pub(super) before_step_hooks: Hooks<WorldImpl>,
    pub(super) after_step_hooks: Hooks<WorldImpl>,

    pub(super) feature_background: Option<BackgroundPayload<FeatureBackgroundGivenStepFnImpl, WorldImpl>>,
    pub(super) rule_background: Option<BackgroundPayload<RuleBackgroundGivenStepFnImpl, WorldImpl>>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn resolve(self) -> ResolvedScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
        self.into()
    }
}

struct ResolvedScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    feature_description: Option<MaybeOwnedStr>,
    rule_description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    before_scenario_hooks_callback: Option<Arc<dyn HookFn<WorldImpl>>>,
    after_scenario_hooks_callback: Option<Arc<dyn HookFn<WorldImpl>>>,
    before_step_hooks_callback: Option<Arc<dyn HookFn<WorldImpl>>>,
    after_step_hooks_callback: Option<Arc<dyn HookFn<WorldImpl>>>,

    feature_background: Option<BackgroundPayload<FeatureBackgroundGivenStepFnImpl, WorldImpl>>,
    rule_background: Option<BackgroundPayload<RuleBackgroundGivenStepFnImpl, WorldImpl>>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> From<ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>> for ResolvedScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(ctx: ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Self {
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

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> ResolvedScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn hook_given_step(&self, callback: impl GivenStepFn<WorldImpl>) -> impl GivenStepFn<WorldImpl> {
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

    fn hook_when_step(&self, callback: impl WhenStepFn<WorldImpl>) -> impl WhenStepFn<WorldImpl> {
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
    
    fn hook_then_step(&self, callback: impl ThenStepFn<WorldImpl>) -> impl ThenStepFn<WorldImpl> {
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