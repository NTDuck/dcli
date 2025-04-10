// use std::marker::PhantomData;
// use std::sync::Arc;

// use crate::elements::step::World;
// use crate::elements::step::GivenStepFn;
// use crate::elements::step::StepMeta;
// use crate::elements::step::StepLabel;
// use crate::elements::VecStepMetaExt;
// use crate::elements::VecExt;
// use crate::utils::aliases::MaybeOwnedStr;

// pub use UnconfiguredBackground as Background;

// pub struct UnconfiguredBackground<WorldImpl> {
//     phantom: PhantomData<WorldImpl>,
// }

// impl<WorldImpl> UnconfiguredBackground<WorldImpl>
// where
//     WorldImpl: World,
// {
//     pub fn named(description: impl Into<MaybeOwnedStr>) -> BackgroundWithDescriptionLastConfigured<WorldImpl> {
//         BackgroundWithDescriptionLastConfigured {
//             description: description.into(),

//             phantom: PhantomData,
//         }
//     }
// }

// pub struct BackgroundWithDescriptionLastConfigured<WorldImpl> {
//     description: MaybeOwnedStr,

//     phantom: PhantomData<WorldImpl>,
// }

// impl<WorldImpl> BackgroundWithDescriptionLastConfigured<WorldImpl>
// where
//     WorldImpl: World,
// {
//     pub fn ignored(self, ignored: impl Into<bool>) -> BackgroundWithIgnoredLastConfigured<WorldImpl> {
//         BackgroundWithIgnoredLastConfigured {
//             description: self.description,
//             ignored: Some(ignored.into()),

//             phantom: PhantomData,
//         }
//     }

//     pub fn given<GivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: GivenStepFnImpl) -> BackgroundWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl>
//     where
//         GivenStepFnImpl: GivenStepFn<WorldImpl>,
//     {
//         let step = StepMeta {
//             label: StepLabel::Given,
//             description: description.into(),
//             callback: callback.into(),
//         };

//         BackgroundWithGivenStepsLastConfigured {
//             description: self.description,
//             ignored: None,

//             given_steps: vec![step],

//             phantom: PhantomData,
//         }
//     }
// }

// pub struct BackgroundWithIgnoredLastConfigured<WorldImpl> {
//     description: MaybeOwnedStr,
//     ignored: Option<bool>,

//     phantom: PhantomData<WorldImpl>,
// }

// impl<WorldImpl> BackgroundWithIgnoredLastConfigured<WorldImpl> 
// where
//     WorldImpl: World,
// {
//     pub fn given<GivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: GivenStepFnImpl) -> BackgroundWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl>
//     where
//         GivenStepFnImpl: GivenStepFn<WorldImpl>,
//     {
//         let step = StepMeta {
//             label: StepLabel::Given,
//             description: description.into(),
//             callback: callback.into(),
//         };

//         BackgroundWithGivenStepsLastConfigured {
//             description: self.description,
//             ignored: self.ignored,

//             given_steps: vec![step],

//             phantom: PhantomData,
//         }
//     }
// }

// pub struct BackgroundWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl> {
//     description: MaybeOwnedStr,
//     ignored: Option<bool>,

//     given_steps: Vec<StepMeta<GivenStepFnImpl>>,

//     phantom: PhantomData<WorldImpl>,
// }

// impl<GivenStepFnImpl, WorldImpl> BackgroundWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl>
// where
//     GivenStepFnImpl: GivenStepFn<WorldImpl>,
//     WorldImpl: World,
// {
//     pub fn and(self, description: impl Into<MaybeOwnedStr>, callback: GivenStepFnImpl) -> Self {
//         let step = StepMeta {
//             label: StepLabel::And,
//             description: description.into(),
//             callback: callback.into(),
//         };

//         Self {
//             given_steps: self.given_steps.with(step),
//             ..self
//         }
//     }

//     pub fn but(self, description: impl Into<MaybeOwnedStr>, callback: GivenStepFnImpl) -> Self {
//         let step = StepMeta {
//             label: StepLabel::But,
//             description: description.into(),
//             callback: callback.into(),
//         };

//         Self {
//             given_steps: self.given_steps.with(step),
//             ..self
//         }
//     }
// }

// impl<GivenStepFnImpl, WorldImpl> From<BackgroundWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl>> for FinalizedBackground<GivenStepFnImpl, WorldImpl>
// where
//     GivenStepFnImpl: GivenStepFn<WorldImpl>,
//     WorldImpl: World,
// {
//     fn from(background: BackgroundWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl>) -> Self {
//         let BackgroundWithGivenStepsLastConfigured {
//             description,
//             ignored,

//             given_steps,
//             ..
//         } = background;

//         Self {
//             description,
//             ignored: match ignored {
//                 Some(ignored) => ignored,
//                 None => false,
//             },

//             given_step_callbacks: given_steps.callbacks()
//                 .into_iter()
//                 .map(Arc::new)
//                 .collect(),

//             phantom: PhantomData,
//         }
//     }
// }

// #[derive(Clone)]
// pub struct FinalizedBackground<GivenStepFnImpl, WorldImpl> {
//     pub(super) description: MaybeOwnedStr,
//     pub(super) ignored: bool,

//     pub(super) given_step_callbacks: Vec<Arc<GivenStepFnImpl>>,

//     phantom: PhantomData<WorldImpl>,
// }
