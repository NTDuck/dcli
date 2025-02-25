use domain::ids::Snowflake;
use domain::ids::SnowflakeWorkerNumber;
use domain::ids::SnowflakeSequenceNumber;
use domain::time::Timestamp;

pub trait SnowflakeProvider {
    fn newSnowflakeFromCurrentTimestamp(&self) -> Snowflake {
        let currentTimestamp = Timestamp::current();
        return self.newSnowflakeFromTimestamp(currentTimestamp);
    }

    fn newSnowflakeFromTimestamp(&self, timestamp: Timestamp) -> Snowflake {
        let workerNumber = self.getWorkerNumber();
        let sequenceNumber = self.getSequenceNumber();

        return Snowflake::new(timestamp, workerNumber, sequenceNumber);
    }

    fn getWorkerNumber(&self) -> SnowflakeWorkerNumber;
    fn getSequenceNumber(&self) -> SnowflakeSequenceNumber;
}
