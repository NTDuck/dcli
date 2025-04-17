pub use UnconfiguredRule as Rule;

use crate::elements::BackgroundPayload;
use crate::elements::ReusableGivenStepFn;
use crate::elements::FinalizableBackground;
use crate::elements::FinalizableScenario;
use crate::elements::World;
use crate::utils::aliases::MaybeOwnedStr;

use super::Hooks;
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
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn named(
        self,
        description: impl Into<MaybeOwnedStr>,
    ) -> RuleWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
        RuleWithDescriptionLastConfigured {
            description: Some(description.into()),

            feature: self.ctx,
        }
    }
}

pub struct RuleWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,

    feature: RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn ignored(
        self,
        ignored: impl Into<bool>,
    ) -> RuleWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
        RuleWithIgnoredLastConfigured {
            description: self.description,
            ignored: Some(ignored.into()),

            feature: self.feature,
        }
    }

    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> RuleWithTagLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        U: Into<Tag>,
    {
        RuleWithTagLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::from_iter(tags),

            feature: self.feature,
        }
    }

    pub fn background<RuleBackgroundGivenStepFnImpl>(
        self,
        background: impl FinalizableBackground<RuleBackgroundGivenStepFnImpl, WorldImpl>,
    ) -> RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        RuleWithBackgroundLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            feature: self.feature,
            background: Some(background.into()),
        }
    }

    pub fn scenario<FromContext, Scenario, RuleBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(FeatureAndRuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        RuleWithScenarioLastConfigured {
            description: self.description,
            ignored: None,
            tags: Tags::default(),

            feature: self.feature,
            background: None,

            trials,
        }
    }

    fn to_ctx<RuleBackgroundGivenStepFnImpl>(
        &self,
    ) -> FeatureAndRuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        (self.feature.clone(), RuleContext {
            description: self.description.clone(),
            ignored: None,
            tags: Tags::default(),

            background: None,
        })
    }
}

pub struct RuleWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    feature: RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> RuleWithTagLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    where
        U: Into<Tag>,
    {
        RuleWithTagLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::from_iter(tags),

            feature: self.feature,
        }
    }

    pub fn background<RuleBackgroundGivenStepFnImpl>(
        self,
        background: impl FinalizableBackground<RuleBackgroundGivenStepFnImpl, WorldImpl>,
    ) -> RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        RuleWithBackgroundLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::default(),

            feature: self.feature,
            background: Some(background.into()),
        }
    }

    pub fn scenario<FromContext, Scenario, RuleBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(FeatureAndRuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        RuleWithScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: Tags::default(),

            feature: self.feature,
            background: None,

            trials,
        }
    }

    fn to_ctx<RuleBackgroundGivenStepFnImpl>(
        &self,
    ) -> FeatureAndRuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        (self.feature.clone(), RuleContext {
            description: self.description.clone(),
            ignored: self.ignored,
            tags: Tags::default(),

            background: None,
        })
    }
}

pub struct RuleWithTagLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    feature: RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithTagLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn background<RuleBackgroundGivenStepFnImpl>(
        self,
        background: impl FinalizableBackground<RuleBackgroundGivenStepFnImpl, WorldImpl>,
    ) -> RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        RuleWithBackgroundLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            feature: self.feature,
            background: Some(background.into()),
        }
    }

    pub fn scenario<FromContext, Scenario, RuleBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(FeatureAndRuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        let ctx = self.to_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        RuleWithScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            feature: self.feature,
            background: None,

            trials,
        }
    }

    fn to_ctx<RuleBackgroundGivenStepFnImpl>(
        &self,
    ) -> FeatureAndRuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    {
        (self.feature.clone(), RuleContext {
            description: self.description.clone(),
            ignored: self.ignored,
            tags: Tags::default(),

            background: None,
        })
    }
}

pub struct RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
{
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    feature: RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
    background: Option<BackgroundPayload<RuleBackgroundGivenStepFnImpl, WorldImpl>>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn scenario<FromContext, Scenario>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext:
            FnOnce(FeatureAndRuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
    {
        let ctx = self.to_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        RuleWithScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            feature: self.feature,
            background: self.background,

            trials,
        }
    }

    fn to_ctx(&self) -> FeatureAndRuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
        (self.feature.clone(), RuleContext {
            description: self.description.clone(),
            ignored: self.ignored,
            tags: self.tags.clone(),

            background: self.background.clone(),
        })
    }
}

pub struct RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Tags,

    feature: RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
    background: Option<BackgroundPayload<RuleBackgroundGivenStepFnImpl, WorldImpl>>,

    trials: Vec<libtest::Trial>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn scenario<FromContext, Scenario>(self, from_ctx: FromContext) -> Self
    where
        FromContext:
            FnOnce(FeatureAndRuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
    {
        let ctx = self.to_ctx();
        let scenario = from_ctx(ctx);

        let mut trials = self.trials;
        trials.push(scenario.into());

        RuleWithScenarioLastConfigured {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            feature: self.feature,
            background: self.background,

            trials,
        }
    }

    fn to_ctx(&self) -> FeatureAndRuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
        (self.feature.clone(), RuleContext {
            description: self.description.clone(),
            ignored: self.ignored,
            tags: self.tags.clone(),

            background: self.background.clone(),
        })
    }
}

pub trait FinalizableRule: Into<Vec<libtest::Trial>> {}

impl<T> FinalizableRule for T where T: Into<Vec<libtest::Trial>> {}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    From<RuleWithScenarioLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>>
    for Vec<libtest::Trial>
where
    FeatureBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
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

pub type FeatureAndRuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> = (RuleContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>, RuleContext<RuleBackgroundGivenStepFnImpl, WorldImpl>);

#[derive(Default)]
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
    BackgroundGivenStepFnImpl: ReusableGivenStepFn<WorldImpl>,
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
