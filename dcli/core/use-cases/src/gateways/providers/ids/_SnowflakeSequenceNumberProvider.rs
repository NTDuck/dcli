use domain::ids::SnowflakeSequenceNumber;

pub trait SnowflakeSequenceNumberProvider {
    fn getSequenceNumber(&self) -> SnowflakeSequenceNumber;
}
