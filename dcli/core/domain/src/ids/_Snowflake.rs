use axiom::behaviours::NewType;
use axiom::interfaces::ddd;

use crate::time::Timestamp;

/// ### See also:
/// - [Twitter's announcement](https://blog.x.com/engineering/en_us/a/2010/announcing-snowflake)
/// - [Comparison to UUID](https://softwaremind.com/blog/the-unique-features-of-snowflake-id-and-its-comparison-to-uuid/)
#[derive(ddd::Identifier, Copy, NewType)]
pub struct Snowflake(u64);

impl Snowflake {
    pub fn new(
        timestamp: Timestamp,
        workerNumber: u16,
        sequenceNumber: u16,
    ) -> Self {
        todo!()
    }

    pub fn getTimestamp(&self) -> Timestamp {
        todo!()
    }

    pub fn getWorkerNumber(&self) -> u16 {
        todo!()
    }

    pub fn getSequenceNumber(&self) -> u16 {
        todo!()
    }

    fn encodeTimestamp(&self, timestamp: Timestamp) -> u64 {
        todo!()
    }

    fn encodeWorkerNumber(&self, workerNumber: u16) -> u64 {
        todo!()
    }

    fn encodeSequenceNumber(&self, sequenceNumber: u16) -> u64 {
        todo!()
    }
}

const TimestampBitmask: u64 = 0x1ffffffffff;
const MachineIdBitmask: u64 = 0x3ff;
const MachineSequenceNumberBitmask: u64 = 0x0fff;

const TimestampShift: usize = SnowflakeBits - ReservedBits - TimestampBits;
const WorkerNumberShift: usize = TimestampShift - MachineIdBits;
const SequenceNumberShift: usize = WorkerNumberShift - MachineSequenceNumberBits;

const SnowflakeBits: usize = 64;
const ReservedBits: usize = 1;
const TimestampBits: usize = 41;
const MachineIdBits: usize = 10;
const MachineSequenceNumberBits: usize = 12;
