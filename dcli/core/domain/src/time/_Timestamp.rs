use std::time::SystemTime;

use axiom::behaviours::NewType;
use axiom::interfaces::ddd;

use crate::time::Interval;

#[derive(ddd::ValueObject, NewType)]
#[derive(Copy, PartialOrd, Ord, Hash)]
pub struct Timestamp(SystemTime);

impl Timestamp {
    pub(in crate::time) const fn fromSystemTime(systemTime: SystemTime) -> Self {
        return Self(systemTime);
    }

    pub const fn asSystemTime(&self) -> SystemTime {
        return self.0;
    }
}

impl Timestamp {
    pub fn current() -> Self {
        let currentSystemTime = SystemTime::now();
        return Self::fromSystemTime(currentSystemTime);
    }

    pub fn intervalSince(&self, previous: Self) -> Option<Interval> {
        let systemTime = self.asSystemTime();
        let previousSystemTime = previous.asSystemTime();
        
        return systemTime
            .duration_since(previousSystemTime)
            .ok()
            .map(Interval::fromDuration);
    }

    pub fn checkedAdd(self, interval: Interval) -> Option<Self> {
        let systemTime = self.asSystemTime();
        let duration = interval.asDuration();
        
        return systemTime
            .checked_add(duration)
            .map(Self::fromSystemTime);
    }

    pub fn checkedSub(self, interval: Interval) -> Option<Self> {
        let systemTime = self.asSystemTime();
        let duration = interval.asDuration();
        
        return systemTime
            .checked_sub(duration)
            .map(Self::fromSystemTime);
    }
}
