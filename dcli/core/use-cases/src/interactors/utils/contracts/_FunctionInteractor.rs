pub trait FunctionInteractor {
    fn apply(&self, request: Self::Request) -> Result<Self::Response, Self::Exception>;

    type Request;
    type Response;
    type Exception;
}
