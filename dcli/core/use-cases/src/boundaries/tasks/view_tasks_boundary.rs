use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::dataclasses::pagination::PaginationRequest;
use crate::utils::dataclasses::pagination::PaginationResponse;
use crate::utils::dataclasses::tasks::TaskModel;
use crate::utils::interfaces::Boundary;

pub trait ViewTasksBoundary: Boundary {
    fn apply(&self, request: ViewTasksRequestModel) -> ViewTasksResponseModel;
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksRequestModel {
    pub pagination_request: PaginationRequest,
}

pub type ViewTasksResponseModel =
    Result<ViewTasksOkResponseModel, ViewTasksErrResponseModel>;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksOkResponseModel {
    pub pagination_response: PaginationResponse<TaskModel>,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksErrResponseModel;
