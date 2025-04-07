use std::borrow::Cow;
use std::mem::MaybeUninit;

use crate::utils::elements::Step;

pub struct GivenState<'sd, 'gd, GF> {
    pub(crate) scenario_description: MaybeUninit<Cow<'sd, str>>,
    pub(crate) given_steps: Vec<Step<'gd, GF>>,
}

impl<'sd, 'gd, GF, World> GivenState<'sd, 'gd, GF>
where
    GF: FnOnce(&mut World),
    World: Default,
{
    pub fn and(self, )
}