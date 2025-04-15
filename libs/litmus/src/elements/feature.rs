use std::marker::PhantomData;

pub use UnconfiguredFeature as Feature;

use crate::elements::BackgroundContext;
use crate::elements::ReusableGivenStepFn;
use crate::elements::FinalizableBackground;
use crate::elements::FinalizableRule;
use crate::elements::FinalizableScenario;
use crate::elements::World;
use crate::utils::aliases::MaybeOwnedStr;

use super::HookFn;
use super::Hooks;
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

    pub fn before_scenario(self, hooks: impl Into<Hooks<WorldImpl>>) -> FeatureWithUntaggedHookLastConfigured<WorldImpl> {
        FeatureWithUntaggedHookLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: hooks.into(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),
        }
    }

    pub fn after_scenario(self, hooks: impl Into<Hooks<WorldImpl>>) -> FeatureWithUntaggedHookLastConfigured<WorldImpl> {
        FeatureWithUntaggedHookLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: hooks.into(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: Hooks::default(),
        }
    }

    pub fn before_step(self, hooks: impl Into<Hooks<WorldImpl>>) -> FeatureWithUntaggedHookLastConfigured<WorldImpl> {
        FeatureWithUntaggedHookLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: hooks.into(),
        }
    }

    pub fn after_step(self, hooks: impl Into<Hooks<WorldImpl>>) -> FeatureWithUntaggedHookLastConfigured<WorldImpl> {
        FeatureWithUntaggedHookLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            before_scenario_hooks: Hooks::default(),
            after_scenario_hooks: Hooks::default(),
            before_step_hooks: Hooks::default(),
            after_step_hooks: hooks.into(),
        }
    }

    pub fn background<BackgroundGivenStepFnImpl>(
        self,
        background: impl FinalizableBackground<BackgroundGivenStepFnImpl, WorldImpl>,
    ) -> FeatureWithBackgroundLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
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

    pub fn rule<FromContext, Rule, BackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
        BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: None,

            background: None,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario, BackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: None,

            background: None,

            trials,
        }
    }

    fn to_ctx<BackgroundGivenStepFnImpl>(&self) -> FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        FeatureContext {
            description: self.description.clone(),
            ignored: None,

            background: None,
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
    pub fn background<BackgroundGivenStepFnImpl>(
        self,
        background: impl FinalizableBackground<BackgroundGivenStepFnImpl, WorldImpl>,
    ) -> FeatureWithBackgroundLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        FeatureWithBackgroundLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: Some(background.into()),
        }
    }

    pub fn rule<FromContext, Rule, BackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
        BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: None,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario, BackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: None,

            trials,
        }
    }

    fn to_ctx<BackgroundGivenStepFnImpl>(&self) -> FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        FeatureContext {
            description: self.description.clone(),
            ignored: self.ignored,

            background: None,
        }
    }
}

pub struct FeatureWithTagLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    phantom: PhantomData<WorldImpl>,
}

pub struct FeatureWithUntaggedHookLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    before_scenario_hooks: Hooks<WorldImpl>,
    after_scenario_hooks: Hooks<WorldImpl>,
    before_step_hooks: Hooks<WorldImpl>,
    after_step_hooks: Hooks<WorldImpl>,
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

pub struct FeatureWithBackgroundLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    before_scenario_hooks: Hooks<WorldImpl>,
    after_scenario_hooks: Hooks<WorldImpl>,
    before_step_hooks: Hooks<WorldImpl>,
    after_step_hooks: Hooks<WorldImpl>,

    background: Option<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> FeatureWithBackgroundLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn rule<FromContext, Rule>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
    {
        let ctx = self.to_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: self.background,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRuleOrScenarioLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
    {
        let ctx = self.to_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRuleOrScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: self.background,

            trials,
        }
    }

    fn to_ctx(&self) -> FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        FeatureContext {
            description: self.description.clone(),
            ignored: self.ignored,

            background: self.background.clone(),
        }
    }
}

pub struct FeatureWithRuleOrScenarioLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    before_scenario_hooks: Hooks<WorldImpl>,
    after_scenario_hooks: Hooks<WorldImpl>,
    before_step_hooks: Hooks<WorldImpl>,
    after_step_hooks: Hooks<WorldImpl>,

    background: Option<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>,

    trials: Vec<libtest::Trial>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl>
    FeatureWithRuleOrScenarioLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn rule<FromContext, Rule>(self, from_ctx: FromContext) -> Self
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
    {
        let ctx = self.to_ctx();
        let rule = from_ctx(ctx);

        let mut trials = self.trials;
        trials.extend(rule.into());

        Self {
            description: self.description,
            ignored: self.ignored,

            background: self.background,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario>(self, from_ctx: FromContext) -> Self
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
    {
        let ctx = self.to_ctx();
        let scenario = from_ctx(ctx);

        let mut trials = self.trials;
        trials.push(scenario.into());

        Self {
            description: self.description,
            ignored: self.ignored,

            background: self.background,

            trials,
        }
    }

    fn to_ctx(&self) -> FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        FeatureContext {
            description: self.description.clone(),
            ignored: self.ignored,

            background: self.background.clone(),
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

#[derive(Default, Clone)]
pub struct FeatureContext<BackgroundGivenStepFnImpl, WorldImpl> {
    pub(super) description: Option<MaybeOwnedStr>,
    pub(super) ignored: Option<bool>,
    pub(super) tags: Tags,

    pub(super) before_scenario_hooks: Hooks<WorldImpl>,
    pub(super) after_scenario_hooks: Hooks<WorldImpl>,
    pub(super) before_step_hooks: Hooks<WorldImpl>,
    pub(super) after_step_hooks: Hooks<WorldImpl>,

    pub(super) background: Option<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>,
}
