use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use axiom::behaviours::New;
use axiom::behaviours::NewType;
use axiom::interfaces::ddd;

#[derive(ddd::ValueObject, New, NewType, PartialOrd, Ord)]
pub struct Timestamp(SystemTime);

pub const Epoch: Timestamp = Timestamp(UNIX_EPOCH);
