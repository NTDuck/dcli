use std::time::SystemTime;

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
    pub paginationRequest: PaginationRequest,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksResponseModel {
    pub paginationResponse: PaginationResponse<ViewTasksTask>,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksTask {
    pub id: u128,
    pub description: String,
    pub status: ViewTasksTaskStatus,
    pub createdAt: SystemTime,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub enum ViewTasksTaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksErrorModel;
