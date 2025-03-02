use std::ops::Add;
use std::ops::Sub;

use axiom::interfaces::ddd;

use crate::time::Interval;

#[derive(ddd::ValueObject)]
#[derive(Copy, PartialOrd, Ord, Hash)]
pub struct Timestamp(i64);

impl Timestamp {
    pub const fn from_millis_since_epoch(millis: i64) -> Self {
        return Self(millis);
    }

    pub const fn as_millis_since_epoch(&self) -> i64 {
        return self.0;
    }

    pub const EPOCH: Self = Self::from_millis_since_epoch(0);
}

impl Add<Interval> for Timestamp {
    type Output = Self;

    fn add(self, interval: Interval) -> Self::Output {
        let millis_since_epoch = self.as_millis_since_epoch()
            .saturating_add(interval.as_millis());
        return Self::from_millis_since_epoch(millis_since_epoch);
    }
}

impl Sub<Interval> for Timestamp {
    type Output = Self;

    fn sub(self, interval: Interval) -> Self::Output {
        let millis_since_epoch = self.as_millis_since_epoch()
            .saturating_sub(interval.as_millis());
        return Self::from_millis_since_epoch(millis_since_epoch);
    }
}

impl Sub<Self> for Timestamp {
    type Output = Interval;

    fn sub(self, other: Self) -> Self::Output {
        let millis = self.as_millis_since_epoch()
            .saturating_sub(other.as_millis_since_epoch());
        return Interval::from_millis(millis);
    }
}
