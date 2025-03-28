pub trait Gateway: Send + Sync {}

impl<T> Gateway for T
where
    T: Send + Sync
{}
