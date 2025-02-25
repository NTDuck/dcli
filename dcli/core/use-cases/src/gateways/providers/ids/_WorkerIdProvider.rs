use domain::ids::WorkerId;

pub trait WorkerIdProvider {
    fn getWorkerId(&self) -> &WorkerId;
}
