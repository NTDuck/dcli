use std::marker::PhantomData;

pub use UnconfiguredFeature as Feature;

use crate::elements::BackgroundContext;
use crate::elements::BackgroundGivenStepFn;
use crate::elements::FinalizableBackground;
use crate::elements::FinalizableRule;
use crate::elements::FinalizableScenario;
use crate::elements::World;
use crate::utils::aliases::MaybeOwnedStr;

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

    pub fn background<BackgroundGivenStepFnImpl>(
        self,
        background: impl FinalizableBackground<BackgroundGivenStepFnImpl, WorldImpl>,
    ) -> FeatureWithBackgroundLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        FeatureWithBackgroundLastConfigured {
            description: self.description,
            ignored: None,

            background: Some(background.into()),
        }
    }

    pub fn rule<FromContext, Rule, BackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        let ctx = self.as_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: None,

            background: None,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario, BackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        let ctx = self.as_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: None,

            background: None,

            trials,
        }
    }

    fn as_ctx<BackgroundGivenStepFnImpl>(&self) -> FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
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
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
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
    ) -> FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        let ctx = self.as_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: None,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario, BackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        let ctx = self.as_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: None,

            trials,
        }
    }

    fn as_ctx<BackgroundGivenStepFnImpl>(&self) -> FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        FeatureContext {
            description: self.description.clone(),
            ignored: self.ignored,

            background: None,
        }
    }
}

pub struct FeatureWithBackgroundLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    background: Option<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> FeatureWithBackgroundLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn rule<FromContext, Rule>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
    {
        let ctx = self.as_ctx();
        let rule = from_ctx(ctx);
        let trials = rule.into();

        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: self.background,

            trials,
        }
    }

    pub fn scenario<FromContext, Scenario>(
        self,
        from_ctx: FromContext,
    ) -> FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
    {
        let ctx = self.as_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: self.background,

            trials,
        }
    }

    fn as_ctx(&self) -> FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        FeatureContext {
            description: self.description.clone(),
            ignored: self.ignored,

            background: self.background.clone(),
        }
    }
}

pub struct FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    background: Option<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>,

    trials: Vec<libtest::Trial>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl>
    FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn rule<FromContext, Rule>(self, from_ctx: FromContext) -> Self
    where
        FromContext: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Rule,
        Rule: FinalizableRule,
    {
        let ctx = self.as_ctx();
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
        let ctx = self.as_ctx();
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

    fn as_ctx(&self) -> FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>
    where
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        FeatureContext {
            description: self.description.clone(),
            ignored: self.ignored,

            background: self.background.clone(),
        }
    }
}

impl<BackgroundGivenStepFnImpl, WorldImpl>
    From<FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>> for Vec<libtest::Trial>
{
    fn from(feature: FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        feature.trials
    }
}

#[derive(Default)]
pub struct FeatureContext<BackgroundGivenStepFnImpl, WorldImpl> {
    pub(super) description: Option<MaybeOwnedStr>,
    pub(super) ignored: Option<bool>,

    pub(super) background: Option<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> Clone for FeatureContext<BackgroundGivenStepFnImpl, WorldImpl> {
    fn clone(&self) -> Self {
        Self {
            description: self.description.clone(),
            ignored: self.ignored,

            background: self.background.clone(),
        }
    }
}
