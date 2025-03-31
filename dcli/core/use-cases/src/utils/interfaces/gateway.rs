pub trait UseCaseGateway: Send + Sync;

impl<T> UseCaseGateway for T
where
    T: Send + Sync,
{}
