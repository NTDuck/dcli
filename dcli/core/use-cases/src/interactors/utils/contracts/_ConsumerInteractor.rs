pub trait ConsumerInteractor {
    fn consume(&self, request: Self::Request) -> Result<(), Self::Exception>;

    type Request;
    type Exception;
}
