pub trait SupplierInteractor {
    fn supply(&self) -> Result<Self::Response, Self::Exception>;

    type Response;
    type Exception;
}
