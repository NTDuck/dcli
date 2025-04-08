use std::{borrow::Cow, marker::PhantomData};

use crate::utils::elements::{Step, StepLabel};

use super::ScenarioWhenState;

pub struct ScenarioGivenState<'s, 'g, G, World> {
    pub(crate) scenario_description: Option<Cow<'s, str>>,
    pub(crate) given_steps: Vec<Step<'g, G>>,
    pub(crate) _phantom: PhantomData<World>,
}

impl<'s, 'g, G, World> ScenarioGivenState<'s, 'g, G, World>
where
    G: FnOnce(&mut World),
    World: Default,
{
    pub fn and<String>(self, description: String, body: G) -> Self
    where
        String: Into<Cow<'g, str>>,
    {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            body,
        };

        let mut given_steps= self.given_steps;
        given_steps.push(step);

        Self {
            given_steps,
            ..self
        }
    }

    pub fn but<String>(self, description: String, body: G) -> Self
    where
        String: Into<Cow<'g, str>>,
    {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            body,
        };

        let mut given_steps= self.given_steps;
        given_steps.push(step);

        Self {
            given_steps,
            ..self
        }
    }

    pub fn when<'w, String, W>(self, description: String, body: W) -> ScenarioWhenState<'s, 'g, 'w, G, W, World>
    where
        String: Into<Cow<'w, str>>,
        W: FnOnce(&mut World),
    {
        let step = Step {
            label: StepLabel::When,
            description: description.into(),
            body,
        };

        let when_steps = vec![step];

        ScenarioWhenState {
            scenario_description: self.scenario_description,
            given_steps: self.given_steps,
            when_steps,
            _phantom: self._phantom,
        }
    }
}
