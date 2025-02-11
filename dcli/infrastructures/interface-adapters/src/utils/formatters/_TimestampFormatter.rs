use std::time::SystemTime;

use chrono::DateTime;
use chrono::Utc;
use domain::utils::Timestamp;

pub struct TimestampFormatter {}

impl TimestampFormatter {
    pub fn format(timestamp: Timestamp) -> String {
        let system_time = SystemTime::now() + (timestamp - Timestamp::now());
        let date_time: DateTime<Utc> = system_time.into();
        return date_time.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    }
}
