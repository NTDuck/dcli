pub trait FunctionInteractor {
    fn apply(&self, request: Self::Request) -> Self::Response;
    
    type Request;
    type Response;
}

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

pub trait FallibleMutableFunctionInteractor {
    fn apply(&mut self, request: Self::Request) -> Result<Self::Response, Self::Error>;

    type Request;
    type Response;
    type Error;
}
