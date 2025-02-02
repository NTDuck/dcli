use domain::Task;

use crate::dataclasses::pagination::PaginationResult;
use crate::gateways::repositories::tasks::TaskRepository;
use crate::utils::contracts::interactors::FunctionInteractor;
use crate::utils::dataclasses::pagination::PaginationRequest;

pub struct ViewTasksInteractor<'deps> {
    task_repository: &'deps dyn TaskRepository,
}

impl<'deps> FunctionInteractor for ViewTasksInteractor<'deps> {
    fn apply(&self, request: Self::Request) -> Self::Response {
        let tasks = self.task_repository.show(request.pagination_request);
        
        return ViewTasksResponseModel { tasks };
    }

    type Request = ViewTasksRequestModel;
    type Response = ViewTasksResponseModel;
}

pub struct ViewTasksRequestModel {
    pub pagination_request: PaginationRequest,
}

pub struct ViewTasksResponseModel {
    pub tasks: PaginationResult<Task>,
}
