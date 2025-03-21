use domain::ids::SnowflakeWorkerNumber;
use domain::ids::SnowflakeSequenceNumber;

pub trait SnowflakeProvider {
    fn get_worker_number(&self) -> SnowflakeWorkerNumber;
    fn get_sequence_number(&self) -> SnowflakeSequenceNumber;
}
