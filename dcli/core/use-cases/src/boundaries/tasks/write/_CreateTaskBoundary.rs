pub trait CreateTaskBoundary {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel>;
}

pub struct CreateTaskRequestModel {
    pub taskDescription: String,
}

pub struct CreateTaskResponseModel;

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
