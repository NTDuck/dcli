pub unsafe trait Concurrent: Send + Sync {}

unsafe impl<T> Concurrent for T
where T: Send + Sync {}
