use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

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
    pub pagination_response: PaginationResponse<ViewTasksTaskModel>,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksTaskModel {
    pub id: u64,
    pub description: String,
    pub status: ViewTasksTaskStatusModel,
    pub created_at: String,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub enum ViewTasksTaskStatusModel {
    Pending,
    InProgress,
    Completed,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksErrorModel;
