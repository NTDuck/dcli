use domain::time::Timestamp;

use crate::utils::interfaces::Gateway;

pub trait TimestampFormatter: Gateway {
    fn format(&self, timestamp: Timestamp) -> String;
}
