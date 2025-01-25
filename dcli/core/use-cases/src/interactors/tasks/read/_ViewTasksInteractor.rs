pub mod ViewTasks {
    use domain::Task;
    
    use crate::interactors::utils;
    use crate::dataproviders::gateways::tasks::TaskGateway;

    pub struct Interactor<'i> {
        task_gateway: &'i dyn TaskGateway,
    }

    impl<'i> Interactor<'i> {
        pub fn new(task_gateway: &'i dyn TaskGateway) -> Self {
            return Self {
                task_gateway,
            };
        }
    }

    impl<'i> utils::contracts::FunctionInteractor for Interactor<'i> {
        fn apply(&self, request: Self::Request) -> Result<Self::Response, Self::Exception> {
            let tasks = self.task_gateway.show(request.offset, request.limit);
            
            if tasks.is_none() {
                return Err(Exception::PaginationInvalid);
            }

            let tasks = tasks.unwrap();

            if tasks.is_empty() {
                return Err(Exception::TodolistEmptyException);
            }

            let response = Response { tasks };
            return Ok(response);
        }

        type Request = Request;
        type Response = Response;
        type Exception = Exception;
    }

    pub struct Request {
        offset: usize,
        limit: usize,
    }
    
    pub struct Response {
        tasks: Vec<Task>,
    }
    
    pub enum Exception {
        PaginationInvalid,
        TodolistEmptyException,
    }
}
