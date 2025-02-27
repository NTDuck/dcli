use axiom::behaviours::NewType;
use axiom::interfaces::ddd;

use crate::time::EPOCH;
use crate::time::Interval;
use crate::time::Timestamp;

/// ### See also:
/// - [Twitter's announcement](https://blog.x.com/engineering/en_us/a/2010/announcing-snowflake)
/// - [Comparison to UUID](https://softwaremind.com/blog/the-unique-features-of-snowflake-id-and-its-comparison-to-uuid/)
#[derive(ddd::Identifier, NewType)]
#[derive(Copy, PartialOrd, Ord)]
pub struct Snowflake(u64);

impl Snowflake {
    pub fn new(
        timestamp: Timestamp,
        worker_number: SnowflakeWorkerNumber,
        sequence_number: SnowflakeSequenceNumber,
    ) -> Self {
        let encoded_timestamp = Self::encode_timestamp(timestamp);
        let encoded_worker_number = Self::encode_worker_number(worker_number);
        let encoded_sequence_number = Self::encode_sequence_number(sequence_number);

        return Self(encoded_timestamp | encoded_worker_number | encoded_sequence_number);
    }

    pub fn get_timestamp(&self) -> Timestamp {
        let inner = self.as_inner();
        let encoded_millis = (inner >> TIMESTAMP_SHIFT) & TIMESTAMP_BITMASK;
        let encoded_interval = Interval::from_millis(encoded_millis);
        
        return EPOCH
            .checked_add(encoded_interval)
            .expect("Timestamp overflow");
    }

    pub fn get_worker_number(&self) -> SnowflakeWorkerNumber {
        let inner = self.as_inner();
        return ((inner >> WORKER_NUMBER_SHIFT) & WORKER_NUMBER_BITMASK) as SnowflakeWorkerNumber;
    }

    pub fn get_sequence_number(&self) -> SnowflakeSequenceNumber {
        let inner = self.as_inner();
        return ((inner >> SEQUENCE_NUMBER_SHIFT) & SEQUENCE_NUMBER_BITMASK) as SnowflakeSequenceNumber;
    }

    fn encode_timestamp(timestamp: Timestamp) -> u64 {
        let interval = timestamp.computer_interval_since(EPOCH)
            .expect("Timestamp earlier than Epoch");
        let millis = interval.as_millis() as u64;
        
        return (millis & TIMESTAMP_BITMASK) << TIMESTAMP_SHIFT;
    }

    fn encode_worker_number(worker_number: SnowflakeWorkerNumber) -> u64 {
        let worker_number = worker_number as u64;
        return (worker_number & WORKER_NUMBER_BITMASK) << WORKER_NUMBER_SHIFT;
    }

    fn encode_sequence_number(sequenceNumber: SnowflakeSequenceNumber) -> u64 {
        let sequenceNumber = sequenceNumber as u64;
        return (sequenceNumber & SEQUENCE_NUMBER_BITMASK) << SEQUENCE_NUMBER_SHIFT;
    }

    fn as_inner(&self) -> u64 {
        return self.0;
    }
}

pub type SnowflakeWorkerNumber = u16;
pub type SnowflakeSequenceNumber = u16;

const SNOWFLAKE_BITS: usize = 64;
const RESERVED_BITS: usize = 1;
const TIMESTAMP_BITS: usize = 41;
const WORKER_NUMBER_BITS: usize = 10;
const SEQUENCE_NUMBER_BITS: usize = 12;

const TIMESTAMP_BITMASK: u64 = 0x1ffffffffff;
const WORKER_NUMBER_BITMASK: u64 = 0x3ff;
const SEQUENCE_NUMBER_BITMASK: u64 = 0x0fff;

const TIMESTAMP_SHIFT: usize = SNOWFLAKE_BITS - RESERVED_BITS - TIMESTAMP_BITS;
const WORKER_NUMBER_SHIFT: usize = TIMESTAMP_SHIFT - WORKER_NUMBER_BITS;
const SEQUENCE_NUMBER_SHIFT: usize = WORKER_NUMBER_SHIFT - SEQUENCE_NUMBER_BITS;
