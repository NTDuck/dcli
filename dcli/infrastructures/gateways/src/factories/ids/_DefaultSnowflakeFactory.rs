use std::ops::Deref;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;

use domain::ids::MachineId;
use domain::ids::Snowflake;
use domain::time::Epoch;
use domain::time::Timestamp;
use use_cases::gateways::factories::ids::SnowflakeFactory;

pub struct DefaultSnowflakeFactory {
    machineSequenceNumber: AtomicU16,
}

impl SnowflakeFactory for DefaultSnowflakeFactory {
    fn newSnowflake(&self, timestamp: Timestamp, machineId: MachineId) -> Snowflake {
        let machineSequenceNumber = self.computeNextSequenceNumber();
        let snowflake = self.encodeSnowflake(timestamp, machineId, machineSequenceNumber);
        return Snowflake::new(snowflake);
    }
}

impl DefaultSnowflakeFactory {
    pub fn new() -> Self {
        return Self {
            machineSequenceNumber: AtomicU16::default(),
        };
    }

    fn computeNextSequenceNumber(&self) -> u16 {
        return self.machineSequenceNumber
            .fetch_add(1, Ordering::SeqCst)
            & (MachineSequenceNumberBitmask as u16);
    }

    fn encodeSnowflake(&self, timestamp: Timestamp, machineId: MachineId, machineSequenceNumber: u16) -> u64 {
        return self.encodeTimestamp(timestamp)
            | self.encodeMachineId(machineId)
            | self.encodeMachineSequenceNumber(machineSequenceNumber);
    }

    fn encodeTimestamp(&self, timestamp: Timestamp) -> u64 {
        let epoch = Epoch.deref().clone();
        let durationSinceEpoch = timestamp.deref()
            .duration_since(epoch)
            .unwrap()
            .as_millis();
        return (durationSinceEpoch as u64 & TimestampBitmask) << TimestampShift;
    }

    fn encodeMachineId(&self, machineId: MachineId) -> u64 {
        return (machineId.deref().clone() as u64 & MachineIdBitmask) << MachineIdShift;
    }

    fn encodeMachineSequenceNumber(&self, machineSequenceNumber: u16) -> u64 {
        return (machineSequenceNumber as u64 & MachineSequenceNumberBitmask) << MachineSequenceNumberShift;
    }
}

const TimestampBitmask: u64 = 0x1ffffffffff;
const MachineIdBitmask: u64 = 0x3ff;
const MachineSequenceNumberBitmask: u64 = 0x0fff;

const TimestampShift: usize = SnowflakeBits - ReservedBits - TimestampBits;
const MachineIdShift: usize = TimestampShift - MachineIdBits;
const MachineSequenceNumberShift: usize = MachineIdShift - MachineSequenceNumberBits;

const SnowflakeBits: usize = 64;
const ReservedBits: usize = 1;
const TimestampBits: usize = 41;
const MachineIdBits: usize = 10;
const MachineSequenceNumberBits: usize = 12;
