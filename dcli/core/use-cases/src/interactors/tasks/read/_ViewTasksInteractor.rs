use domain::Task;

use crate::dataproviders::gateways::common::PaginationParams;
use crate::dataproviders::gateways::tasks::TaskGateway;
use crate::interactors::utils::contracts::FunctionInteractor;

pub struct ViewTasksInteractor<'deps> {
    task_gateway: &'deps dyn TaskGateway,
}

impl<'deps> FunctionInteractor for ViewTasksInteractor<'deps> {
    fn apply(&self, request: Self::Request) -> Self::Response {
        let pagination_params = PaginationParams::new(request.page_number, request.page_size);
        let tasks = self.task_gateway.show(pagination_params);
        
        return ViewTasksResponse { tasks };
    }

    type Request = ViewTasksRequest;
    type Response = ViewTasksResponse;
}

pub struct ViewTasksRequest {
    page_number: usize,
    page_size: usize,
}

pub struct ViewTasksResponse {
    tasks: Vec<Task>,
}
