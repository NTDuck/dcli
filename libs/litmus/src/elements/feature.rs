use std::marker::PhantomData;

pub use UnconfiguredFeature as Feature;

use crate::elements::background::BackgroundContext;
use crate::elements::step::BackgroundGivenStepFn;
use crate::elements::step::World;
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
        background: impl Into<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>,
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

    pub fn rule<F, R, BackgroundGivenStepFnImpl>(
        self,
        f: F,
    ) -> FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        F: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> R,
        R: Into<Vec<libtest::Trial>>,
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        let context = self.as_context();
        let trials = f(context).into();

        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: None,

            background: None,
            trials,
        }
    }

    pub fn scenario<F, R, BackgroundGivenStepFnImpl>(
        self,
        f: F,
    ) -> FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        F: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> R,
        R: Into<libtest::Trial>,
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        let context = self.as_context();
        let trial = f(context).into();

        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: None,

            background: None,
            trials: vec![trial],
        }
    }

    fn as_context<BackgroundGivenStepFnImpl>(&self) -> FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>
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
        background: impl Into<BackgroundContext<BackgroundGivenStepFnImpl, WorldImpl>>,
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

    pub fn rule<F, R, BackgroundGivenStepFnImpl>(
        self,
        f: F,
    ) -> FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        F: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> R,
        R: Into<Vec<libtest::Trial>>,
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        let context = self.as_context();
        let trials = f(context).into();

        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: None,
            trials,
        }
    }

    pub fn scenario<F, R, BackgroundGivenStepFnImpl>(
        self,
        f: F,
    ) -> FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        F: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> R,
        R: Into<libtest::Trial>,
        BackgroundGivenStepFnImpl: BackgroundGivenStepFn<WorldImpl>,
    {
        let context = self.as_context();
        let trial = f(context).into();

        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: None,
            trials: vec![trial],
        }
    }

    fn as_context<BackgroundGivenStepFnImpl>(&self) -> FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>
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
    pub fn rule<F, R>(self, f: F) -> FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        F: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> R,
        R: Into<Vec<libtest::Trial>>,
    {
        let context = self.as_context();
        let trials = f(context).into();

        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: None,
            trials,
        }
    }

    pub fn scenario<F, R>(self, f: F) -> FeatureWithRulesOrScenariosLastConfigured<BackgroundGivenStepFnImpl, WorldImpl>
    where
        F: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> R,
        R: Into<libtest::Trial>,
    {
        let context = self.as_context();
        let trial = f(context).into();

        FeatureWithRulesOrScenariosLastConfigured {
            description: self.description,
            ignored: self.ignored,

            background: None,
            trials: vec![trial],
        }
    }

    fn as_context(&self) -> FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>
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
    pub fn rule<F, R>(self, f: F) -> Self
    where
        F: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> R,
        R: Into<Vec<libtest::Trial>>,
    {
        let context = self.as_context();
        let mut trials = f(context).into();

        let mut this_trials = self.trials;
        this_trials.append(&mut trials);

        Self {
            description: self.description,
            ignored: self.ignored,

            background: None,
            trials: this_trials,
        }
    }

    pub fn scenario<F, R>(self, f: F) -> Self
    where
        F: FnOnce(FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>) -> R,
        R: Into<libtest::Trial>,
    {
        let context = self.as_context();
        let trial = f(context).into();

        let mut this_trials = self.trials;
        this_trials.push(trial);

        Self {
            description: self.description,
            ignored: self.ignored,

            background: None,
            trials: this_trials,
        }
    }

    fn as_context(&self) -> FeatureContext<BackgroundGivenStepFnImpl, WorldImpl>
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
