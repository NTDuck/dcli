use domain::ids::MachineId;
use domain::ids::Snowflake;
use domain::time::Timestamp;

pub trait SnowflakeParser {
    fn getTimestampFromSnowflake(&self, snowflake: Snowflake) -> Timestamp;
    fn getMachineIdFromSnowflake(&self, snowflake: Snowflake) -> MachineId;
}
