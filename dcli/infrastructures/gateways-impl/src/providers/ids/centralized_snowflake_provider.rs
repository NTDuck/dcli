use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;

use axiom::behaviours::New;
use domain::ids::SnowflakeSequenceNumber;
use domain::ids::SnowflakeWorkerNumber;
use use_cases::gateways::providers::ids::SnowflakeProvider;

#[derive(New, Default)]
pub struct CentralizedSnowflakeProvider {
    worker_number: SnowflakeWorkerNumber,
    sequence_number: AtomicU16,
}

impl CentralizedSnowflakeProvider {
    fn compute_and_assign_next_sequence_number(
        &self,
    ) -> SnowflakeSequenceNumber {
        return self
            .sequence_number
            .fetch_update(
                Ordering::Relaxed,
                Ordering::Relaxed,
                |sequence_number| Some((sequence_number) + 1 & 0xfff),
            )
            .unwrap();
    }
}

impl SnowflakeProvider for CentralizedSnowflakeProvider {
    fn get_worker_number(&self) -> SnowflakeWorkerNumber {
        return self.worker_number;
    }

    fn get_sequence_number(&self) -> SnowflakeSequenceNumber {
        return self.compute_and_assign_next_sequence_number();
    }
}
