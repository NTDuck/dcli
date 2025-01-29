#[allow(dead_code)]
pub trait SupplierInteractor {
    fn supply(&self) -> Self::Response;

    type Response;
}

#[allow(dead_code)]
pub trait MutableSupplierInteractor {
    fn supply(&mut self) -> Self::Response;

    type Response;
}

#[allow(dead_code)]
pub trait FallibleSupplierInteractor {
    fn supply(&self) -> Result<Self::Response, Self::Error>;

    type Response;
    type Error;
}

#[allow(dead_code)]
pub trait FallibleMutableSupplierInteractor {
    fn supply(&mut self) -> Result<Self::Response, Self::Error>;

    type Response;
    type Error;
}
