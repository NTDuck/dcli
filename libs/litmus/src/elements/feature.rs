use std::marker::PhantomData;

pub use UnconfiguredFeature as Feature;

use crate::elements::BackgroundPayload;
use crate::elements::ReusableGivenStepFn;
use crate::elements::FinalizableBackground;
use crate::elements::FinalizableRule;
use crate::elements::FinalizableScenario;
use crate::elements::World;
use crate::utils::aliases::MaybeOwnedStr;

use super::BackgroundContext;
use super::HookFn;
use super::Hooks;
use super::NoOpGivenStepFn;
use super::RuleContext;
use super::ScenarioContext;
use super::Tag;
use super::Tags;

pub struct UnconfiguredFeature<WorldImpl> {
    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> UnconfiguredFeature<WorldImpl>
where
    WorldImpl: World,
{
    pub fn named(description: impl Into<MaybeOwnedStr>) -> FeatureWithDescriptionLastConfigured<WorldImpl> {
        FeatureWithDescriptionLastConfigured {
            description: Some(description.into()),

            phantom: PhantomData,
        }
    }
}

pub struct FeatureWithDescriptionLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,

    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> FeatureWithDescriptionLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn ignored(self, ignored: impl Into<bool>) -> FeatureWithIgnoredLastConfigured<WorldImpl> {
        FeatureWithIgnoredLastConfigured {
            description: self.description,
            ignored: Some(ignored.into()),

            phantom: PhantomData,
        }
    }

    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> FeatureWithTagLastConfigured<WorldImpl>
    where
        U: Into<Tag>,
    {
        FeatureWithTagLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::from_iter(tags),

            phantom: PhantomData,
        }
    }

    pub fn before_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeScenarioHookLastConfigured<WorldImpl> {
        FeatureWithBeforeScenarioHookLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default().cache(hook),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),
        }
    }

    pub fn after_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterScenarioHookLastConfigured<WorldImpl> {
        FeatureWithAfterScenarioHookLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default().cache(hook),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),
        }
    }

    pub fn before_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeStepHookLastConfigured<WorldImpl> {
        FeatureWithBeforeStepHookLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default().cache(hook),
            after_step_hooks: Hooks::default(),
        }
    }

    pub fn after_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterStepHookLastConfigured<WorldImpl> {
        FeatureWithAfterStepHookLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default().cache(hook),
        }
    }

    pub fn background<FromContext, Background, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        FeatureWithBackgroundLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            background: Some(background.into()),
        }
    }

    pub fn rule<FromContext, Rule, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_rule_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            background: None,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            background: None,

            trials,
        }
    }

    fn to_background_ctx(&self) -> BackgroundContext<WorldImpl> {
        BackgroundContext {
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),
        }
    }

    fn to_rule_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        RuleContext {
            feature_description: self.description.clone(),
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            feature_background: None,
        }
    }

    fn to_scenario_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.description.clone(),
            rule_description: None,
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            feature_background: None,
            rule_background: None,
        }
    }
}

pub struct FeatureWithIgnoredLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> FeatureWithIgnoredLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> FeatureWithTagLastConfigured<WorldImpl>
    where
        U: Into<Tag>,
    {
        FeatureWithTagLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::from_iter(tags),

            phantom: PhantomData,
        }
    }

    pub fn before_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeScenarioHookLastConfigured<WorldImpl> {
        FeatureWithBeforeScenarioHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default().cache(hook),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),
        }
    }

    pub fn after_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterScenarioHookLastConfigured<WorldImpl> {
        FeatureWithAfterScenarioHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default().cache(hook),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),
        }
    }

    pub fn before_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeStepHookLastConfigured<WorldImpl> {
        FeatureWithBeforeStepHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default().cache(hook),
            after_step_hooks: Hooks::default(),
        }
    }

    pub fn after_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterStepHookLastConfigured<WorldImpl> {
        FeatureWithAfterStepHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default().cache(hook),
        }
    }

    pub fn background<FromContext, Background, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        FeatureWithBackgroundLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            background: Some(background.into()),
        }
    }

    pub fn rule<FromContext, Rule, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_rule_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            background: None,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            background: None,

            trials,
        }
    }

    fn to_background_ctx(&self) -> BackgroundContext<WorldImpl> {
        BackgroundContext {
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),
        }
    }

    fn to_rule_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        RuleContext {
            feature_description: self.description.clone(),
            ignored: self.ignored,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            feature_background: None,
        }
    }

    fn to_scenario_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.description.clone(),
            rule_description: None,
            ignored: self.ignored,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            feature_background: None,
            rule_background: None,
        }
    }
}

pub struct FeatureWithTagLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> FeatureWithTagLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn before_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeScenarioHookLastConfigured<WorldImpl> {
        FeatureWithBeforeScenarioHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: Hooks::default().cache(hook),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),
        }
    }

    pub fn after_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterScenarioHookLastConfigured<WorldImpl> {
        FeatureWithAfterScenarioHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default().cache(hook),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),
        }
    }

    pub fn before_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeStepHookLastConfigured<WorldImpl> {
        FeatureWithBeforeStepHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default().cache(hook),
            after_step_hooks: Hooks::default(),
        }
    }

    pub fn after_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterStepHookLastConfigured<WorldImpl> {
        FeatureWithAfterStepHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default().cache(hook),
        }
    }

    pub fn background<FromContext, Background, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        FeatureWithBackgroundLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            background: Some(background.into()),
        }
    }

    pub fn rule<FromContext, Rule, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_rule_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            background: None,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            background: None,

            trials,
        }
    }

    fn to_background_ctx(&self) -> BackgroundContext<WorldImpl> {
        BackgroundContext {
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),
        }
    }

    fn to_rule_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        RuleContext {
            feature_description: self.description.clone(),
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            feature_background: None,
        }
    }

    fn to_scenario_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.description.clone(),
            rule_description: None,
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),

            feature_background: None,
            rule_background: None,
        }
    }
}

pub struct FeatureWithBeforeScenarioHookLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    before_scenario_hooks: Hooks<WorldImpl>,
    after_scenario_hooks: Hooks<WorldImpl>,
    before_step_hooks: Hooks<WorldImpl>,
    after_step_hooks: Hooks<WorldImpl>,
}

impl<WorldImpl> FeatureWithBeforeScenarioHookLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> FeatureWithTaggedHookLastConfigured<WorldImpl>
    where
        U: Into<Tag>,
    {
        FeatureWithTaggedHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_tagged(tags),
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,
        }
    }

    pub fn before_scenario(self, hook: impl HookFn<WorldImpl>) -> Self {
        Self {
            before_scenario_hooks: self.before_scenario_hooks
                .take_cached_as_untagged()
                .then_cache(hook),
            ..self            
        }
    }

    pub fn after_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterScenarioHookLastConfigured<WorldImpl> {
        FeatureWithAfterScenarioHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.cache(hook),
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,
        }
    }

    pub fn before_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeStepHookLastConfigured<WorldImpl> {
        FeatureWithBeforeStepHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks.cache(hook),
            after_step_hooks: self.after_step_hooks,
        }
    }

    pub fn after_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterStepHookLastConfigured<WorldImpl> {
        FeatureWithAfterStepHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks.cache(hook),
        }
    }

    pub fn background<FromContext, Background, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        FeatureWithBackgroundLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: Some(background.into()),
        }
    }

    pub fn rule<FromContext, Rule, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_rule_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: None,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: None,

            trials,
        }
    }

    fn to_background_ctx(&self) -> BackgroundContext<WorldImpl> {
        BackgroundContext {
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),
        }
    }

    fn to_rule_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        RuleContext {
            feature_description: self.description.clone(),
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone().take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.clone().take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),

            feature_background: None,
        }
    }

    fn to_scenario_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.description.clone(),
            rule_description: None,
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone().take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.clone().take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),

            feature_background: None,
            rule_background: None,
        }
    }
}

pub struct FeatureWithAfterScenarioHookLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    before_scenario_hooks: Hooks<WorldImpl>,
    after_scenario_hooks: Hooks<WorldImpl>,
    before_step_hooks: Hooks<WorldImpl>,
    after_step_hooks: Hooks<WorldImpl>,
}

impl<WorldImpl> FeatureWithAfterScenarioHookLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> FeatureWithTaggedHookLastConfigured<WorldImpl>
    where
        U: Into<Tag>,
    {
        FeatureWithTaggedHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_tagged(tags),
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,
        }
    }

    pub fn before_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeScenarioHookLastConfigured<WorldImpl> {
        FeatureWithBeforeScenarioHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.cache(hook),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,
        }
    }

    pub fn after_scenario(self, hook: impl HookFn<WorldImpl>) -> Self {
        Self {
            after_step_hooks: self.after_step_hooks
                .take_cached_as_untagged()
                .then_cache(hook),
            ..self
        }
    }

    pub fn before_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeStepHookLastConfigured<WorldImpl> {
        FeatureWithBeforeStepHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.cache(hook),
            after_step_hooks: self.after_step_hooks,
        }
    }

    pub fn after_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterStepHookLastConfigured<WorldImpl> {
        FeatureWithAfterStepHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks.cache(hook),
        }
    }

    pub fn background<FromContext, Background, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        FeatureWithBackgroundLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: Some(background.into()),
        }
    }

    pub fn rule<FromContext, Rule, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_rule_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: None,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: None,

            trials,
        }
    }

    fn to_background_ctx(&self) -> BackgroundContext<WorldImpl> {
        BackgroundContext {
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),
        }
    }

    fn to_rule_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        RuleContext {
            feature_description: self.description.clone(),
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone().take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.clone().take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),

            feature_background: None,
        }
    }

    fn to_scenario_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.description.clone(),
            rule_description: None,
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone().take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.clone().take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),

            feature_background: None,
            rule_background: None,
        }
    }
}

pub struct FeatureWithBeforeStepHookLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    before_scenario_hooks: Hooks<WorldImpl>,
    after_scenario_hooks: Hooks<WorldImpl>,
    before_step_hooks: Hooks<WorldImpl>,
    after_step_hooks: Hooks<WorldImpl>,
}

impl<WorldImpl> FeatureWithBeforeStepHookLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> FeatureWithTaggedHookLastConfigured<WorldImpl>
    where
        U: Into<Tag>,
    {
        FeatureWithTaggedHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks.take_cached_as_tagged(tags),
            after_step_hooks: self.after_step_hooks,
        }
    }

    pub fn before_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeScenarioHookLastConfigured<WorldImpl> {
        FeatureWithBeforeScenarioHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.cache(hook),
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks,
        }
    }

    pub fn after_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterScenarioHookLastConfigured<WorldImpl> {
        FeatureWithAfterScenarioHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks.cache(hook),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks,
        }
    }

    pub fn before_step(self, hook: impl HookFn<WorldImpl>) -> Self {
        Self {
            before_step_hooks: self.before_step_hooks
                .take_cached_as_untagged()
                .then_cache(hook),
            ..self
        }
    }

    pub fn after_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterStepHookLastConfigured<WorldImpl> {
        FeatureWithAfterStepHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.cache(hook),
        }
    }

    pub fn background<FromContext, Background, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        FeatureWithBackgroundLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: Some(background.into()),
        }
    }

    pub fn rule<FromContext, Rule, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_rule_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: None,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: None,

            trials,
        }
    }

    fn to_background_ctx(&self) -> BackgroundContext<WorldImpl> {
        BackgroundContext {
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),
        }
    }

    fn to_rule_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        RuleContext {
            feature_description: self.description.clone(),
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone().take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.clone().take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),

            feature_background: None,
        }
    }

    fn to_scenario_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.description.clone(),
            rule_description: None,
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone().take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.clone().take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),

            feature_background: None,
            rule_background: None,
        }
    }
}

pub struct FeatureWithAfterStepHookLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    before_scenario_hooks: Hooks<WorldImpl>,
    after_scenario_hooks: Hooks<WorldImpl>,
    before_step_hooks: Hooks<WorldImpl>,
    after_step_hooks: Hooks<WorldImpl>,
}

impl<WorldImpl> FeatureWithAfterStepHookLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> FeatureWithTaggedHookLastConfigured<WorldImpl>
    where
        U: Into<Tag>,
    {
        FeatureWithTaggedHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks.take_cached_as_tagged(tags),
        }
    }

    pub fn before_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeScenarioHookLastConfigured<WorldImpl> {
        FeatureWithBeforeScenarioHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.cache(hook),
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),
        }
    }

    pub fn after_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterScenarioHookLastConfigured<WorldImpl> {
        FeatureWithAfterScenarioHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks.cache(hook),
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),
        }
    }

    pub fn before_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeStepHookLastConfigured<WorldImpl> {
        FeatureWithBeforeStepHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks.cache(hook),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),
        }
    }

    pub fn after_step(self, hook: impl HookFn<WorldImpl>) -> Self {
        Self {
            after_step_hooks: self.after_step_hooks
                .take_cached_as_untagged()
                .then_cache(hook),
            ..self
        }
    }

    pub fn background<FromContext, Background, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        FeatureWithBackgroundLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: Some(background.into()),
        }
    }

    pub fn rule<FromContext, Rule, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_rule_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: None,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: None,

            trials,
        }
    }

    fn to_background_ctx(&self) -> BackgroundContext<WorldImpl> {
        BackgroundContext {
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),
        }
    }

    fn to_rule_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        RuleContext {
            feature_description: self.description.clone(),
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone().take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.clone().take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),

            feature_background: None,
        }
    }

    fn to_scenario_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.description.clone(),
            rule_description: None,
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone().take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.clone().take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),

            feature_background: None,
            rule_background: None,
        }
    }
}

pub struct FeatureWithTaggedHookLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    before_scenario_hooks: Hooks<WorldImpl>,
    after_scenario_hooks: Hooks<WorldImpl>,
    before_step_hooks: Hooks<WorldImpl>,
    after_step_hooks: Hooks<WorldImpl>,
}

impl<WorldImpl> FeatureWithTaggedHookLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn before_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeScenarioHookLastConfigured<WorldImpl> {
        FeatureWithBeforeScenarioHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.cache(hook),
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,
        }
    }

    pub fn after_scenario(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterScenarioHookLastConfigured<WorldImpl> {
        FeatureWithAfterScenarioHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks.cache(hook),
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,
        }
    }

    pub fn before_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithBeforeStepHookLastConfigured<WorldImpl> {
        FeatureWithBeforeStepHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks.cache(hook),
            after_step_hooks: self.after_step_hooks,
        }
    }

    pub fn after_step(self, hook: impl HookFn<WorldImpl>) -> FeatureWithAfterStepHookLastConfigured<WorldImpl> {
        FeatureWithAfterStepHookLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks.cache(hook),
        }
    }

    pub fn background<FromContext, Background, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(BackgroundContext<WorldImpl>) -> Background,
        Background: FinalizableBackground<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_background_ctx();
        let background = from_ctx(ctx);

        FeatureWithBackgroundLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: Some(background.into()),
        }
    }

    pub fn rule<FromContext, Rule, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_rule_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: None,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario, FeatureBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks.take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.take_cached_as_untagged(),

            background: None,

            trials,
        }
    }

    fn to_background_ctx(&self) -> BackgroundContext<WorldImpl> {
        BackgroundContext {
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),
        }
    }

    fn to_rule_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        RuleContext {
            feature_description: self.description.clone(),
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone().take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.clone().take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),

            feature_background: None,
        }
    }

    fn to_scenario_ctx<FeatureBackgroundGivenStepFnImpl>(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>
    where
        FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        ScenarioContext {
            feature_description: self.description.clone(),
            rule_description: None,
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone().take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.clone().take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),

            feature_background: None,
            rule_background: None,
        }
    }
}

pub struct FeatureWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    before_scenario_hooks: Hooks<WorldImpl>,
    after_scenario_hooks: Hooks<WorldImpl>,
    before_step_hooks: Hooks<WorldImpl>,
    after_step_hooks: Hooks<WorldImpl>,

    background: Option<BackgroundPayload<FeatureBackgroundGivenStepFnImpl, WorldImpl>>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl> FeatureWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn rule<FromContext, Rule>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
    {
        let ctx = self.to_rule_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,

            background: self.background,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
    {
        let ctx = self.to_scenario_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,

            background: self.background,

            trials,
        }
    }

    fn to_rule_ctx(&self) -> RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
        RuleContext {
            feature_description: self.description.clone(),
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone(),
            after_scenario_hooks: self.after_scenario_hooks.clone(),
            before_step_hooks: self.before_step_hooks.clone(),
            after_step_hooks: self.after_step_hooks.clone(),

            feature_background: self.background.clone(),
        }
    }

    fn to_scenario_ctx(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl> {
        ScenarioContext {
            feature_description: self.description.clone(),
            rule_description: None,
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone(),
            after_scenario_hooks: self.after_scenario_hooks.clone(),
            before_step_hooks: self.before_step_hooks.clone(),
            after_step_hooks: self.after_step_hooks.clone(),

            feature_background: self.background.clone(),
            rule_background: None,
        }
    }
}

pub struct FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    before_scenario_hooks: Hooks<WorldImpl>,
    after_scenario_hooks: Hooks<WorldImpl>,
    before_step_hooks: Hooks<WorldImpl>,
    after_step_hooks: Hooks<WorldImpl>,

    background: Option<BackgroundPayload<FeatureBackgroundGivenStepFnImpl, WorldImpl>>,

    trials: Vec<libtest::Trial>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    FeatureWithRuleOrScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn rule<FromContext, Rule>(
        self,
        from_ctx: FromContext,
    ) -> Self
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
    {
        let ctx = self.to_rule_ctx();
        let rule = from_ctx(ctx);

        let mut trials = self.trials;
        trials.extend(rule.into());

        Self {
            trials,
            ..self
        }
    }

    pub fn scenario<FromContext, Scenario>(
        self,
        from_ctx: FromContext,
    ) -> Self
    where
        FromContext: FnOnce(ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
    {
        let ctx = self.to_scenario_ctx();
        let scenario: Scenario = from_ctx(ctx);
        
        let mut trials = self.trials;
        trials.push(scenario.into());

        Self {
            trials,
            ..self
        }
    }

    fn to_rule_ctx(&self) -> RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
        RuleContext {
            feature_description: self.description.clone(),
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone().take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.clone().take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),

            feature_background: self.background.clone(),
        }
    }

    fn to_scenario_ctx(&self) -> ScenarioContext<FeatureBackgroundGivenStepFnImpl, NoOpGivenStepFn<WorldImpl>, WorldImpl> {
        ScenarioContext {
            feature_description: self.description.clone(),
            rule_description: None,
            ignored: self.ignored,
            tags: self.tags.clone(),

            before_scenario_hooks: self.before_scenario_hooks.clone().take_cached_as_untagged(),
            after_scenario_hooks: self.after_scenario_hooks.clone().take_cached_as_untagged(),
            before_step_hooks: self.before_step_hooks.clone().take_cached_as_untagged(),
            after_step_hooks: self.after_step_hooks.clone().take_cached_as_untagged(),

            feature_background: self.background.clone(),
            rule_background: None,
        }
    }
}

impl<BackgroundGivenStepFnImpl, WorldImpl>
    From<FeatureWithRuleOrScenarioLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>> for Vec<libtest::Trial>
{
    fn from(feature: FeatureWithRuleOrScenarioLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        feature.trials
    }
}
