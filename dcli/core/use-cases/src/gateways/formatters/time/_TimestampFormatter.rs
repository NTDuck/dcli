use domain::time::Timestamp;

pub trait TimestampFormatter {
    fn format(&self, timestamp: Timestamp) -> String;
}
