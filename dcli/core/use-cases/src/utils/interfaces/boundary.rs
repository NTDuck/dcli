pub trait UseCaseBoundary: Send + Sync;

impl<T> UseCaseBoundary for T
where
    T: Send + Sync,
{}
