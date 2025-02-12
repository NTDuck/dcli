use crate::boundaries::tasks::ViewTasksBoundary;
use crate::boundaries::tasks::ViewTasksErrorModel;
use crate::boundaries::tasks::ViewTasksRequestModel;
use crate::boundaries::tasks::ViewTasksResponseModel;
use crate::gateways::pointers::handles::abc::PointerHandle;
use crate::gateways::pointers::SharedPointer;
use crate::gateways::repositories::tasks::TaskRepository;

pub struct ViewTasksInteractor<Handle: PointerHandle> {
    taskRepository: SharedPointer<Box<dyn TaskRepository>, Handle>,
}

impl<Handle: PointerHandle> ViewTasksInteractor<Handle> {
    pub const fn new(taskRepository: SharedPointer<Box<dyn TaskRepository>, Handle>) -> Self {
        return Self {
            taskRepository,
        };
    }
}

impl<Handle: PointerHandle> ViewTasksBoundary for ViewTasksInteractor<Handle> {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel> {
        let ViewTasksRequestModel {
            paginationRequest,
        } = request;

        let paginationResponse = self.taskRepository.unwrap()
            .showOrderedByCreatedAtDesc(paginationRequest);

        return Ok(ViewTasksResponseModel { paginationResponse });
    }
}
