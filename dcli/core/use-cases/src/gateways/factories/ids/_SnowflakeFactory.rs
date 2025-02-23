use domain::ids::MachineId;
use domain::ids::Snowflake;
use domain::time::Timestamp;

pub trait SnowflakeFactory {
    fn newSnowflake(&self, timestamp: Timestamp, machineId: MachineId) -> Snowflake;
}
