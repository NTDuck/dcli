use std::borrow::Cow;
use std::mem::MaybeUninit;

use crate::elements::steps::GivenState;
use crate::utils::elements::{Step, StepLabel};

pub struct Scenario<'sd> {
    pub(crate) scenario_description: Cow<'sd, str>,
}

impl<'sd> Scenario<'sd> {
    pub fn named<S>(description: S) -> ScenarioState<'sd>
    where
        S: Into<Cow<'sd, str>>,
    {
        ScenarioState {
            scenario_description: MaybeUninit::new(description.into()),
        }
    }

    pub fn unnamed() -> ScenarioState<'sd> {
        ScenarioState {
            scenario_description: MaybeUninit::uninit(),
        }
    }
}

impl<'sd> From<Scenario<'sd>> for libtest::Trial {
    fn from(scenario: Scenario<'sd>) -> Self {
        todo!()
    }
}

pub struct ScenarioState<'sd> {
    pub(crate) scenario_description: MaybeUninit<Cow<'sd, str>>,
}

impl<'sd> ScenarioState<'sd> {
    pub fn given<'gd, S, GF, World>(self, description: S, body: GF) -> GivenState<'sd, 'gd, GF>
    where
        S: Into<Cow<'gd, str>>,
        GF: FnOnce(&mut World),
        World: Default,
    {
        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            body,
        };

        GivenState {
            scenario_description: self.scenario_description,
            given_steps: vec![step],
        }
    }
}
