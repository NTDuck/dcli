use std::str::FromStr;

use domain::ids::Snowflake;

use crate::parameters::time::TimestampParameter;

#[derive(cucumber::Parameter)]
#[param(name = "snowflake", regex = r"^\d+$")]
pub struct SnowflakeParameter(Snowflake);

impl FromStr for SnowflakeParameter {
    type Err = <TimestampParameter as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timestamp = TimestampParameter::from_str(s)?.into();
        let snowflake = Snowflake::new(timestamp, 0, 0);
        Ok(Self(snowflake))
    }
}

impl From<SnowflakeParameter> for Snowflake {
    fn from(parameter: SnowflakeParameter) -> Self {
        parameter.0
    }
}
