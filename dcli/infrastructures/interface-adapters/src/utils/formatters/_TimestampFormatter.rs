use std::time::SystemTime;

use chrono::DateTime;
use chrono::Utc;
use domain::utils::dataclasses::time::Timestamp;

pub(crate) struct TimestampFormatter {}

impl TimestampFormatter {
    pub(crate) fn format(timestamp: Timestamp) -> String {
        let systemTime = SystemTime::now() + (timestamp - Timestamp::now());
        let dateTime: DateTime<Utc> = systemTime.into();
        return dateTime.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    }
}
