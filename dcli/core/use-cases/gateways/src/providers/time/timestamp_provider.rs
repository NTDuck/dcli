use domain::time::Timestamp;

pub trait TimestampProvider: Send + Sync {
    fn get_current_timestamp(&self) -> Timestamp;
}
