use std::time::Instant;

use axiom::interfaces::SerdelessDataTransferObject;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::dataclasses::pagination::PaginationRequest;
use crate::utils::dataclasses::pagination::PaginationResponse;

pub trait ViewTasksBoundary {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel>;
}

#[derive(SerdelessDataTransferObject, Serialize, Deserialize)]
pub struct ViewTasksRequestModel {
    pub paginationRequest: PaginationRequest,
}

#[derive(SerdelessDataTransferObject, Serialize, Deserialize)]
pub struct ViewTasksResponseModel {
    pub paginationResponse: PaginationResponse<Task>,
}

#[derive(SerdelessDataTransferObject, Serialize, Deserialize)]
pub struct Task {
    pub id: u128,
    pub description: String,
    pub status: TaskStatus,
    pub createdAt: Instant,
}

#[derive(SerdelessDataTransferObject, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(SerdelessDataTransferObject, Serialize, Deserialize)]
pub struct ViewTasksErrorModel;
