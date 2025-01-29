pub trait SupplierInteractor {
    fn supply(&self) -> Self::Response;

    type Response;
}

pub trait FallibleSupplierInteractor {
    fn supply(&self) -> Result<Self::Response, Self::Error>;

    type Response;
    type Error;
}
