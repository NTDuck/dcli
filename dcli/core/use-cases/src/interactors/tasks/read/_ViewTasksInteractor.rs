use std::sync::Arc;
use std::sync::RwLock;

use types::New;

use crate::boundaries::tasks::ViewTasksBoundary;
use crate::boundaries::tasks::ViewTasksErrorModel;
use crate::boundaries::tasks::ViewTasksRequestModel;
use crate::boundaries::tasks::ViewTasksResponseModel;
use crate::gateways::repositories::tasks::TaskRepository;

#[derive(New)]
pub struct ViewTasksInteractor {
    task_repository: Arc<RwLock<dyn TaskRepository>>,
}

impl ViewTasksBoundary for ViewTasksInteractor {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel> {
        let ViewTasksRequestModel {
            pagination_request,
        } = request;

        let pagination_response = self.task_repository.read().unwrap()
            .show(pagination_request);

        return Ok(ViewTasksResponseModel { pagination_response });
    }
}
