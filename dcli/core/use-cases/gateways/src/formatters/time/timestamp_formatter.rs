use domain::time::Timestamp;

pub trait TimestampFormatter: Send + Sync {
    fn format(&self, timestamp: Timestamp) -> String;
}
