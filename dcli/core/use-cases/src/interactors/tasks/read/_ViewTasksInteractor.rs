use crate::boundaries::tasks::ViewTasksBoundary;
use crate::boundaries::tasks::ViewTasksErrorModel;
use crate::boundaries::tasks::ViewTasksRequestModel;
use crate::boundaries::tasks::ViewTasksResponseModel;
use crate::gateways::repositories::tasks::TaskRepository;
use crate::utils::pointers::SharedPointer;

pub struct ViewTasksInteractor {
    task_repository: SharedPointer<Box<dyn TaskRepository>>,
}

impl ViewTasksInteractor {
    pub const fn new(task_repository: SharedPointer<Box<dyn TaskRepository>>) -> Self {
        return Self {
            task_repository,
        };
    }
}

impl ViewTasksBoundary for ViewTasksInteractor {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel> {
        let ViewTasksRequestModel {
            pagination_request,
        } = request;

        let pagination_response = self.task_repository.unwrap()
            .show(pagination_request);

        return Ok(ViewTasksResponseModel { pagination_response });
    }
}
