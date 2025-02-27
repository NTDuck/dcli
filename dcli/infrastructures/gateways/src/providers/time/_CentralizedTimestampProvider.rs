use std::time::SystemTime;

use domain::time::Timestamp;
use use_cases::gateways::providers::time::TimestampProvider;

pub struct CentralizedTimestampProvider;

impl TimestampProvider for CentralizedTimestampProvider {
    fn getCurrentTimestamp(&self) -> Timestamp {
        return Timestamp::fromSystemTime(SystemTime::now());
    }
}
