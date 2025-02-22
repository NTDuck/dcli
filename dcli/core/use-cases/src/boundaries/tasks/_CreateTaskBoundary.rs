use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

pub trait CreateTaskBoundary {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel>;
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct CreateTaskRequestModel {
    pub taskDescription: String,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct CreateTaskResponseModel;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub enum CreateTaskErrorModel {
    TaskDescriptionLengthUnderflow {
        actualLength: usize,
        minLengthRequired: usize,
    },
    TaskDescriptionLengthOverflow {
        actualLength: usize,
        maxLengthAllowed: usize,
    },
}
