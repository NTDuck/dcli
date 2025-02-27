use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;

use axiom::behaviours::New;
use domain::ids::SnowflakeWorkerNumber;
use domain::ids::SnowflakeSequenceNumber;
use use_cases::gateways::providers::ids::SnowflakeProvider;

#[derive(New)]
pub struct CentralizedSnowflakeProvider {
    workerNumber: SnowflakeWorkerNumber,
    sequenceNumber: AtomicU16,
}

impl CentralizedSnowflakeProvider {
    fn computeAndAssignNextSequenceNumber(&self) -> SnowflakeSequenceNumber {
        return self.sequenceNumber.fetch_update(
            Ordering::Relaxed,
            Ordering::Relaxed,
            |sequenceNumber| Some((sequenceNumber) + 1 & 0xfff))
        .unwrap();
    }
}

impl SnowflakeProvider for CentralizedSnowflakeProvider {
    fn getWorkerNumber(&self) -> SnowflakeWorkerNumber {
        return self.workerNumber;
    }

    fn getSequenceNumber(&self) -> SnowflakeSequenceNumber {
        return self.computeAndAssignNextSequenceNumber();
    }
}
