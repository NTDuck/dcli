pub trait CreateTaskBoundary {
    fn apply(&mut self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel>;
}

pub struct CreateTaskRequestModel {
    pub task_description: String,
}

pub struct CreateTaskResponseModel;

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
