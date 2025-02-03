use crate::boundaries::tasks::ViewTasksBoundary;
use crate::boundaries::tasks::ViewTasksErrorModel;
use crate::boundaries::tasks::ViewTasksRequestModel;
use crate::boundaries::tasks::ViewTasksResponseModel;
use crate::gateways::repositories::tasks::TaskRepository;

pub struct ViewTasksInteractor<'deps> {
    task_repository: &'deps dyn TaskRepository,
}

impl<'deps> ViewTasksBoundary for ViewTasksInteractor<'deps> {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel> {
        let ViewTasksRequestModel {
            pagination_request,
        } = request;

        return Ok(ViewTasksResponseModel {
            tasks: self.task_repository.show(pagination_request),
        });
    }
}
