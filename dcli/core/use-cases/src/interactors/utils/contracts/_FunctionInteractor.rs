pub trait FunctionInteractor {
    fn apply(&self, request: Self::Request) -> Result<Self::Response, Self::Error>;

    type Request;
    type Response;
    type Error;
}
