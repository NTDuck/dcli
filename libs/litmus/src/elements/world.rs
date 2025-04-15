pub trait World: Default + Send + Sync + 'static {}

impl<T> World for T where T: Default + Send + Sync + 'static {}
