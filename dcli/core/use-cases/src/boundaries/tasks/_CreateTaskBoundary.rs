use axiom::interfaces::DataTransferObject;
use serde::Deserialize;
use serde::Serialize;

pub trait CreateTaskBoundary {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel>;
}

#[derive(DataTransferObject, Serialize, Deserialize)]
pub struct CreateTaskRequestModel {
    pub taskDescription: String,
}

#[derive(DataTransferObject, Serialize, Deserialize)]
pub struct CreateTaskResponseModel;

#[derive(DataTransferObject, Serialize, Deserialize)]
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
