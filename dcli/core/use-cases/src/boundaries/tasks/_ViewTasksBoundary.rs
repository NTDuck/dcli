use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

use crate::boundaries::tasks::common::TaskModel;
use crate::utils::dataclasses::pagination::PaginationRequest;
use crate::utils::dataclasses::pagination::PaginationResponse;

pub trait ViewTasksBoundary {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel>;
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksRequestModel {
    pub pagination_request: PaginationRequest,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksResponseModel {
    pub pagination_response: PaginationResponse<TaskModel>,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksErrorModel;
