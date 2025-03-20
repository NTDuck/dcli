use domain::time::Timestamp;

pub trait TimestampProvider {
    fn get_current_timestamp(&self) -> Timestamp;
}
