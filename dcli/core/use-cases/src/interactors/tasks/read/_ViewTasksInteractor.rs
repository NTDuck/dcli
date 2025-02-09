use crate::boundaries::tasks::ViewTasksBoundary;
use crate::boundaries::tasks::ViewTasksErrorModel;
use crate::boundaries::tasks::ViewTasksRequestModel;
use crate::boundaries::tasks::ViewTasksResponseModel;
use crate::gateways::pointers::SharedPointerHandle;
use crate::gateways::repositories::tasks::TaskRepository;
use crate::utils::pointers::SharedPointer;

pub struct ViewTasksInteractor<Handle: SharedPointerHandle> {
    task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,
}

impl<Handle: SharedPointerHandle> ViewTasksInteractor<Handle> {
    pub const fn new(task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>) -> Self {
        return Self {
            task_repository,
        };
    }
}

impl<Handle: SharedPointerHandle> ViewTasksBoundary for ViewTasksInteractor<Handle> {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel> {
        let ViewTasksRequestModel {
            pagination_request,
        } = request;

        let pagination_response = self.task_repository.unwrap()
            .show(pagination_request);

        return Ok(ViewTasksResponseModel { pagination_response });
    }
}
