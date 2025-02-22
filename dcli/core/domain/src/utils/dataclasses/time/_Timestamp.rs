use std::time::SystemTime;

use axiom::behaviours::NewType;
use axiom::interfaces::ddd;

#[derive(ddd::ValueObject, NewType, PartialOrd, Ord)]
pub struct Timestamp(SystemTime);

impl Timestamp {
    pub fn now() -> Self {
        return Self(SystemTime::now());
    }
}
