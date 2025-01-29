#[allow(dead_code)]
pub trait RunnableInteractor {
    fn run(&self);
}

pub trait MutableRunnableInteractor {
    fn run(&mut self);
}

#[allow(dead_code)]
pub trait FallibleRunnableInteractor {
    fn run(&self) -> Result<(), Self::Error>;

    type Error;
}

#[allow(dead_code)]
pub trait FallibleMutableRunnableInteractor {
    fn run(&mut self) -> Result<(), Self::Error>;

    type Error;
}
