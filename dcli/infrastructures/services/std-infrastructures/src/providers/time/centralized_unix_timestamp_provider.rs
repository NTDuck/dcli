use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use axiom::behaviours::New;
use domain::time::Timestamp;
use gateways::providers::time::TimestampProvider;

#[derive(New)]
pub struct CentralizedSystemTimestampProvider;

impl TimestampProvider for CentralizedSystemTimestampProvider {
    fn get_current_timestamp(&self) -> Timestamp {
        let current_system_time = SystemTime::now();
        let millis_since_epoch = current_system_time
            .duration_since(Self::EPOCH).unwrap()
            .as_millis() as i64;
        return Timestamp::from_millis_since_epoch(millis_since_epoch);
    }

}

impl CentralizedSystemTimestampProvider {
    const EPOCH: SystemTime = UNIX_EPOCH;
}
