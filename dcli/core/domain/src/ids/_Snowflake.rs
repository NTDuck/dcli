use axiom::interfaces::ddd;

use crate::time::Epoch;
use crate::time::Interval;
use crate::time::Timestamp;

/// ### See also:
/// - [Twitter's announcement](https://blog.x.com/engineering/en_us/a/2010/announcing-snowflake)
/// - [Comparison to UUID](https://softwaremind.com/blog/the-unique-features-of-snowflake-id-and-its-comparison-to-uuid/)
#[derive(ddd::Identifier)]
#[derive(Copy, PartialOrd, Ord)]
pub struct Snowflake(u64);

impl Snowflake {
    pub fn new(
        timestamp: Timestamp,
        workerNumber: u16,
        sequenceNumber: u16,
    ) -> Self {
        let encodedTimestamp = Self::encodeTimestamp(timestamp);
        let encodedWorkerNumber = Self::encodeWorkerNumber(workerNumber);
        let encodedSequenceNumber = Self::encodeSequenceNumber(sequenceNumber);

        return Self(encodedTimestamp | encodedWorkerNumber | encodedSequenceNumber);
    }

    pub fn getTimestamp(&self) -> Timestamp {
        let encodedSnowflake = self.asEncodedSnowflake();
        let encodedMilliseconds = (encodedSnowflake >> TimestampShift) & TimestampBitmask;
        let encodedInterval = Interval::fromMilliseconds(encodedMilliseconds);
        
        return Epoch
            .checkedAdd(encodedInterval)
            .expect("Timestamp overflow");
    }

    pub fn getWorkerNumber(&self) -> u16 {
        let encodedSnowflake = self.asEncodedSnowflake();
        return ((encodedSnowflake >> WorkerNumberShift) & WorkerNumberBitmask) as u16;
    }

    pub fn getSequenceNumber(&self) -> u16 {
        let encodedSnowflake = self.asEncodedSnowflake();
        return ((encodedSnowflake >> SequenceNumberShift) & SequenceNumberBitmask) as u16;
    }

    fn encodeTimestamp(timestamp: Timestamp) -> u64 {
        let interval = timestamp.intervalSince(Epoch)
            .expect("Timestamp earlier than Epoch");
        let milliseconds = interval.asMilliseconds() as u64;
        
        return (milliseconds & TimestampBitmask) << TimestampShift;
    }

    fn encodeWorkerNumber(workerNumber: u16) -> u64 {
        let workerNumber = workerNumber as u64;
        return (workerNumber & WorkerNumberBitmask) << WorkerNumberShift;
    }

    fn encodeSequenceNumber(sequenceNumber: u16) -> u64 {
        let sequenceNumber = sequenceNumber as u64;
        return (sequenceNumber & SequenceNumberBitmask) << SequenceNumberShift;
    }

    fn asEncodedSnowflake(&self) -> u64 {
        return self.0;
    }
}

const SnowflakeBits: usize = 64;
const ReservedBits: usize = 1;
const TimestampBits: usize = 41;
const WorkerNumberBits: usize = 10;
const SequenceNumberBits: usize = 12;

const TimestampBitmask: u64 = 0x1ffffffffff;
const WorkerNumberBitmask: u64 = 0x3ff;
const SequenceNumberBitmask: u64 = 0x0fff;

const TimestampShift: usize = SnowflakeBits - ReservedBits - TimestampBits;
const WorkerNumberShift: usize = TimestampShift - WorkerNumberBits;
const SequenceNumberShift: usize = WorkerNumberShift - SequenceNumberBits;
