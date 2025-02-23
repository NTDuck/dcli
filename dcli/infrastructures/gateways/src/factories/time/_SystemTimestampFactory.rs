use std::time::SystemTime;

use axiom::behaviours::New;
use domain::time::Timestamp;
use use_cases::gateways::factories::time::TimestampFactory;

#[derive(New)]
pub struct SystemTimestampFactory;

impl TimestampFactory for SystemTimestampFactory {
    fn currentTimestamp(&self) -> Timestamp {
        return Timestamp::new(SystemTime::now());
    }
}
