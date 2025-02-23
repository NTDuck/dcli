use std::ops::Deref;
use std::time::Duration;

use axiom::behaviours::New;
use chrono::Utc;
use domain::time::Epoch;
use domain::time::Timestamp;
use use_cases::gateways::factories::time::TimestampFactory;

#[derive(New)]
pub struct ChronoTimestampFactory;

impl TimestampFactory for ChronoTimestampFactory {
    fn currentTimestamp(&self) -> Timestamp {
        let epoch = Epoch.deref().clone();
        let durationSinceEpoch = Duration::from_secs(Utc::now().timestamp() as u64);
        return Timestamp::new(epoch + durationSinceEpoch);
    }
}
