use domain::ids::Snowflake;
use domain::ids::SnowflakeWorkerNumber;
use domain::ids::SnowflakeSequenceNumber;
use domain::time::Timestamp;

pub trait SnowflakeProvider {
    fn new_snowflake_from_timestamp(&self, timestamp: Timestamp) -> Snowflake {
        let worker_number = self.get_worker_number();
        let sequence_number = self.get_sequence_number();

        return Snowflake::new(timestamp, worker_number, sequence_number);
    }

    fn get_worker_number(&self) -> SnowflakeWorkerNumber;
    fn get_sequence_number(&self) -> SnowflakeSequenceNumber;
}
