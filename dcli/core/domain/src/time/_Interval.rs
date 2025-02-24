use std::time::Duration;

use axiom::behaviours::NewType;
use axiom::interfaces::ddd;

#[derive(ddd::ValueObject, NewType)]
#[derive(Copy, PartialOrd, Ord, Hash)]
pub struct Interval(Duration);

impl Interval {
    pub(in crate::time) const fn fromDuration(duration: Duration) -> Self {
        return Self(duration);
    }

    pub const fn asDuration(&self) -> Duration {
        return self.0;
    }
}

impl Interval {
    pub const fn fromSeconds(seconds: u64) -> Self {
        let duration = Duration::from_secs(seconds);
        return Self::fromDuration(duration);
    }

    pub const fn fromMilliseconds(milliseconds: u64) -> Self {
        let duration = Duration::from_millis(milliseconds);
        return Self::fromDuration(duration);
    }

    pub const fn fromMicroseconds(microseconds: u64) -> Self {
        let duration = Duration::from_micros(microseconds);
        return Self::fromDuration(duration);
    }

    pub const fn fromNanoseconds(nanoseconds: u64) -> Self {
        let duration = Duration::from_nanos(nanoseconds);
        return Self::fromDuration(duration);
    }

    pub const fn asSeconds(&self) -> u64 {
        let duration = self.asDuration();
        return duration.as_secs();
    }

    pub const fn asMilliseconds(&self) -> u128 {
        let duration = self.asDuration();
        return duration.as_millis();
    }

    pub const fn asMicroseconds(&self) -> u128 {
        let duration = self.asDuration();
        return duration.as_micros();
    }

    pub const fn asNanoseconds(&self) -> u128 {
        let duration = self.asDuration();
        return duration.as_nanos();
    }

    pub fn checkedAdd(self, other: Self) -> Option<Self> {
        let duration = self.asDuration();
        let otherDuration = other.asDuration();

        return duration
            .checked_add(otherDuration)
            .map(Self::fromDuration);
    }

    pub fn checkedSub(self, other: Self) -> Option<Self> {
        let duration = self.asDuration();
        let otherDuration = other.asDuration();

        return duration
            .checked_sub(otherDuration)
            .map(Self::fromDuration);
    }

    pub fn checkedMul(self, multiplier: u32) -> Option<Self> {
        let duration = self.asDuration();

        return duration
            .checked_mul(multiplier)
            .map(Self::fromDuration);
    }

    pub fn checkedDiv(self, divisor: u32) -> Option<Self> {
        let duration = self.asDuration();

        return duration
            .checked_div(divisor)
            .map(Self::fromDuration);
    }
}
