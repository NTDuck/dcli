use std::borrow::Cow;
use std::marker::PhantomData;

use crate::elements::steps::ScenarioGivenState;
use crate::utils::elements::{Step, StepLabel};

pub struct Scenario<'g, 'w, 't, G, W, T, World>{
    pub(crate) description: String,
    pub(crate) given_steps: Vec<Step<'g, G>>,
    pub(crate) when_steps: Vec<Step<'w, W>>,
    pub(crate) then_steps: Vec<Step<'t, T>>,
    pub(crate) _phantom: PhantomData<World>,
}

impl<'g, 'w, 't, G, W, T, World> Scenario<'g, 'w, 't, G, W, T, World>
where
    G: FnOnce(&mut World),
    W: FnOnce(&mut World),
    T: FnOnce(&World),
    World: Default,
{
    pub fn named<'s, String>(description: String) -> ScenarioEmptyState<'s>
    where
        String: Into<Cow<'s, str>>,
    {
        ScenarioEmptyState {
            scenario_description: Some(description.into()),
        }
    }

    pub fn unnamed<'s>() -> ScenarioEmptyState<'s> {
        ScenarioEmptyState {
            scenario_description: None,
        }
    }
}

impl<'g, 'w, 't, G, W, T, World> From<Scenario<'g, 'w, 't, G, W, T, World>> for libtest::Trial
where
    G: FnOnce(&mut World) + Send,
    W: FnOnce(&mut World) + Send,
    T: FnOnce(&World) + Send,
    World: Default,
{
    fn from(scenario: Scenario<'g, 'w, 't, G, W, T, World>) -> Self {
        let body = || {
            let mut world = World::default();

            scenario.given_steps
                .into_iter()
                .map(|step| step.body)
                .for_each(|given| given(&mut world));

            scenario.when_steps
                .into_iter()
                .map(|step| step.body)
                .for_each(|when| when(&mut world));

            scenario.then_steps
                .into_iter()
                .map(|step| step.body)
                .for_each(|then| then(&world));

            Result::<(), libtest::Failed>::Ok(())
        };
        
        libtest::Trial::test(scenario.description, body)
    }
}

pub struct ScenarioEmptyState<'s> {
    pub(crate) scenario_description: Option<Cow<'s, str>>,
}

impl<'s> ScenarioEmptyState<'s> {
    pub fn given<'g, String, G, World>(self, description: String, body: G) -> ScenarioGivenState<'s, 'g, G, World>
    where
        String: Into<Cow<'g, str>>,
        G: FnOnce(&mut World),
        World: Default,
    {
        let step = Step {
            label: StepLabel::Given,
            description: description.into(),
            body,
        };

        let given_steps = vec![step];

        ScenarioGivenState {
            scenario_description: self.scenario_description,
            given_steps,
            _phantom: PhantomData,
        }
    }
}
