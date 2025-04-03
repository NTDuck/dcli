use std::num::ParseIntError;
use std::str::FromStr;

use domain::time::Timestamp;

#[derive(cucumber::Parameter)]
#[param(name = "timestamp", regex = r"^\d+$")]
pub struct TimestampParameter(Timestamp);

impl FromStr for TimestampParameter {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let millis = s.parse()?;
        let timestamp = Timestamp::from_millis_since_epoch(millis);
        Ok(Self(timestamp))
    }
}

impl From<TimestampParameter> for Timestamp {
    fn from(parameter: TimestampParameter) -> Self {
        parameter.0
    }
}
