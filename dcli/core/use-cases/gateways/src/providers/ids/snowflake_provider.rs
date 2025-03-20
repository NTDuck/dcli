use domain::ids::Snowflake;
use domain::ids::SnowflakeWorkerNumber;
use domain::ids::SnowflakeSequenceNumber;
use domain::time::Timestamp;

pub trait SnowflakeProvider {
    fn new_snowflake_from_timestamp(&self, timestamp: Timestamp) -> Snowflake {
        let workerNumber = self.get_worker_number();
        let sequenceNumber = self.get_sequence_number();

        return Snowflake::new(timestamp, workerNumber, sequenceNumber);
    }

    fn get_worker_number(&self) -> SnowflakeWorkerNumber;
    fn get_sequence_number(&self) -> SnowflakeSequenceNumber;
}
