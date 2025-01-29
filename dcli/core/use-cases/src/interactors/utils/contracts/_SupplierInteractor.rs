pub trait SupplierInteractor {
    fn supply(&self) -> Result<Self::Response, Self::Error>;

    type Response;
    type Error;
}
