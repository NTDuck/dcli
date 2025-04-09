// use std::marker::PhantomData;

// use crate::elements::aliases::World;
// use crate::utils::aliases::MaybeOwnedStr;

// pub use UnconfiguredFeature as Feature;

// pub struct UnconfiguredFeature<WorldImpl> {
//     phantom: PhantomData<WorldImpl>,
// }

// impl<WorldImpl> UnconfiguredFeature<WorldImpl>
// where
//     WorldImpl: World,
// {
//     pub fn named(value: impl Into<MaybeOwnedStr>) -> FeatureWithDescriptionLastConfigured<WorldImpl> {
//         FeatureWithDescriptionLastConfigured {
//             description: Some(value.into()),
//             phantom: PhantomData,
//         }
//     }

//     pub fn unnamed() -> FeatureWithDescriptionLastConfigured<WorldImpl> {
//         FeatureWithDescriptionLastConfigured {
//             description: None,
//             phantom: PhantomData,
//         }
//     }
// }

// pub struct FeatureWithDescriptionLastConfigured<WorldImpl> {
//     description: Option<MaybeOwnedStr>,

//     phantom: PhantomData<WorldImpl>,
// }

// impl<WorldImpl> FeatureWithDescriptionLastConfigured<WorldImpl>
// where
//     WorldImpl: World,
// {
//     pub fn ignored(self, value: impl Into<bool>) -> FeatureWithIgnoredLastConfigured<WorldImpl> {
//         FeatureWithIgnoredLastConfigured {
//             description: self.description,
//             ignored: value.into(),

//             phantom: PhantomData,
//         }
//     }
// }

// pub struct FeatureWithIgnoredLastConfigured<WorldImpl> {
//     description: Option<MaybeOwnedStr>,
//     ignored: bool,

//     phantom: PhantomData<WorldImpl>,
// }

// pub struct FeatureWithBackgroundLastConfigured<WorldImpl> {
//     phantom: PhantomData<WorldImpl>,
// }

// pub struct FeatureWithRuleOrScenarioLastConfigured<WorldImpl> {
//     phantom: PhantomData<WorldImpl>,
// }

// pub(crate) struct FinalizedFeature<WorldImpl> {
//     phantom: PhantomData<WorldImpl>,
// }

// impl<WorldImpl> From<FinalizedFeature<WorldImpl>> for Vec<libtest::Trial> {
//     fn from(feature: FinalizedFeature<WorldImpl>) -> Self {
//         todo!()
//     }
// }
