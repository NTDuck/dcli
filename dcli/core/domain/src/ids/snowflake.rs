use axiom::behaviours::NewType;
use axiom::interfaces::ddd;

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
        let encoded_sequence_number =
            Self::encode_sequence_number(sequence_number);

        return Self(
            encoded_timestamp | encoded_worker_number | encoded_sequence_number,
        );
    }

    pub fn get_timestamp(&self) -> Timestamp {
        let encoded_millis =
            (self.as_u64() >> TIMESTAMP_SHIFT) & TIMESTAMP_BITMASK;
        return Timestamp::from_millis_since_epoch(encoded_millis as i64);
    }

    pub fn get_worker_number(&self) -> SnowflakeWorkerNumber {
        return ((self.as_u64() >> WORKER_NUMBER_SHIFT) & WORKER_NUMBER_BITMASK)
            as SnowflakeWorkerNumber;
    }

    pub fn get_sequence_number(&self) -> SnowflakeSequenceNumber {
        return ((self.as_u64() >> SEQUENCE_NUMBER_SHIFT)
            & SEQUENCE_NUMBER_BITMASK)
            as SnowflakeSequenceNumber;
    }

    fn encode_timestamp(timestamp: Timestamp) -> u64 {
        let millis = timestamp.as_millis_since_epoch() as u64;
        return (millis & TIMESTAMP_BITMASK) << TIMESTAMP_SHIFT;
    }

    fn encode_worker_number(worker_number: SnowflakeWorkerNumber) -> u64 {
        let worker_number = worker_number as u64;
        return (worker_number & WORKER_NUMBER_BITMASK) << WORKER_NUMBER_SHIFT;
    }

    fn encode_sequence_number(sequence_number: SnowflakeSequenceNumber) -> u64 {
        let sequence_number = sequence_number as u64;
        return (sequence_number & SEQUENCE_NUMBER_BITMASK)
            << SEQUENCE_NUMBER_SHIFT;
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
