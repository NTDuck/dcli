pub trait Boundary: Send + Sync {}

impl<T> Boundary for T
where
    T: Send + Sync,
{}
