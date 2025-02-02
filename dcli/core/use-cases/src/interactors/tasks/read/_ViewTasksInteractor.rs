use domain::Task;

use crate::gateways::repositories::common::PaginationParams;
use crate::gateways::repositories::tasks::TaskRepository;
use crate::utils::contracts::interactors::FunctionInteractor;

pub struct ViewTasksInteractor<'deps> {
    task_repository: &'deps dyn TaskRepository,
}

impl<'deps> FunctionInteractor for ViewTasksInteractor<'deps> {
    fn apply(&self, request: Self::Request) -> Self::Response {
        let pagination_params = PaginationParams::new(request.page_number, request.page_size);
        let tasks = self.task_repository.show(pagination_params);
        
        return ViewTasksResponseModel { tasks };
    }

    type Request = ViewTasksRequestModel;
    type Response = ViewTasksResponseModel;
}

pub struct ViewTasksRequestModel {
    pub page_number: usize,
    pub page_size: usize,
}

pub struct ViewTasksResponseModel {
    pub tasks: Vec<Task>,
}
