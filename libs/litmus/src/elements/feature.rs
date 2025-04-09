use std::marker::PhantomData;

use crate::elements::step::World;
use crate::elements::step::GivenStepFn;
use crate::elements::step::ThenStepFn;
use crate::elements::step::WhenStepFn;
use crate::elements::step::VecExt;
use crate::elements::background::FinalizedBackground;
use crate::elements::rule::FinalizedRule;
use crate::elements::scenario::FinalizedScenario;
use crate::utils::aliases::MaybeOwnedStr;

pub use UnconfiguredFeature as Feature;

pub struct UnconfiguredFeature<WorldImpl> {
    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> UnconfiguredFeature<WorldImpl>
where
    WorldImpl: World,
{
    pub fn named(description: impl Into<MaybeOwnedStr>) -> FeatureWithDescriptionLastConfigured<WorldImpl> {
        FeatureWithDescriptionLastConfigured {
            description: description.into(),

            phantom: PhantomData,
        }
    }
}

pub struct FeatureWithDescriptionLastConfigured<WorldImpl> {
    description: MaybeOwnedStr,

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

    pub fn with_background<GivenStepFnImpl>(self, background: impl Into<FinalizedBackground<GivenStepFnImpl, WorldImpl>>) -> FeatureWithBackgroundLastConfigured<GivenStepFnImpl, WorldImpl>
    where
        GivenStepFnImpl: GivenStepFn<WorldImpl>,
    {
        FeatureWithBackgroundLastConfigured {
            description: self.description,
            ignored: None,

            background: Some(background.into()),
        }
    }

    pub fn with_rule<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl>(self, rule: impl Into<FinalizedRule<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>) -> FeatureWithRulesOrScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
    where
        GivenStepFnImpl: GivenStepFn<WorldImpl>,
        WhenStepFnImpl: WhenStepFn<WorldImpl>,
        ThenStepFnImpl: ThenStepFn<WorldImpl>,
    {
        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: None,

            background: None,
            rules: vec![rule.into()],
            scenarios: vec![],
        }
    }

    pub fn with_scenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl>(self, scenario: impl Into<FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>) -> FeatureWithRulesOrScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
    where
        GivenStepFnImpl: GivenStepFn<WorldImpl>,
        WhenStepFnImpl: WhenStepFn<WorldImpl>,
        ThenStepFnImpl: ThenStepFn<WorldImpl>,
    {
        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: None,

            background: None,
            rules: vec![],
            scenarios: vec![scenario.into()],
        }
    }
}

pub struct FeatureWithIgnoredLastConfigured<WorldImpl> {
    description: MaybeOwnedStr,
    ignored: Option<bool>,

    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> FeatureWithIgnoredLastConfigured<WorldImpl> 
where
    WorldImpl: World,
{
    pub fn with_background<GivenStepFnImpl>(self, background: impl Into<FinalizedBackground<GivenStepFnImpl, WorldImpl>>) -> FeatureWithBackgroundLastConfigured<GivenStepFnImpl, WorldImpl>
    where
        GivenStepFnImpl: GivenStepFn<WorldImpl>,
    {
        FeatureWithBackgroundLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: Some(background.into()),
        }
    }

    pub fn with_rule<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl>(self, rule: impl Into<FinalizedRule<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>) -> FeatureWithRulesOrScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
    where
        GivenStepFnImpl: GivenStepFn<WorldImpl>,
        WhenStepFnImpl: WhenStepFn<WorldImpl>,
        ThenStepFnImpl: ThenStepFn<WorldImpl>,
    {
        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: None,
            rules: vec![rule.into()],
            scenarios: vec![],
        }
    }

    pub fn with_scenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl>(self, scenario: impl Into<FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>) -> FeatureWithRulesOrScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
    where
        GivenStepFnImpl: GivenStepFn<WorldImpl>,
        WhenStepFnImpl: WhenStepFn<WorldImpl>,
        ThenStepFnImpl: ThenStepFn<WorldImpl>,
    {
        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: None,
            rules: vec![],
            scenarios: vec![scenario.into()],
        }
    }
}

pub struct FeatureWithBackgroundLastConfigured<GivenStepFnImpl, WorldImpl> {
    description: MaybeOwnedStr,
    ignored: Option<bool>,

    background: Option<FinalizedBackground<GivenStepFnImpl, WorldImpl>>,
}

impl<GivenStepFnImpl, WorldImpl> FeatureWithBackgroundLastConfigured<GivenStepFnImpl, WorldImpl>
where
    GivenStepFnImpl: GivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn with_rule<WhenStepFnImpl, ThenStepFnImpl>(self, rule: impl Into<FinalizedRule<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>) -> FeatureWithRulesOrScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
    where
        GivenStepFnImpl: GivenStepFn<WorldImpl>,
        WhenStepFnImpl: WhenStepFn<WorldImpl>,
        ThenStepFnImpl: ThenStepFn<WorldImpl>,
    {
        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: self.background,
            rules: vec![rule.into()],
            scenarios: vec![],
        }
    }

    pub fn with_scenario<WhenStepFnImpl, ThenStepFnImpl>(self, scenario: impl Into<FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>) -> FeatureWithRulesOrScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
    where
        GivenStepFnImpl: GivenStepFn<WorldImpl>,
        WhenStepFnImpl: WhenStepFn<WorldImpl>,
        ThenStepFnImpl: ThenStepFn<WorldImpl>,
    {
        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: self.background,
            rules: vec![],
            scenarios: vec![scenario.into()],
        }
    }
}

pub struct FeatureWithRulesOrScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> {
    description: MaybeOwnedStr,
    ignored: Option<bool>,

    background: Option<FinalizedBackground<GivenStepFnImpl, WorldImpl>>,
    rules: Vec<FinalizedRule<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>,
    scenarios: Vec<FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>,
}

impl<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> FeatureWithRulesOrScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
where
    GivenStepFnImpl: GivenStepFn<WorldImpl>,
    WhenStepFnImpl: WhenStepFn<WorldImpl>,
    ThenStepFnImpl: ThenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn with_rule(self, rule: impl Into<FinalizedRule<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>) -> Self {
        Self {
            rules: self.rules.with(rule.into()),
            ..self
        }
    }

    pub fn with_scenario(self, scenario: impl Into<FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>) -> Self {
        Self {
            scenarios: self.scenarios.with(scenario.into()),
            ..self
        }
    }
}

impl<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> From<FeatureWithRulesOrScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>> for Vec<libtest::Trial>
where
    GivenStepFnImpl: GivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(feature: FeatureWithRulesOrScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>) -> Self {
        let FeatureWithRulesOrScenariosLastConfigured {
            description,
            ignored,
            
            background,
            rules,
            scenarios,
        } = feature;

        // let capacity = rules.len() + scenarios.len();
        // let trials = Vec::with_capacity(capacity);

        todo!()
    }
}
