use domain::time::Timestamp;

pub trait TimestampProvider {
    fn getCurrentTimestamp(&self) -> Timestamp;
}
