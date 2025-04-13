pub use UnconfiguredRule as Rule;

use crate::elements::FeatureContext;
use crate::elements::BackgroundContext;
use crate::elements::BackgroundGivenStepFn;
use crate::elements::World;
use crate::utils::aliases::MaybeOwnedStr;

use crate::elements::FinalizableBackground;
use crate::elements::FinalizableScenario;

pub struct UnconfiguredRule<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    feature: FeatureContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl> From<FeatureContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>>
    for UnconfiguredRule<FeatureBackgroundGivenStepFnImpl, WorldImpl>
{
    fn from(feature: FeatureContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        Self {
            feature,
        }
    }
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl> UnconfiguredRule<FeatureBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn named(
        self,
        description: impl Into<MaybeOwnedStr>,
    ) -> RuleWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
        RuleWithDescriptionLastConfigured {
            description: Some(description.into()),

            feature: self.feature,
        }
    }
}

pub struct RuleWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,

    feature: FeatureContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithDescriptionLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
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

    pub fn background<RuleBackgroundGivenStepFnImpl>(
        self,
        background: impl FinalizableBackground<RuleBackgroundGivenStepFnImpl, WorldImpl>,
    ) -> RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        RuleWithBackgroundLastConfigured {
            description: self.description,
            ignored: None,

            feature: self.feature,
            background: Some(background.into()),
        }
    }

    pub fn scenario<FromContext, Scenario, RuleBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenariosLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        let ctx = self.as_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        RuleWithScenariosLastConfigured {
            description: self.description,
            ignored: None,

            feature: self.feature,
            background: None,

            trials,
        }
    }

    fn as_ctx<RuleBackgroundGivenStepFnImpl>(
        &self,
    ) -> RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        RuleContext {
            description: self.description.clone(),
            ignored: None,

            feature: self.feature.clone(),
            background: None,
        }
    }
}

pub struct RuleWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    feature: FeatureContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
}

impl<FeatureBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithIgnoredLastConfigured<FeatureBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn background<RuleBackgroundGivenStepFnImpl>(
        self,
        background: impl FinalizableBackground<RuleBackgroundGivenStepFnImpl, WorldImpl>,
    ) -> RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        RuleWithBackgroundLastConfigured {
            description: self.description,
            ignored: self.ignored,

            feature: self.feature,
            background: Some(background.into()),
        }
    }

    pub fn scenario<FromContext, Scenario, RuleBackgroundGivenStepFnImpl>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenariosLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
        RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        let ctx = self.as_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        RuleWithScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            feature: self.feature,
            background: None,

            trials,
        }
    }

    fn as_ctx<RuleBackgroundGivenStepFnImpl>(
        &self,
    ) -> RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        RuleContext {
            description: self.description.clone(),
            ignored: self.ignored,

            feature: self.feature.clone(),
            background: None,
        }
    }
}

pub struct RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    feature: FeatureContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
    background: Option<BackgroundContext<RuleBackgroundGivenStepFnImpl, WorldImpl>>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithBackgroundLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn scenario<FromContext, Scenario>(
        self,
        from_ctx: FromContext,
    ) -> RuleWithScenariosLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
    {
        let ctx = self.as_ctx();
        let scenario = from_ctx(ctx);
        let trials = vec![scenario.into()];

        RuleWithScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            feature: self.feature,
            background: self.background,

            trials,
        }
    }

    fn as_ctx(&self) -> RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    where
        RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        RuleContext {
            description: self.description.clone(),
            ignored: self.ignored,

            feature: self.feature.clone(),
            background: self.background.clone(),
        }
    }
}

pub struct RuleWithScenariosLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    feature: FeatureContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
    background: Option<BackgroundContext<RuleBackgroundGivenStepFnImpl, WorldImpl>>,

    trials: Vec<libtest::Trial>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    RuleWithScenariosLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn scenario<FromContext, Scenario>(self, from_ctx: FromContext) -> Self
    where
        FromContext: FnOnce(RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>) -> Scenario,
        Scenario: FinalizableScenario,
    {
        let ctx = self.as_ctx();
        let scenario = from_ctx(ctx);

        let mut trials = self.trials;
        trials.push(scenario.into());

        RuleWithScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            feature: self.feature,
            background: self.background,

            trials,
        }
    }

    fn as_ctx(&self) -> RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
        RuleContext {
            description: self.description.clone(),
            ignored: self.ignored,

            feature: self.feature.clone(),
            background: self.background.clone(),
        }
    }
}

pub trait FinalizableRule: Into<Vec<libtest::Trial>> {}

impl<T> FinalizableRule for T
where 
    T: Into<Vec<libtest::Trial>>,
{
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
    From<RuleWithScenariosLastConfigured<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>>
    for Vec<libtest::Trial>
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(
        rule: RuleWithScenariosLastConfigured<
            FeatureBackgroundGivenStepFnImpl,
            RuleBackgroundGivenStepFnImpl,
            WorldImpl,
        >,
    ) -> Self {
        rule.trials
    }
}

#[derive(Default)]
pub struct RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> {
    pub(super) description: Option<MaybeOwnedStr>,
    pub(super) ignored: Option<bool>,

    pub(super) feature: FeatureContext<FeatureBackgroundGivenStepFnImpl, WorldImpl>,
    pub(super) background: Option<BackgroundContext<RuleBackgroundGivenStepFnImpl, WorldImpl>>,
}

impl<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl> Clone
    for RuleContext<FeatureBackgroundGivenStepFnImpl, RuleBackgroundGivenStepFnImpl, WorldImpl>
where
    FeatureBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    RuleBackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn clone(&self) -> Self {
        Self {
            description: self.description.clone(),
            ignored: self.ignored,

            feature: self.feature.clone(),
            background: self.background.clone(),
        }
    }
}
