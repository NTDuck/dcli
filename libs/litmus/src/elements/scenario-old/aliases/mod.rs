pub trait GivenFn<WorldImpl>: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static {}

impl<WorldImpl, T> GivenFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait WhenFn<WorldImpl>: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static {}

impl<WorldImpl, T> WhenFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ThenFn<WorldImpl>: FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static {}

impl<WorldImpl, T> ThenFn<WorldImpl> for T
where
    T: FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait World: Default {}

impl<T> World for T
where
    T: Default,
{
}
