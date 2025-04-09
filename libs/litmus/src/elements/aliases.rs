pub trait GivenStepFn<WorldImpl>: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static {}

impl<WorldImpl, T> GivenStepFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait WhenStepFn<WorldImpl>: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static {}

impl<WorldImpl, T> WhenStepFn<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ThenStepFn<WorldImpl>: FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static {}

impl<WorldImpl, T> ThenStepFn<WorldImpl> for T
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
