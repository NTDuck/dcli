use std::fmt::Debug;

pub trait Gateway: Send + Sync + Debug {}

impl<T> Gateway for T where T: Send + Sync + Debug {}
