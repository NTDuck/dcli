pub use UnconfiguredScenario as Scenario;

use crate::elements::ReusableHookedStepFn;
use crate::elements::GivenStepFn;
use crate::elements::ThenStepFn;
use crate::elements::WhenStepFn;
use crate::elements::Step;
use crate::elements::StepLabel;
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

        let ctx = ScenarioContext {
            ignored: Some(match self.ctx.ignored {
                Some(feature_or_rule_ignored) => feature_or_rule_ignored || scenario_ignored,
                None => scenario_ignored,
            }),
            ..self.ctx
        };

        ScenarioWithIgnoredLastConfigured {
            description: self.description,

            ctx,
        }
    }

    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> ScenarioWithTagLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        U: Into<Tag>,
    {
        let scenario_tags = Tags::from_iter(tags);

        let ctx = ScenarioContext {
            tags: self.ctx.tags.union(scenario_tags),
            ..self.ctx
        };
        let ctx = ctx.resolve();

        ScenarioWithTagLastConfigured {
            description: self.description,

            ctx,
        }
    }

    pub fn given(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl GivenStepFn<WorldImpl>,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl HookedStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let ctx = self.ctx.resolve();
        let callback = ctx.hook_given_step(callback);

        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,

            steps: HookedSteps::from(step),

            ctx,
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
        let scenario_tags = Tags::from_iter(tags);

        let ctx = ScenarioContext {
            tags: self.ctx.tags.union(scenario_tags),
            ..self.ctx
        };
        let ctx = ctx.resolve();

        ScenarioWithTagLastConfigured {
            description: self.description,

            ctx,
        }
    }

    pub fn given(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl GivenStepFn<WorldImpl>,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl HookedStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let ctx = self.ctx.resolve();
        let callback = ctx.hook_given_step(callback);

        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,

            steps: HookedSteps::from(step),

            ctx,
        }
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
    pub fn given(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl GivenStepFn<WorldImpl>,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl HookedStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let callback = self.ctx.hook_given_step(callback);

        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,

            steps: HookedSteps::from(step),

            ctx: self.ctx,
        }
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

impl<ScenarioHookedStepFnImpl, FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    ScenarioWithGivenStepsLastConfigured<
        ScenarioHookedStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
where
    ScenarioHookedStepFnImpl: HookedStepFn<WorldImpl>,
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl GivenStepFn<WorldImpl>,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl HookedStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let callback = self.ctx.hook_given_step(callback);

        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            steps: self.steps.chain(step),

            ctx: self.ctx,
        }
    }

    pub fn but(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl GivenStepFn<WorldImpl>,
    ) -> ScenarioWithGivenStepsLastConfigured<
        impl HookedStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let callback = self.ctx.hook_given_step(callback);

        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback,
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            steps: self.steps.chain(step),

            ctx: self.ctx,
        }
    }

    pub fn when(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl WhenStepFn<WorldImpl>,
    ) -> ScenarioWithWhenStepsLastConfigured<
        impl HookedStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let callback = self.ctx.hook_when_step(callback);

        let step = Step {
            label: StepLabel::When,
            description: description.into(),
            callback,
        };

        ScenarioWithWhenStepsLastConfigured {
            description: self.description,
            steps: self.steps.chain(step),

            ctx: self.ctx,
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
        ScenarioHookedStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    ScenarioWithWhenStepsLastConfigured<
        ScenarioHookedStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
where
    ScenarioHookedStepFnImpl: HookedStepFn<WorldImpl>,
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl WhenStepFn<WorldImpl>,
    ) -> ScenarioWithWhenStepsLastConfigured<
        impl HookedStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let callback = self.ctx.hook_when_step(callback);

        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback,
        };

        ScenarioWithWhenStepsLastConfigured {
            description: self.description,
            steps: self.steps.chain(step),

            ctx: self.ctx,
        }
    }

    pub fn but(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl WhenStepFn<WorldImpl>,
    ) -> ScenarioWithWhenStepsLastConfigured<
        impl HookedStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let callback = self.ctx.hook_when_step(callback);

        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback,
        };

        ScenarioWithWhenStepsLastConfigured {
            description: self.description,
            steps: self.steps.chain(step),

            ctx: self.ctx,
        }
    }

    pub fn then(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl ThenStepFn<WorldImpl>,
    ) -> ScenarioWithThenStepsLastConfigured<
        impl HookedStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let callback = self.ctx.hook_then_step(callback);

        let step = Step {
            label: StepLabel::Then,
            description: description.into(),
            callback,
        };

        ScenarioWithThenStepsLastConfigured {
            description: self.description,
            steps: self.steps.chain(step),

            ctx: self.ctx,
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
        ScenarioHookedStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    ScenarioWithThenStepsLastConfigured<
        ScenarioHookedStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
where
    ScenarioHookedStepFnImpl: HookedStepFn<WorldImpl>,
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl ThenStepFn<WorldImpl>,
    ) -> ScenarioWithThenStepsLastConfigured<
        impl HookedStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let callback = self.ctx.hook_then_step(callback);

        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback,
        };

        ScenarioWithThenStepsLastConfigured {
            description: self.description,
            steps: self.steps.chain(step),

            ctx: self.ctx,
        }
    }

    pub fn but(
        self,
        description: impl Into<MaybeOwnedStr>,
        callback: impl ThenStepFn<WorldImpl>,
    ) -> ScenarioWithThenStepsLastConfigured<
        impl HookedStepFn<WorldImpl>,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    {
        let callback = self.ctx.hook_then_step(callback);

        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback,
        };

        ScenarioWithThenStepsLastConfigured {
            description: self.description,
            steps: self.steps.chain(step),

            ctx: self.ctx,
        }
    }
}

pub trait FinalizableScenario: Into<libtest::Trial> {}

impl<T> FinalizableScenario for T where T: Into<libtest::Trial> {}

impl<
        ScenarioHookedStepFnImpl,
        FeatureBackgroundHookedStepFnImpl,
        RuleBackgroundHookedStepFnImpl,
        WorldImpl,
    >
    From<
        ScenarioWithThenStepsLastConfigured<
            ScenarioHookedStepFnImpl,
            FeatureBackgroundHookedStepFnImpl,
            RuleBackgroundHookedStepFnImpl,
            WorldImpl,
        >,
    > for libtest::Trial
where
    ScenarioHookedStepFnImpl: HookedStepFn<WorldImpl>,
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(
        scenario: ScenarioWithThenStepsLastConfigured<
            ScenarioHookedStepFnImpl,
            FeatureBackgroundHookedStepFnImpl,
            RuleBackgroundHookedStepFnImpl,
            WorldImpl,
        >,
    ) -> Self {
        let _ = scenario.ctx.feature_description;
        let _ = scenario.ctx.rule_description;

        let description = match scenario.description {
            Some(description) => description,
            None => format!("{}", scenario.steps).into(),
        };

        libtest::Trial::test(description, move || {
            let mut world = WorldImpl::default();

            scenario.ctx.before_scenario_hooks_callback
                .as_ref()
                .map(|hook| (hook)(&mut world));
            
            let result = {
                scenario.ctx.feature_background
                    .filter(|background| background.ignored.map_or(true, |ignored| !ignored))
                    .map(|background| (background.steps_callback)(&mut world))
                    .transpose()?;

                scenario.ctx.rule_background
                    .filter(|background| background.ignored.map_or(true, |ignored| !ignored))
                    .map(|background| (background.steps_callback)(&mut world))
                    .transpose()?;

                (scenario.steps.callback)(&mut world)
            };
            
            scenario.ctx.after_scenario_hooks_callback
                .as_ref()
                .map(|hook| (hook)(&mut world));

            result
        })
            .with_ignored_flag(scenario.ctx.ignored.unwrap_or(false))
            .with_kind(scenario.ctx.tags
                .iter()
                .map(|tag| tag.as_ref())
                .collect::<Vec<_>>()
                .join(", "))
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