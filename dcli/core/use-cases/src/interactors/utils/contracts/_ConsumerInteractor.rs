pub trait ConsumerInteractor {
    fn consume(&self, request: Self::Request) -> Result<(), Self::Error>;

    type Request;
    type Error;
}
