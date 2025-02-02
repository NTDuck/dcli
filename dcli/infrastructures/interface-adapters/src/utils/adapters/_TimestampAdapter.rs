use std::time::{Instant, SystemTime};

use chrono::{DateTime, Utc};

pub struct TimestampAdapter {}

impl TimestampAdapter {
    pub fn format(timestamp: Instant) -> String {
        let system_time = SystemTime::now() + (timestamp - Instant::now());
        let date_time: DateTime<Utc> = system_time.into();
        return date_time.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    }
}
