use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use axiom::interfaces::ddd;

use crate::time::Interval;

#[derive(ddd::ValueObject)]
#[derive(Copy, PartialOrd, Ord, Hash)]
pub struct Timestamp(SystemTime);

impl Timestamp {
    pub const fn from_system_time(systemTime: SystemTime) -> Self {
        return Self(systemTime);
    }

    pub const fn as_system_time(&self) -> SystemTime {
        return self.0;
    }
}

impl Timestamp {
    pub fn computer_interval_since(&self, previous: Self) -> Option<Interval> {
        let system_time = self.as_system_time();
        let previous_system_time = previous.as_system_time();
        
        return system_time
            .duration_since(previous_system_time)
            .ok()
            .map(Interval::from_duration);
    }

    pub fn checked_add(self, interval: Interval) -> Option<Self> {
        let system_time = self.as_system_time();
        let duration = interval.as_duration();
        
        return system_time
            .checked_add(duration)
            .map(Self::from_system_time);
    }

    pub fn checked_sub(self, interval: Interval) -> Option<Self> {
        let system_time = self.as_system_time();
        let duration = interval.as_duration();
        
        return system_time
            .checked_sub(duration)
            .map(Self::from_system_time);
    }
}

pub const EPOCH: Timestamp = Timestamp::from_system_time(UNIX_EPOCH);
