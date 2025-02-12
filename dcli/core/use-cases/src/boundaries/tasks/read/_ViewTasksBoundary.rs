use domain::Task;

use crate::utils::dataclasses::pagination::PaginationRequest;
use crate::utils::dataclasses::pagination::PaginationResponse;

pub trait ViewTasksBoundary {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel>;
}

pub struct ViewTasksRequestModel {
    pub paginationRequest: PaginationRequest,
}

pub struct ViewTasksResponseModel {
    pub paginationResponse: PaginationResponse<Task>,
}

pub struct ViewTasksErrorModel;
