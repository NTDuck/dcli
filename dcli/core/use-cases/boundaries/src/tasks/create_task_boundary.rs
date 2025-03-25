use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

pub trait CreateTaskInputBoundary {
    fn accept(&self, request: CreateTaskRequestModel);
}

pub trait CreateTaskOutputBoundary {
    fn accept(&self, response: CreateTaskResponseModel);
}

pub trait CreateTaskBoundary {
    fn apply(&self, request: CreateTaskRequestModel) -> CreateTaskResponseModel;
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct CreateTaskRequestModel {
    pub task_description: String,
}

pub type CreateTaskResponseModel = Result<CreateTaskOkResponseModel, CreateTaskErrResponseModel>;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct CreateTaskOkResponseModel;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub enum CreateTaskErrResponseModel {
    TaskDescriptionLengthUnderflow {
        actual_length: usize,
        min_length_required: usize,
    },
    TaskDescriptionLengthOverflow {
        actual_length: usize,
        max_length_allowed: usize,
    },
}
