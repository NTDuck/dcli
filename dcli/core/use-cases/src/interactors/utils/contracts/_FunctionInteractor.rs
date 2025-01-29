#[allow(dead_code)]
pub trait FunctionInteractor {
    fn apply(&self, request: Self::Request) -> Self::Response;
    
    type Request;
    type Response;
}

#[allow(dead_code)]
pub trait MutableFunctionInteractor {
    fn apply(&mut self, request: Self::Request) -> Self::Response;
    
    type Request;
    type Response;
}

pub trait FallibleFunctionInteractor {
    fn apply(&self, request: Self::Request) -> Result<Self::Response, Self::Error>;

    type Request;
    type Response;
    type Error;
}

#[allow(dead_code)]
pub trait FallibleMutableFunctionInteractor {
    fn apply(&mut self, request: Self::Request) -> Result<Self::Response, Self::Error>;

    type Request;
    type Response;
    type Error;
}
