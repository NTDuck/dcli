use domain::ids::{Snowflake, SnowflakeSequenceNumber, SnowflakeWorkerNumber};

use crate::params::Param;

impl From<Param<usize>> for Snowflake {
    fn from(Param(millis): Param<usize>) -> Self {
        Self::from(Param((millis, 0, 0)))
    }
}

impl From<Param<(usize, usize, usize)>> for Snowflake {
    fn from(Param((millis, worker_number, sequence_number)): Param<(usize, usize, usize)>) -> Self {
        let timestamp = Param(millis).into();
        let worker_number = worker_number as SnowflakeWorkerNumber;
        let sequence_number = sequence_number as SnowflakeSequenceNumber;

        Self::new(timestamp, worker_number, sequence_number)
    }
}
