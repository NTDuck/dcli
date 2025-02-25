use domain::ids::SnowflakeSequenceNumber;
use domain::ids::WorkerId;

pub trait WorkerIdEncoder {
    fn encodeWorkerId(&self, workerId: &WorkerId) -> SnowflakeSequenceNumber;
}
