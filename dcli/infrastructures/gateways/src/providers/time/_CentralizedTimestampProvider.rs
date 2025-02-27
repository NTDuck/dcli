use std::time::SystemTime;

use domain::time::Timestamp;
use use_cases::gateways::providers::time::TimestampProvider;

pub struct CentralizedTimestampProvider;

impl TimestampProvider for CentralizedTimestampProvider {
    fn get_current_timestamp(&self) -> Timestamp {
        return Timestamp::from_system_time(SystemTime::now());
    }
}
