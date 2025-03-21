use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::dataclasses::pagination::PaginationRequest;
use crate::utils::dataclasses::pagination::PaginationResponse;
use crate::utils::dataclasses::tasks::TaskModel;

pub trait ViewTasksInputBoundary {
    fn accept(&self, request: ViewTasksRequestModel);
}

pub trait ViewTasksOutputBoundary {
    fn accept(&self, response: ViewTasksResponseModel);
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksRequestModel {
    pub pagination_request: PaginationRequest,
}

pub type ViewTasksResponseModel = Result<ViewTasksOkResponseModel, ViewTasksErrResponseModel>;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksOkResponseModel {
    pub pagination_response: PaginationResponse<TaskModel>,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksErrResponseModel;
