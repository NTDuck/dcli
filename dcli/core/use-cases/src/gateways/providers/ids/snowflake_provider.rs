use domain::ids::SnowflakeSequenceNumber;
use domain::ids::SnowflakeWorkerNumber;

use crate::utils::interfaces::Gateway;

pub trait SnowflakeProvider: Gateway {
    fn get_worker_number(&self) -> SnowflakeWorkerNumber;
    fn get_sequence_number(&self) -> SnowflakeSequenceNumber;
}
