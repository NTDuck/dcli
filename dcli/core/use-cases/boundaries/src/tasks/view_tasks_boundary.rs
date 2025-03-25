use axiom::interfaces::DataTransferObjectWithoutSerde;
use models::pagination::PaginationRequest;
use models::pagination::PaginationResponse;
use models::tasks::TaskModel;
use serde::Deserialize;
use serde::Serialize;

pub trait ViewTasksInputBoundary {
    fn accept(&self, request: ViewTasksRequestModel);
}

pub trait ViewTasksOutputBoundary {
    fn accept(&self, response: ViewTasksResponseModel);
}

pub trait ViewTasksBoundary {
    fn apply(&self, request: ViewTasksRequestModel) -> ViewTasksResponseModel;
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
