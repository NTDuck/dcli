pub trait WorkerNumberProvider {
    fn getWorkerNumber(&self) -> u16;
}
