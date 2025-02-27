use std::time::Duration;

use axiom::interfaces::ddd;

#[derive(ddd::ValueObject)]
#[derive(Copy, PartialOrd, Ord, Hash)]
pub struct Interval(Duration);

impl Interval {
    pub(in crate::time) const fn from_duration(duration: Duration) -> Self {
        return Self(duration);
    }

    pub const fn from_secs(seconds: u64) -> Self {
        let duration = Duration::from_secs(seconds);
        return Self::from_duration(duration);
    }

    pub const fn from_millis(milliseconds: u64) -> Self {
        let duration = Duration::from_millis(milliseconds);
        return Self::from_duration(duration);
    }

    pub const fn from_micros(microseconds: u64) -> Self {
        let duration = Duration::from_micros(microseconds);
        return Self::from_duration(duration);
    }

    pub const fn from_nanos(nanoseconds: u64) -> Self {
        let duration = Duration::from_nanos(nanoseconds);
        return Self::from_duration(duration);
    }

    pub const fn as_duration(&self) -> Duration {
        return self.0;
    }

    pub const fn as_secs(&self) -> u64 {
        let duration = self.as_duration();
        return duration.as_secs();
    }

    pub const fn as_millis(&self) -> u128 {
        let duration = self.as_duration();
        return duration.as_millis();
    }

    pub const fn as_micros(&self) -> u128 {
        let duration = self.as_duration();
        return duration.as_micros();
    }

    pub const fn as_nanos(&self) -> u128 {
        let duration = self.as_duration();
        return duration.as_nanos();
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        let duration = self.as_duration();
        let other_duration = other.as_duration();

        return duration
            .checked_add(other_duration)
            .map(Self::from_duration);
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        let duration = self.as_duration();
        let other_duration = other.as_duration();

        return duration
            .checked_sub(other_duration)
            .map(Self::from_duration);
    }

    pub fn checked_mul(self, multiplier: u32) -> Option<Self> {
        let duration = self.as_duration();

        return duration
            .checked_mul(multiplier)
            .map(Self::from_duration);
    }

    pub fn checked_div(self, divisor: u32) -> Option<Self> {
        let duration = self.as_duration();

        return duration
            .checked_div(divisor)
            .map(Self::from_duration);
    }
}
