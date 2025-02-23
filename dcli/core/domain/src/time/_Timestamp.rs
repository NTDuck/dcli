use std::time::SystemTime;

use axiom::behaviours::NewType;
use axiom::interfaces::ddd;

#[derive(ddd::ValueObject, NewType, PartialOrd, Ord)]
pub struct Timestamp(SystemTime);

pub const UnixEpoch: Timestamp = Timestamp(std::time::UNIX_EPOCH);
