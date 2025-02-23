use axiom::behaviours::New;
use axiom::behaviours::NewType;
use axiom::interfaces::ddd;

#[derive(ddd::Identifier, Copy, New, NewType)]
pub struct MachineId(u16);
