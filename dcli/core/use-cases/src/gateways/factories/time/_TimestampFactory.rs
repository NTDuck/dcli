use domain::time::Timestamp;

pub trait TimestampFactory {
    fn currentTimestamp(&self) -> Timestamp;
}
