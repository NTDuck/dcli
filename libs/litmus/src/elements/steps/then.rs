use std::{borrow::Cow, marker::PhantomData};

use crate::{elements::Scenario, utils::elements::{Step, StepLabel}};

pub struct ScenarioThenState<'s, 'g, 'w, 't, G, W, T, World> {
    pub(crate) scenario_description: Option<Cow<'s, str>>,
    pub(crate) given_steps: Vec<Step<'g, G>>,
    pub(crate) when_steps: Vec<Step<'w, W>>,
    pub(crate) then_steps: Vec<Step<'t, T>>,
    pub(crate) _phantom: PhantomData<World>,
}

impl<'s, 'g, 'w, 't, G, W, T, World> ScenarioThenState<'s, 'g, 'w, 't, G, W, T, World>
where
    G: FnOnce(&mut World),
    W: FnOnce(&mut World),
    T: FnOnce(&World),
    World: Default,
{
    pub fn and<String>(self, description: String, body: T) -> Self
    where
        String: Into<Cow<'t, str>>,
    {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            body,
        };

        let mut then_steps = self.then_steps;
        then_steps.push(step);

        Self {
            then_steps,
            ..self
        }
    }

    pub fn but<String>(self, description: String, body: T) -> Self
    where
        String: Into<Cow<'t, str>>,
    {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            body,
        };

        let mut then_steps = self.then_steps;
        then_steps.push(step);

        Self {
            then_steps,
            ..self
        }
    }
}

impl<'s, 'g, 'w, 't, G, W, T, World> From<ScenarioThenState<'s, 'g, 'w, 't, G, W, T, World>> for Scenario<'g, 'w, 't, G, W, T, World> {
    fn from(state: ScenarioThenState<'s, 'g, 'w, 't, G, W, T, World>) -> Self {
        let scenario_description = match state.scenario_description {
            Some(description) => description,
            None => state.given_steps
                .iter()
                .map(|step| format!("{}", step))
                .chain(
                    state.when_steps
                        .iter()
                        .map(|step| format!("{}", step))
                )
                .chain(
                    state.then_steps
                        .iter()
                        .map(|step| format!("{}", step))
                )
                .collect::<Vec<_>>()
                .join(" ")
                .into(),
        };

        Self {
            description: scenario_description.into_owned(),
            given_steps: state.given_steps,
            when_steps: state.when_steps,
            then_steps: state.then_steps,
            _phantom: state._phantom,
        }
    }
}
