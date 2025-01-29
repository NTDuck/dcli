use domain::Task;

use crate::dataproviders::gateways::common::PaginationParams;
use crate::dataproviders::gateways::tasks::TaskGateway;
use crate::interactors::utils::contracts::FallibleFunctionInteractor;

pub struct ViewTasksInteractor<'deps> {
    task_gateway: &'deps dyn TaskGateway,
}

impl<'deps> FallibleFunctionInteractor for ViewTasksInteractor<'deps> {
    fn apply(&self, request: Self::Request) -> Result<Self::Response, Self::Error> {
        let pagination_params = PaginationParams::new(request.page_number, request.page_size);
        let Some(tasks) = self.task_gateway.show(pagination_params) else {
            return Err(ViewTasksError::PaginationInvalid);
        };

        return Ok(ViewTasksResponse { tasks });
    }

    type Request = ViewTasksRequest;
    type Response = ViewTasksResponse;
    type Error = ViewTasksError;
}

pub struct ViewTasksRequest {
    page_number: usize,
    page_size: usize,
}

pub struct ViewTasksResponse {
    tasks: Vec<Task>,
}

pub enum ViewTasksError {
    PaginationInvalid,
}