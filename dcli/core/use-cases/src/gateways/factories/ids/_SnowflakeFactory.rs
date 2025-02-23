use domain::utils::dataclasses::ids::Snowflake;
use domain::utils::dataclasses::ids::SnowflakeMachineId;
use domain::utils::dataclasses::ids::SnowflakeMachineSequenceNumber;
use domain::utils::dataclasses::ids::SnowflakeTimestamp;
use domain::utils::dataclasses::time::Timestamp;
use domain::utils::dataclasses::time::UnixEpoch;

pub trait SnowflakeFactory {
    const Epoch: Timestamp = UnixEpoch;

    fn newSnowflake(&self,
        timestamp: SnowflakeTimestamp,
        machineId: SnowflakeMachineId,
        machineSequenceNumber: SnowflakeMachineSequenceNumber,
    ) -> Snowflake;

    fn getTimestampFromSnowflake(&self, snowflake: Snowflake) -> SnowflakeTimestamp;
    fn getMachineIdFromSnowflake(&self, snowflake: Snowflake) -> SnowflakeMachineId;
    fn getMachineSequenceNumberFromSnowflake(&self, snowflake: Snowflake) -> SnowflakeMachineSequenceNumber;
}
