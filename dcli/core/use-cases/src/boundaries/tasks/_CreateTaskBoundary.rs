use axiom::interfaces::SerdelessDataTransferObject;
use serde::Deserialize;
use serde::Serialize;

pub trait CreateTaskBoundary {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel>;
}

#[derive(SerdelessDataTransferObject, Serialize, Deserialize)]
pub struct CreateTaskRequestModel {
    pub taskDescription: String,
}

#[derive(SerdelessDataTransferObject, Serialize, Deserialize)]
pub struct CreateTaskResponseModel;

#[derive(SerdelessDataTransferObject, Serialize, Deserialize)]
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
