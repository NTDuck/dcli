use std::marker::PhantomData;

use crate::elements::aliases::World;
use crate::elements::aliases::GivenStepFn;
use crate::elements::aliases::ThenStepFn;
use crate::elements::aliases::WhenStepFn;
use crate::utils::aliases::MaybeOwnedStr;

pub use UnconfiguredScenario as Scenario;

pub struct UnconfiguredScenario<WorldImpl> {
    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> UnconfiguredScenario<WorldImpl>
where
    WorldImpl: World,
{
    pub fn named(description: impl Into<MaybeOwnedStr>) -> ScenarioWithDescriptionLastConfigured<WorldImpl> {
        ScenarioWithDescriptionLastConfigured {
            description: Some(description.into()),

            phantom: PhantomData,
        }
    }

    pub fn unnamed() -> ScenarioWithDescriptionLastConfigured<WorldImpl> {
        ScenarioWithDescriptionLastConfigured {
            description: None,

            phantom: PhantomData,
        }
    }
}

pub struct ScenarioWithDescriptionLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,

    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> ScenarioWithDescriptionLastConfigured<WorldImpl>
where
    WorldImpl: World,
{
    pub fn ignored(self, ignored: impl Into<bool>) -> ScenarioWithIgnoredLastConfigured<WorldImpl> {
        ScenarioWithIgnoredLastConfigured {
            description: self.description,
            ignored: Some(ignored.into()),

            phantom: PhantomData,
        }
    }

    pub fn given<GivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<GivenStepFnImpl>) -> ScenarioWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl>
    where
        GivenStepFnImpl: GivenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback: callback.into(),
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            ignored: None,

            given_steps: vec![step],

            phantom: PhantomData,
        }
    }
}

pub struct ScenarioWithIgnoredLastConfigured<WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    phantom: PhantomData<WorldImpl>,
}

impl<WorldImpl> ScenarioWithIgnoredLastConfigured<WorldImpl> 
where
    WorldImpl: World,
{
    pub fn given<GivenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<GivenStepFnImpl>) -> ScenarioWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl>
    where
        GivenStepFnImpl: GivenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            callback: callback.into(),
        };

        ScenarioWithGivenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: vec![step],

            phantom: PhantomData,
        }
    }
}

pub struct ScenarioWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: Vec<Step<GivenStepFnImpl>>,

    phantom: PhantomData<WorldImpl>,
}

impl<GivenStepFnImpl, WorldImpl> ScenarioWithGivenStepsLastConfigured<GivenStepFnImpl, WorldImpl>
where
    GivenStepFnImpl: GivenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<GivenStepFnImpl>) -> Self {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback: callback.into(),
        };

        Self {
            given_steps: self.given_steps.with(step),
            ..self
        }
    }

    pub fn but(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<GivenStepFnImpl>) -> Self {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback: callback.into(),
        };

        Self {
            given_steps: self.given_steps.with(step),
            ..self
        }
    }

    pub fn when<WhenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<WhenStepFnImpl>) -> ScenarioWithWhenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, WorldImpl>
    where
        WhenStepFnImpl: WhenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::When,
            description: description.into(),
            callback: callback.into(),
        };

        ScenarioWithWhenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: vec![step],

            phantom: PhantomData,
        }
    }
}

pub struct ScenarioWithWhenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: Vec<Step<GivenStepFnImpl>>,
    when_steps: Vec<Step<WhenStepFnImpl>>,

    phantom: PhantomData<WorldImpl>,
}

impl<GivenStepFnImpl, WhenStepFnImpl, WorldImpl> ScenarioWithWhenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, WorldImpl>
where
    GivenStepFnImpl: GivenStepFn<WorldImpl>,
    WhenStepFnImpl: WhenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<WhenStepFnImpl>) -> Self {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback: callback.into(),
        };

        Self {
            when_steps: self.when_steps.with(step),
            ..self
        }
    }

    pub fn but(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<WhenStepFnImpl>) -> Self {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback: callback.into(),
        };

        Self {
            when_steps: self.when_steps.with(step),
            ..self
        }
    }

    pub fn then<ThenStepFnImpl>(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<ThenStepFnImpl>) -> ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
    where
        ThenStepFnImpl: ThenStepFn<WorldImpl>,
    {
        let step = Step {
            label: StepLabel::Then,
            description: description.into(),
            callback: callback.into(),
        };

        ScenarioWithThenStepsLastConfigured {
            description: self.description,
            ignored: self.ignored,

            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps: vec![step],

            phantom: PhantomData,
        }
    }
}

pub struct ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,

    given_steps: Vec<Step<GivenStepFnImpl>>,
    when_steps: Vec<Step<WhenStepFnImpl>>,
    then_steps: Vec<Step<ThenStepFnImpl>>,

    phantom: PhantomData<WorldImpl>,
}

impl<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
where
    GivenStepFnImpl: GivenStepFn<WorldImpl>,
    WhenStepFnImpl: WhenStepFn<WorldImpl>,
    ThenStepFnImpl: ThenStepFn<WorldImpl>,
    WorldImpl: World,
{
    pub fn and(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<ThenStepFnImpl>) -> Self {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            callback: callback.into(),
        };

        Self {
            then_steps: self.then_steps.with(step),
            ..self
        }
    }

    pub fn but(self, description: impl Into<MaybeOwnedStr>, callback: impl Into<ThenStepFnImpl>) -> Self {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            callback: callback.into(),
        };

        Self {
            then_steps: self.then_steps.with(step),
            ..self
        }
    }
}

impl<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> From<ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>> for FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>
where
    GivenStepFnImpl: GivenStepFn<WorldImpl>,
    WhenStepFnImpl: WhenStepFn<WorldImpl>,
    ThenStepFnImpl: ThenStepFn<WorldImpl>,
    WorldImpl: World,
{
    fn from(scenario: ScenarioWithThenStepsLastConfigured<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>) -> Self {
        let ScenarioWithThenStepsLastConfigured {
            description,
            ignored,

            given_steps,
            when_steps,
            then_steps,
            ..
        } = scenario;

        return Self {
            description: match description {
                Some(description) => description,
                None => compute_default_description(&given_steps, &when_steps, &then_steps),
            },
            ignored: match ignored {
                Some(ignored) => ignored,
                None => false,
            },

            given_step_callbacks: given_steps.callbacks(),
            when_step_callbacks: when_steps.callbacks(),
            then_step_callbacks: then_steps.callbacks(),

            phantom: PhantomData,
        };

        fn compute_default_description<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl>(
            given_steps: &Vec<Step<GivenStepFnImpl>>,
            when_steps: &Vec<Step<WhenStepFnImpl>>,
            then_steps: &Vec<Step<ThenStepFnImpl>>,
        ) -> MaybeOwnedStr
        where
            GivenStepFnImpl: GivenStepFn<WorldImpl>,
            WhenStepFnImpl: WhenStepFn<WorldImpl>,
            ThenStepFnImpl: ThenStepFn<WorldImpl>,
            WorldImpl: World,
        {
            std::iter::empty()
                .chain(given_steps.format())
                .chain(when_steps.format())
                .chain(then_steps.format())
                .collect::<Vec<_>>()
                .join(" | ")
                .into()
        }
    }
}

pub(super) struct FinalizedScenario<GivenStepFnImpl, WhenStepFnImpl, ThenStepFnImpl, WorldImpl> {
    description: MaybeOwnedStr,
    ignored: bool,

    given_step_callbacks: Vec<GivenStepFnImpl>,
    when_step_callbacks: Vec<WhenStepFnImpl>,
    then_step_callbacks: Vec<ThenStepFnImpl>,

    phantom: PhantomData<WorldImpl>,
}

struct Step<StepFnImpl> {
    pub label: StepLabel,
    pub description: MaybeOwnedStr,
    pub callback: StepFnImpl,
}

impl<StepFnImpl> std::fmt::Display for Step<StepFnImpl> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} {}", self.label, self.description)
    }
}

#[derive(strum::Display)]
enum StepLabel {
    Given,
    When,
    Then,
    And,
    But,
}

trait VecExt<T> {
    fn with(self, item: T) -> Self;
}

impl<T> VecExt<T> for Vec<T> {
    fn with(self, item: T) -> Self {
        let mut this = self;
        this.push(item);
        this
    }
}

trait VecStepExt<StepFnImpl> {
    fn format(&self) -> impl IntoIterator<Item = String>;
    fn callbacks(self) -> Vec<StepFnImpl>;
}

impl<StepFnImpl> VecStepExt<StepFnImpl> for Vec<Step<StepFnImpl>> {
    fn format(&self) -> impl IntoIterator<Item = String> {
        self
            .iter()
            .map(|step| format!("{}", step))
    }
    
    fn callbacks(self) -> Vec<StepFnImpl> {
        self
            .into_iter()
            .map(|step| step.callback)
            .collect()
    }
}
