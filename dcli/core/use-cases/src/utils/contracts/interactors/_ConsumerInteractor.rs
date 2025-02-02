pub trait ConsumerInteractor {
    fn consume(&self, request: Self::Request);

    type Request;
}

pub trait MutableConsumerInteractor {
    fn consume(&mut self, request: Self::Request);

    type Request;
}

pub trait FallibleConsumerInteractor {
    fn consume(&self, request: Self::Request) -> Result<(), Self::Error>;

    type Request;
    type Error;
}

pub trait FallibleMutableConsumerInteractor {
    fn consume(&mut self, request: Self::Request) -> Result<(), Self::Error>;

    type Request;
    type Error;
}
