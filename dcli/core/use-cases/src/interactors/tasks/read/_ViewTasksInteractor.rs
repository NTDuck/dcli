use crate::boundaries::tasks::ViewTasksBoundary;
use crate::boundaries::tasks::ViewTasksErrorModel;
use crate::boundaries::tasks::ViewTasksRequestModel;
use crate::boundaries::tasks::ViewTasksResponseModel;
use crate::gateways::pointers::handles::abc::PointerHandle;
use crate::gateways::pointers::SharedPointer;
use crate::gateways::repositories::tasks::TaskRepository;

pub struct ViewTasksInteractor<Handle: PointerHandle> {
    task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,
}

impl<Handle: PointerHandle> ViewTasksInteractor<Handle> {
    pub const fn new(task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>) -> Self {
        return Self {
            task_repository,
        };
    }
}

impl<Handle: PointerHandle> ViewTasksBoundary for ViewTasksInteractor<Handle> {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel> {
        let ViewTasksRequestModel {
            pagination_request,
        } = request;

        let pagination_response = self.task_repository.unwrap()
            .show_most_recent(pagination_request);

        return Ok(ViewTasksResponseModel { pagination_response });
    }
}
