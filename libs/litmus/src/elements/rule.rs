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

pub struct UnconfiguredRule<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    ctx: RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl> From<RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>>
    for UnconfiguredRule<FeatureBackgroundGivenStepFnImpl, WorldImpl>
{
    fn from(ctx: RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        Self {
            ctx,
        }
    }
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl> UnconfiguredRule<FeatureBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn named(
        self,
        description: impl Into<MaybeOwnedStr>,
    ) -> RuleWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
        RuleWithDescriptionLastConfigured {
            description: Some(description.into()),

            ctx: self.ctx,
        }
    }
}

pub struct RuleWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,

    ctx: RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn ignored(
        self,
        ignored: impl Into<bool>,
    ) -> RuleWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
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

    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> RuleWithTagLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
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

    pub fn background<FromContext, Background, RuleBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<RuleBackgroundGivenStepFnImpl, WorldImpl>,
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        RuleWithBackgroundLastConfigured {
            description: self.description,
            background: Some(background.into()),

            ctx: self.ctx.clone(),
        }
    }

    pub fn scenario<FromContext, Scenario, RuleBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
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

    fn to_scenario_ctx<RuleBackgroundGivenStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
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

pub struct RuleWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,

    ctx: RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> RuleWithTagLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
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

    pub fn background<FromContext, Background, RuleBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<RuleBackgroundGivenStepFnImpl, WorldImpl>,
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        RuleWithBackgroundLastConfigured {
            description: self.description,
            background: Some(background.into()),

            ctx: self.ctx.clone(),
        }
    }

    pub fn scenario<FromContext, Scenario, RuleBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
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

    fn to_scenario_ctx<RuleBackgroundGivenStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
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

pub struct RuleWithTagLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,

    ctx: RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithTagLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn background<FromContext, Background, RuleBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<RuleBackgroundGivenStepFnImpl, WorldImpl>,
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        RuleWithBackgroundLastConfigured {
            description: self.description,
            background: Some(background.into()),

            ctx: self.ctx.clone(),
        }
    }

    pub fn scenario<FromContext, Scenario, RuleBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
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

    fn to_scenario_ctx<RuleBackgroundGivenStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
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

pub struct RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
{
    description: Option<MaybeOwnedStr>,
    background: Option<BackgroundPayload<RuleBackgroundGivenStepFnImpl, WorldImpl>>,

    ctx: RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn scenario<FromContext, Scenario>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
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
    
    fn to_scenario_ctx(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
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

pub struct RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    background: Option<BackgroundPayload<RuleBackgroundGivenStepFnImpl, WorldImpl>>,

    ctx: RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,

    trials: Vec<libtest::Trial>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn scenario<FromContext, Scenario>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
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
    
    fn to_scenario_ctx(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
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

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    From<RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>>
    for Vec<libtest::Trial>
where
    FeatureBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(
        rule: RuleWithScenarioLastConfigured<
            FeatureBackgroundGivenStepFnImpl,
            RuleBackgroundGivenStepFnImpl,
            WorldImpl,
        >,
    ) -> Self {
        rule.trials
    }
}

pub struct RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    pub(super) feature_description: Option<MaybeOwnedStr>,
    pub(super) ignored: Option<bool>,
    pub(super) tags: Tags,

    pub(super) before_scenario_hooks: Hooks<WorldImpl>,
    pub(super) after_scenario_hooks: Hooks<WorldImpl>,
    pub(super) before_step_hooks: Hooks<WorldImpl>,
    pub(super) after_step_hooks: Hooks<WorldImpl>,

    pub(super) feature_background: Option<BackgroundPayload<FeatureBackgroundGivenStepFnImpl, WorldImpl>>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> Clone for RuleContext<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: ReusableHookedStepFn<WorldImpl>,
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
