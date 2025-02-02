pub trait RunnableInteractor {
    fn run(&self);
}

pub trait MutableRunnableInteractor {
    fn run(&mut self);
}

pub trait FallibleRunnableInteractor {
    fn run(&self) -> Result<(), Self::Error>;

    type Error;
}

pub trait FallibleMutableRunnableInteractor {
    fn run(&mut self) -> Result<(), Self::Error>;

    type Error;
}
