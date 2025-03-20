use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;

use axiom::interfaces::ddd;

#[derive(ddd::ValueObject)]
#[derive(Copy, PartialOrd, Ord, Hash)]
pub struct Interval(i64);

impl Interval {
    pub const fn from_millis(millis: i64) -> Self {
        return Self(millis);
    }

    pub const fn as_millis(&self) -> i64 {
        return self.0;
    }
}

impl Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let millis = self.as_millis().neg();
        return Self::from_millis(millis);
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, interval: Interval) -> Self::Output {
        let millis = self.as_millis()
            .saturating_add(interval.as_millis());
        return Self::from_millis(millis);
    }
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, interval: Interval) -> Self::Output {
        let millis = self.as_millis()
            .saturating_sub(interval.as_millis());
        return Self::from_millis(millis);
    }
}
