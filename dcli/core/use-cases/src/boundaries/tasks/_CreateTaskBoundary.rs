use axiom::interfaces::deriveDataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

pub trait CreateTaskBoundary {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel>;
}

#[derive(deriveDataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct CreateTaskRequestModel {
    pub taskDescription: String,
}

#[derive(deriveDataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct CreateTaskResponseModel;

#[derive(deriveDataTransferObjectWithoutSerde, Serialize, Deserialize)]
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
