use std::marker::PhantomData;

use crate::elements::step::World;
use crate::elements::step::BackgroundGivenStepFn;
use crate::elements::background::BackgroundContext;
use crate::utils::aliases::MaybeOwnedStr;

pub use UnconfiguredRule as Rule;

use super::FeatureContext;

pub struct UnconfiguredRule<BackgroundGivenStepFnImpl, WorldImpl> {
    feature: FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> From<FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>> for UnconfiguredRule<BackgroundGivenStepFnImpl, WorldImpl> {
    fn from(feature: FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        Self {
            feature,
        }
    }
}

impl<BackgroundGivenStepFnImpl, WorldImpl> UnconfiguredRule<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn named(self, description: impl Into<MaybeOwnedStr>) -> RuleWithDescriptionLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
        RuleWithDescriptionLastConfigured {
            description: Some(description.into()),

            feature: self.feature,
        }
    }
}

pub struct RuleWithDescriptionLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    
    feature: FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> RuleWithDescriptionLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn ignored(self, ignored: impl Into<bool>) -> RuleWithIgnoredLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
        RuleWithIgnoredLastConfigured {
            description: self.description,
            ignored: Some(ignored.into()),

            feature: self.feature,
        }
    }

    pub fn background(self, background: impl Into<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>) -> RuleWithBackgroundLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
        RuleWithBackgroundLastConfigured {
            description: self.description,
            ignored: None,

            feature: self.feature,
            background: Some(background.into()),
        }
    }

    pub fn scenario<F, R>(self, f: F) -> RuleWithScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        F: FnOnce(RuleContext<BackgroundGivenStepFnImpl, WorldImpl>) -> R,
        R: Into<libtest::Trial>,
    {
        let context = self.as_context();
        let trial = f(context).into();

        RuleWithScenariosLastConfigured {
            description: self.description,
            ignored: None,

            feature: self.feature,
            background: None,

            trials: vec![trial],
        }
    }

    fn as_context(&self) -> RuleContext<BackgroundGivenStepFnImpl, WorldImpl> {
        RuleContext {
            description: self.description.clone(),
            ignored: None,

            feature: self.feature.clone(),
            background: None,
        }
    }
}

pub struct RuleWithIgnoredLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    feature: FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> RuleWithIgnoredLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> 
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn background(self, background: impl Into<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>) -> RuleWithBackgroundLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
        RuleWithBackgroundLastConfigured {
            description: self.description,
            ignored: None,

            feature: self.feature,
            background: Some(background.into()),
        }
    }

    pub fn scenario<F, R>(self, f: F) -> RuleWithScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        F: FnOnce(RuleContext<BackgroundGivenStepFnImpl, WorldImpl>) -> R,
        R: Into<libtest::Trial>,
    {
        let context = self.as_context();
        let trial = f(context).into();

        RuleWithScenariosLastConfigured {
            description: self.description,
            ignored: None,

            feature: self.feature,
            background: None,

            trials: vec![trial],
        }
    }

    fn as_context(&self) -> RuleContext<BackgroundGivenStepFnImpl, WorldImpl> {
        RuleContext {
            description: self.description.clone(),
            ignored: self.ignored.clone(),

            feature: self.feature.clone(),
            background: None,
        }
    }
}

pub struct RuleWithBackgroundLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    feature: FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>,
    background: Option<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> RuleWithBackgroundLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn scenario<F, R>(self, f: F) -> RuleWithScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        F: FnOnce(RuleContext<BackgroundGivenStepFnImpl, WorldImpl>) -> R,
        R: Into<libtest::Trial>,
    {
        let context = self.as_context();
        let trial = f(context).into();

        RuleWithScenariosLastConfigured {
            description: self.description,
            ignored: None,

            feature: self.feature,
            background: None,

            trials: vec![trial],
        }
    }

    fn as_context(&self) -> RuleContext<BackgroundGivenStepFnImpl, WorldImpl> {
        RuleContext {
            description: self.description.clone(),
            ignored: self.ignored.clone(),

            feature: self.feature.clone(),
            background: self.background.clone(),
        }
    }
}

pub struct RuleWithScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    feature: FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>,
    background: Option<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>,

    trials: Vec<libtest::Trial>,
}

impl<BackgroundGivenStepFnImpl, WorldImpl> RuleWithScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
where
    BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn scenario<F, R>(self, f: F) -> Self
    where
        F: FnOnce(RuleContext<BackgroundGivenStepFnImpl, WorldImpl>) -> R,
        R: Into<libtest::Trial>,
    {
        let context = self.as_context();
        let trial = f(context).into();

        let mut this_trials = self.trials;
        this_trials.push(trial);

        Self {
            description: self.description,
            ignored: self.ignored,

            feature: self.feature,
            background: self.background,
            trials: this_trials,
        }
    }

    fn as_context(&self) -> RuleContext<BackgroundGivenStepFnImpl, WorldImpl> {
        RuleContext {
            description: self.description.clone(),
            ignored: self.ignored.clone(),

            feature: self.feature.clone(),
            background: self.background.clone(),
        }
    }
}

impl<BackgroundGivenStepFnImpl, WorldImpl> From<RuleWithScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>> for Vec<libtest::Trial> {
    fn from(rule: RuleWithScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>) -> Self {
        rule.trials
    }
}

#[derive(Clone, Default)]
pub struct RuleContext<BackgroundGivenStepFnImpl, WorldImpl> {
    pub(super) description: Option<MaybeOwnedStr>,
    pub(super) ignored: Option<bool>,

    pub(super) feature: FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>,
    pub(super) background: Option<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>,
}

// impl<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> From<RuleWithScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>> for FinalizedRule<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
// where
//     GivenStepFnImpl: ScenarioGivenStepFn<WorldImpl>,
//     WorldImpl: World,
// {
//     fn from(rule: RuleWithScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>) -> Self {
//         let RuleWithScenariosLastConfigured {
//             description,
//             ignored,

//             background,
//             scenarios,
//             ..
//         } = rule;

//         Self {
//             description,
//             ignored: match ignored {
//                 Some(ignored) => ignored,
//                 None => false,
//             },

//             background,
//             scenarios,

//             phantom: PhantomData,
//         }
//     }
// }

// pub struct FinalizedRule<BackgroundGivenStepFnImpl, WorldImpl> {
//     pub(super) description: MaybeOwnedStr,
//     pub(super) ignored: bool,

//     pub(super) background: Option<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>,
//     pub(super) trials: Vec<libtest::Trial>,
// }
