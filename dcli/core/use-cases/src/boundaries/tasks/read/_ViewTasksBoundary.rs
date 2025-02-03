use domain::Task;

use crate::utils::dataclasses::pagination::PaginationRequest;
use crate::utils::dataclasses::pagination::PaginationResult;

pub trait ViewTasksBoundary {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel>;
}

pub struct ViewTasksRequestModel {
    pub pagination_request: PaginationRequest,
}

pub struct ViewTasksResponseModel {
    pub tasks: PaginationResult<Task>,
}

pub struct ViewTasksErrorModel;
