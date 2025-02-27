use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

pub trait CreateTaskBoundary {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel>;
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct CreateTaskRequestModel {
    pub task_description: String,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct CreateTaskResponseModel;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub enum CreateTaskErrorModel {
    TaskDescriptionLengthUnderflow {
        actual_length: usize,
        min_length_required: usize,
    },
    TaskDescriptionLengthOverflow {
        actual_length: usize,
        max_length_allowed: usize,
    },
}
