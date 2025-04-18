pub use UnconfiguredRule as Rule;

use crate::elements::BackgroundPayload;
use crate::elements::ReusableHookedStepFn;
use crate::elements::FinalizableBackground;
use crate::elements::FinalizableScenario;
use crate::elements::World;
use crate::utils::aliases::MaybeOwnedStr;

use super::BackgroundContext;
use super::Hooks;
use super::ScenarioContext;
use super::Tag;
use super::Tags;

pub struct UnconfiguredRule<FeatureBackgroundHookedStepFnImpl, WorldImpl> {
    ctx: RuleContext<FeatureBackgroundHookedStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundHookedStepFnImpl, WorldImpl> From<RuleContext<FeatureBackgroundHookedStepFnImpl, WorldImpl>>
    for UnconfiguredRule<FeatureBackgroundHookedStepFnImpl, WorldImpl>
{
    fn from(ctx: RuleContext<FeatureBackgroundHookedStepFnImpl, WorldImpl>) -> Self {
        Self {
            ctx,
        }
    }
}

impl<FeatureBackgroundHookedStepFnImpl, WorldImpl> UnconfiguredRule<FeatureBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn named(
        self,
        description: impl Into<MaybeOwnedStr>,
    ) -> RuleWithDescriptionLastConfigured<FeatureBackgroundHookedStepFnImpl, WorldImpl> {
        RuleWithDescriptionLastConfigured {
            description: Some(description.into()),

            ctx: self.ctx,
        }
    }
}

pub struct RuleWithDescriptionLastConfigured<FeatureBackgroundHookedStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,

    ctx: RuleContext<FeatureBackgroundHookedStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundHookedStepFnImpl, WorldImpl>
    RuleWithDescriptionLastConfigured<FeatureBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn ignored(
        self,
        ignored: impl Into<bool>,
    ) -> RuleWithIgnoredLastConfigured<FeatureBackgroundHookedStepFnImpl, WorldImpl> {
        let rule_ignored = ignored.into();

        RuleWithIgnoredLastConfigured {
            description: self.description,

            ctx: RuleContext {
                ignored: Some(match self.ctx.ignored {
                    Some(feature_ignored) => feature_ignored || rule_ignored,
                    None => rule_ignored,
                }),
                ..self.ctx
            },
        }
    }

    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> RuleWithTagLastConfigured<FeatureBackgroundHookedStepFnImpl, WorldImpl>
    where
        U: Into<Tag>,
    {
        let rule_tags = Tags::from_iter(tags);

        RuleWithTagLastConfigured {
            description: self.description,

            ctx: RuleContext {
                tags: self.ctx.tags.union(rule_tags),
                ..self.ctx
            },
        }
    }

    pub fn background<FromContext, Background, RuleBackgroundHookedStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithBackgroundLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<RuleBackgroundHookedStepFnImpl, WorldImpl>,
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        RuleWithBackgroundLastConfigured {
            description: self.description,
            background: Some(background.into()),

            ctx: self.ctx.clone(),
        }
    }

    pub fn scenario<FromContext, Scenario, RuleBackgroundHookedStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        RuleWithScenarioLastConfigured {
            description: self.description,
            background: None,

            ctx: self.ctx,

            trials,
        }
    }
    
    fn to_background_ctx(&self) -> BackgroundContext<WorldImpl> {
        BackgroundContext {
            before_step_hooks_callback: self.ctx.before_step_hooks.untagged.clone(),
            after_step_hooks_callback: self.ctx.after_scenario_hooks.untagged.clone(),
        }
    }

    fn to_scenario_ctx<RuleBackgroundHookedStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.ctx.feature_description.clone(),
            rule_description: self.description.clone(),
            ignored: self.ctx.ignored,
            tags: self.ctx.tags.clone(),

            before_scenario_hooks: self.ctx.before_scenario_hooks.clone(),
            after_scenario_hooks: self.ctx.after_scenario_hooks.clone(),
            before_step_hooks: self.ctx.before_step_hooks.clone(),
            after_step_hooks: self.ctx.after_step_hooks.clone(),

            feature_background: self.ctx.feature_background.clone(),
            rule_background: None,
        }
    }
}

pub struct RuleWithIgnoredLastConfigured<FeatureBackgroundHookedStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,

    ctx: RuleContext<FeatureBackgroundHookedStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundHookedStepFnImpl, WorldImpl>
    RuleWithIgnoredLastConfigured<FeatureBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> RuleWithTagLastConfigured<FeatureBackgroundHookedStepFnImpl, WorldImpl>
    where
        U: Into<Tag>,
    {
        let rule_tags = Tags::from_iter(tags);

        RuleWithTagLastConfigured {
            description: self.description,

            ctx: RuleContext {
                tags: self.ctx.tags.union(rule_tags),
                ..self.ctx
            },
        }
    }

    pub fn background<FromContext, Background, RuleBackgroundHookedStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithBackgroundLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<RuleBackgroundHookedStepFnImpl, WorldImpl>,
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        RuleWithBackgroundLastConfigured {
            description: self.description,
            background: Some(background.into()),

            ctx: self.ctx.clone(),
        }
    }

    pub fn scenario<FromContext, Scenario, RuleBackgroundHookedStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        RuleWithScenarioLastConfigured {
            description: self.description,
            background: None,

            ctx: self.ctx,

            trials,
        }
    }
    
    fn to_background_ctx(&self) -> BackgroundContext<WorldImpl> {
        BackgroundContext {
            before_step_hooks_callback: self.ctx.before_step_hooks.untagged.clone(),
            after_step_hooks_callback: self.ctx.after_scenario_hooks.untagged.clone(),
        }
    }

    fn to_scenario_ctx<RuleBackgroundHookedStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.ctx.feature_description.clone(),
            rule_description: self.description.clone(),
            ignored: self.ctx.ignored,
            tags: self.ctx.tags.clone(),

            before_scenario_hooks: self.ctx.before_scenario_hooks.clone(),
            after_scenario_hooks: self.ctx.after_scenario_hooks.clone(),
            before_step_hooks: self.ctx.before_step_hooks.clone(),
            after_step_hooks: self.ctx.after_step_hooks.clone(),

            feature_background: self.ctx.feature_background.clone(),
            rule_background: None,
        }
    }
}

pub struct RuleWithTagLastConfigured<FeatureBackgroundHookedStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,

    ctx: RuleContext<FeatureBackgroundHookedStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundHookedStepFnImpl, WorldImpl>
    RuleWithTagLastConfigured<FeatureBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn background<FromContext, Background, RuleBackgroundHookedStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithBackgroundLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<RuleBackgroundHookedStepFnImpl, WorldImpl>,
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        RuleWithBackgroundLastConfigured {
            description: self.description,
            background: Some(background.into()),

            ctx: self.ctx.clone(),
        }
    }

    pub fn scenario<FromContext, Scenario, RuleBackgroundHookedStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        RuleWithScenarioLastConfigured {
            description: self.description,
            background: None,

            ctx: self.ctx,

            trials,
        }
    }
    
    fn to_background_ctx(&self) -> BackgroundContext<WorldImpl> {
        BackgroundContext {
            before_step_hooks_callback: self.ctx.before_step_hooks.untagged.clone(),
            after_step_hooks_callback: self.ctx.after_scenario_hooks.untagged.clone(),
        }
    }

    fn to_scenario_ctx<RuleBackgroundHookedStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.ctx.feature_description.clone(),
            rule_description: self.description.clone(),
            ignored: self.ctx.ignored,
            tags: self.ctx.tags.clone(),

            before_scenario_hooks: self.ctx.before_scenario_hooks.clone(),
            after_scenario_hooks: self.ctx.after_scenario_hooks.clone(),
            before_step_hooks: self.ctx.before_step_hooks.clone(),
            after_step_hooks: self.ctx.after_step_hooks.clone(),

            feature_background: self.ctx.feature_background.clone(),
            rule_background: None,
        }
    }
}

pub struct RuleWithBackgroundLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
{
    description: Option<MaybeOwnedStr>,
    background: Option<BackgroundPayload<RuleBackgroundHookedStepFnImpl, WorldImpl>>,

    ctx: RuleContext<FeatureBackgroundHookedStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    RuleWithBackgroundLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn scenario<FromContext, Scenario>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        RuleWithScenarioLastConfigured {
            description: self.description,
            background: self.background,

            ctx: self.ctx,

            trials,
        }
    }
    
    fn to_scenario_ctx(&self) -> ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.ctx.feature_description.clone(),
            rule_description: self.description.clone(),
            ignored: self.ctx.ignored,
            tags: self.ctx.tags.clone(),

            before_scenario_hooks: self.ctx.before_scenario_hooks.clone(),
            after_scenario_hooks: self.ctx.after_scenario_hooks.clone(),
            before_step_hooks: self.ctx.before_step_hooks.clone(),
            after_step_hooks: self.ctx.after_step_hooks.clone(),

            feature_background: self.ctx.feature_background.clone(),
            rule_background: self.background.clone(),
        }
    }
}

pub struct RuleWithScenarioLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    background: Option<BackgroundPayload<RuleBackgroundHookedStepFnImpl, WorldImpl>>,

    ctx: RuleContext<FeatureBackgroundHookedStepFnImpl, WorldImpl>,

    trials: Vec<libtest::Trial>,
}

impl<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    RuleWithScenarioLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn scenario<FromContext, Scenario>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);

        let mut trials = self.trials;
        trials.push(scenario.into());

        RuleWithScenarioLastConfigured {
            description: self.description,
            background: self.background,

            ctx: self.ctx,

            trials,
        }
    }
    
    fn to_scenario_ctx(&self) -> ScenarioContext<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    where
        RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.ctx.feature_description.clone(),
            rule_description: self.description.clone(),
            ignored: self.ctx.ignored,
            tags: self.ctx.tags.clone(),

            before_scenario_hooks: self.ctx.before_scenario_hooks.clone(),
            after_scenario_hooks: self.ctx.after_scenario_hooks.clone(),
            before_step_hooks: self.ctx.before_step_hooks.clone(),
            after_step_hooks: self.ctx.after_step_hooks.clone(),

            feature_background: self.ctx.feature_background.clone(),
            rule_background: self.background.clone(),
        }
    }
}

pub trait FinalizableRule: Into<Vec<libtest::Trial>> {}

impl<T> FinalizableRule for T where T: Into<Vec<libtest::Trial>> {}

impl<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>
    From<RuleWithScenarioLastConfigured<FeatureBackgroundHookedStepFnImpl, RuleBackgroundHookedStepFnImpl, WorldImpl>>
    for Vec<libtest::Trial>
where
    FeatureBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(
        rule: RuleWithScenarioLastConfigured<
            FeatureBackgroundHookedStepFnImpl,
            RuleBackgroundHookedStepFnImpl,
            WorldImpl,
        >,
    ) -> Self {
        rule.trials
    }
}

pub struct RuleContext<FeatureBackgroundHookedStepFnImpl, WorldImpl> {
    pub(super) feature_description: Option<MaybeOwnedStr>,
    pub(super) ignored: Option<bool>,
    pub(super) tags: Tags,

    pub(super) before_scenario_hooks: Hooks<WorldImpl>,
    pub(super) after_scenario_hooks: Hooks<WorldImpl>,
    pub(super) before_step_hooks: Hooks<WorldImpl>,
    pub(super) after_step_hooks: Hooks<WorldImpl>,

    pub(super) feature_background: Option<BackgroundPayload<FeatureBackgroundHookedStepFnImpl, WorldImpl>>,
}

impl<BackgroundHookedStepFnImpl, WorldImpl> Clone for RuleContext<BackgroundHookedStepFnImpl, WorldImpl>
where
    BackgroundHookedStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn clone(&self) -> Self {
        Self {
            feature_description: self.feature_description.clone(),
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone(),
            after_scenario_hooks: self.after_scenario_hooks.clone(),
            before_step_hooks: self.before_step_hooks.clone(),
            after_step_hooks: self.after_step_hooks.clone(),

            feature_background: self.feature_background.clone(),
        }
    }
}
