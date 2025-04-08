use std::{borrow::Cow, marker::PhantomData};

use crate::utils::elements::{Step, StepLabel};

use super::ScenarioThenState;

pub struct ScenarioWhenState<'s, 'g, 'w, G, W, World> {
    pub(crate) scenario_description: Option<Cow<'s, str>>,
    pub(crate) given_steps: Vec<Step<'g, G>>,
    pub(crate) when_steps: Vec<Step<'w, W>>,
    pub(crate) _phantom: PhantomData<World>,
}

impl<'s, 'g, 'w, G, W, World> ScenarioWhenState<'s, 'g, 'w, G, W, World>
where
    G: FnOnce(&mut World),
    W: FnOnce(&mut World),
    World: Default,
{
    pub fn and<String>(self, description: String, body: W) -> Self
    where
        String: Into<Cow<'w, str>>,
    {
        let step = Step {
            label: StepLabel::And,
            description: description.into(),
            body,
        };

        let mut when_steps = self.when_steps;
        when_steps.push(step);

        Self {
            when_steps,
            ..self
        }
    }

    pub fn but<String>(self, description: String, body: W) -> Self
    where
        String: Into<Cow<'w, str>>,
    {
        let step = Step {
            label: StepLabel::But,
            description: description.into(),
            body,
        };

        let mut when_steps = self.when_steps;
        when_steps.push(step);

        Self {
            when_steps,
            ..self
        }
    }

    pub fn then<'t, String, T>(self, description: String, body: T) -> ScenarioThenState<'s, 'g, 'w, 't, G, W, T, World>
    where
        String: Into<Cow<'t, str>>,
        T: FnOnce(&World),
    {
        let step = Step {
            label: StepLabel::Then,
            description: description.into(),
            body,
        };

        let then_steps = vec![step];

        ScenarioThenState {
            scenario_description: self.scenario_description,
            given_steps: self.given_steps,
            when_steps: self.when_steps,
            then_steps,
            _phantom: self._phantom,
        }
    }
}
