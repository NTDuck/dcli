use axiom::behaviours::New;
use axiom::behaviours::NewType;
use axiom::interfaces::ddd;

#[derive(ddd::Identifier, New, NewType)]
pub struct WorkerId(String);
