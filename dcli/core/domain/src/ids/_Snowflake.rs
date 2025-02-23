use axiom::behaviours::New;
use axiom::behaviours::NewType;
use axiom::interfaces::ddd;

use crate::time::Timestamp;

#[derive(ddd::Identifier, Copy, New, NewType)]
pub struct Snowflake(u64);

pub type SnowflakeTimestamp = Timestamp;
pub type SnowflakeMachineId = u16;
pub type SnowflakeMachineSequenceNumber = u16;

pub const SnowflakeTimestampBitsCount: usize = 41;
pub const SnowflakeMachineIdBitsCount: usize = 10;
pub const SnowflakeMachineSequenceNumberBitsCount: usize = 12;
