pub trait SequenceNumberProvider {
    fn getSequenceNumber(&self) -> u16;
}
