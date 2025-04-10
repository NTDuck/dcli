// use std::marker::PhantomData;

// use crate::elements::step::World;
// use crate::elements::step::GivenStepFn;
// use crate::elements::step::ThenStepFn;
// use crate::elements::step::WhenStepFn;
// use crate::elements::step::VecExt;
// use crate::elements::background::FinalizedBackground;
// use crate::elements::scenario::FinalizedScenario;
// use crate::utils::aliases::MaybeOwnedStr;

// pub use UnconfiguredRule as Rule;

// pub struct UnconfiguredRule<WorldImpl> {
//     phantom: PhantomData<WorldImpl>,
// }

// impl<WorldImpl> UnconfiguredRule<WorldImpl>
// where
//     WorldImpl: World,
// {
//     pub fn named(description: impl Into<MaybeOwnedStr>) -> RuleWithDescriptionLastConfigured<WorldImpl> {
//         RuleWithDescriptionLastConfigured {
//             description: description.into(),

//             phantom: PhantomData,
//         }
//     }
// }

// pub struct RuleWithDescriptionLastConfigured<WorldImpl> {
//     description: MaybeOwnedStr,

//     phantom: PhantomData<WorldImpl>,
// }

// impl<WorldImpl> RuleWithDescriptionLastConfigured<WorldImpl>
// where
//     WorldImpl: World,
// {
//     pub fn ignored(self, ignored: impl Into<bool>) -> RuleWithIgnoredLastConfigured<WorldImpl> {
//         RuleWithIgnoredLastConfigured {
//             description: self.description,
//             ignored: Some(ignored.into()),

//             phantom: PhantomData,
//         }
//     }

//     pub fn with_background<GivenStepFnImpl>(self, background: impl Into<FinalizedBackground<GivenStepFnImpl, WorldImpl>>) -> RuleWithBackgroundLastConfigured<GivenStepFnImpl, WorldImpl>
//     where
//         GivenStepFnImpl: GivenStepFn<WorldImpl>,
//     {
//         RuleWithBackgroundLastConfigured {
//             description: self.description,
//             ignored: None,

//             background: Some(background.into()),
//         }
//     }

//     pub fn with_scenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl>(self, scenario: impl Into<FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>) -> RuleWithScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
//     where
//         GivenStepFnImpl: GivenStepFn<WorldImpl>,
//         WhenStepFnImpl: WhenStepFn<WorldImpl>,
//         ThenStepFnImpl: ThenStepFn<WorldImpl>,
//     {
//         RuleWithScenariosLastConfigured {
//             description: self.description,
//             ignored: None,

//             background: None,
//             scenarios: vec![scenario.into()],
//         }
//     }
// }

// pub struct RuleWithIgnoredLastConfigured<WorldImpl> {
//     description: MaybeOwnedStr,
//     ignored: Option<bool>,

//     phantom: PhantomData<WorldImpl>,
// }

// impl<WorldImpl> RuleWithIgnoredLastConfigured<WorldImpl> 
// where
//     WorldImpl: World,
// {
//     pub fn with_background<GivenStepFnImpl>(self, background: impl Into<FinalizedBackground<GivenStepFnImpl, WorldImpl>>) -> RuleWithBackgroundLastConfigured<GivenStepFnImpl, WorldImpl>
//     where
//         GivenStepFnImpl: GivenStepFn<WorldImpl>,
//     {
//         RuleWithBackgroundLastConfigured {
//             description: self.description,
//             ignored: self.ignored,

//             background: Some(background.into()),
//         }
//     }

//     pub fn with_scenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl>(self, scenario: impl Into<FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>) -> RuleWithScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
//     where
//         GivenStepFnImpl: GivenStepFn<WorldImpl>,
//         WhenStepFnImpl: WhenStepFn<WorldImpl>,
//         ThenStepFnImpl: ThenStepFn<WorldImpl>,
//     {
//         RuleWithScenariosLastConfigured {
//             description: self.description,
//             ignored: self.ignored,

//             background: None,
//             scenarios: vec![scenario.into()],
//         }
//     }
// }

// pub struct RuleWithBackgroundLastConfigured<GivenStepFnImpl, WorldImpl> {
//     description: MaybeOwnedStr,
//     ignored: Option<bool>,

//     background: Option<FinalizedBackground<GivenStepFnImpl, WorldImpl>>,
// }

// impl<GivenStepFnImpl, WorldImpl> RuleWithBackgroundLastConfigured<GivenStepFnImpl, WorldImpl>
// where
//     GivenStepFnImpl: GivenStepFn<WorldImpl>,
//     WorldImpl: World,
// {
//     pub fn with_scenario<WhenStepFnImpl, ThenStepFnImpl>(self, scenario: impl Into<FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>) -> RuleWithScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
//     where
//         WhenStepFnImpl: WhenStepFn<WorldImpl>,
//         ThenStepFnImpl: ThenStepFn<WorldImpl>,
//     {
//         RuleWithScenariosLastConfigured {
//             description: self.description,
//             ignored: self.ignored,

//             background: self.background,
//             scenarios: vec![scenario.into()],
//         }
//     }
// }

// pub struct RuleWithScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> {
//     description: MaybeOwnedStr,
//     ignored: Option<bool>,

//     background: Option<FinalizedBackground<GivenStepFnImpl, WorldImpl>>,
//     scenarios: Vec<FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>,
// }

// impl<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> RuleWithScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
// where
//     GivenStepFnImpl: GivenStepFn<WorldImpl>,
//     WhenStepFnImpl: WhenStepFn<WorldImpl>,
//     ThenStepFnImpl: ThenStepFn<WorldImpl>,
//     WorldImpl: World,
// {
//     pub fn with_scenario(self, scenario: impl Into<FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>) -> Self {
//         Self {
//             scenarios: self.scenarios.with(scenario.into()),
//             ..self
//         }
//     }
// }

// impl<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> From<RuleWithScenariosLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>> for FinalizedRule<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
// where
//     GivenStepFnImpl: GivenStepFn<WorldImpl>,
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

// pub struct FinalizedRule<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> {
//     pub(super) description: MaybeOwnedStr,
//     pub(super) ignored: bool,

//     pub(super) background: Option<FinalizedBackground<GivenStepFnImpl, WorldImpl>>,
//     pub(super) scenarios: Vec<FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>>,

//     phantom: PhantomData<WorldImpl>,
// }
