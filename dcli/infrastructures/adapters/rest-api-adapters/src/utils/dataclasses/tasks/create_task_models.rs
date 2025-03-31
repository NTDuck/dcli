use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;
use use_cases::boundaries::tasks::CreateTaskErrResponseModel;
use use_cases::boundaries::tasks::CreateTaskOkResponseModel;
use use_cases::boundaries::tasks::CreateTaskRequestModel;
use use_cases::boundaries::tasks::CreateTaskResponseModel;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct CreateTaskRequestObject {
    pub task_description: String,
}

#[cfg(feature = "client")]
impl From<CreateTaskRequestModel> for CreateTaskRequestObject {
    fn from(response: CreateTaskRequestModel) -> Self {
        return Self {
            task_description: response.task_description,
        };
    }
}

#[cfg(feature = "server")]
impl Into<CreateTaskRequestModel> for CreateTaskRequestObject {
    fn into(self) -> CreateTaskRequestModel {
        return CreateTaskRequestModel {
            task_description: self.task_description,
        };
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTaskViewModel {
    Ok(CreateTaskOkViewModel),
    Err(CreateTaskErrViewModel),
}

impl From<CreateTaskResponseModel> for CreateTaskViewModel {
    fn from(response: CreateTaskResponseModel) -> Self {
        let response = response
            .map(CreateTaskOkViewModel::from)
            .map_err(CreateTaskErrViewModel::from);

        match response {
            Ok(response) => Self::Ok(response),
            Err(response) => Self::Err(response),
        }
    }
}

impl Into<Result<CreateTaskOkViewModel, CreateTaskErrViewModel>> for CreateTaskViewModel {
    fn into(self) -> Result<CreateTaskOkViewModel, CreateTaskErrViewModel> {
        match self {
            Self::Ok(response) => Ok(response),
            Self::Err(response) => Err(response),
        }
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct CreateTaskOkViewModel;

impl From<CreateTaskOkResponseModel> for CreateTaskOkViewModel {
    fn from(_: CreateTaskOkResponseModel) -> Self {
        return Self;
    }
}

impl Into<CreateTaskOkResponseModel> for CreateTaskOkViewModel {
    fn into(self) -> CreateTaskOkResponseModel {
        return CreateTaskOkResponseModel;
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub enum CreateTaskErrViewModel {
    TaskDescriptionLengthUnderflow {
        actual_length: usize,
        min_length_required: usize,
    },
    TaskDescriptionLengthOverflow {
        actual_length: usize,
        max_length_allowed: usize,
    },
}

impl From<CreateTaskErrResponseModel> for CreateTaskErrViewModel {
    fn from(response: CreateTaskErrResponseModel) -> Self {
        match response {
            CreateTaskErrResponseModel::TaskDescriptionLengthUnderflow {
                actual_length,
                min_length_required,
            } => Self::TaskDescriptionLengthUnderflow {
                actual_length,
                min_length_required,
            },
            CreateTaskErrResponseModel::TaskDescriptionLengthOverflow {
                actual_length,
                max_length_allowed,
            } => Self::TaskDescriptionLengthOverflow {
                actual_length,
                max_length_allowed,
            },
        }
    }
}

impl Into<CreateTaskErrResponseModel> for CreateTaskErrViewModel {
    fn into(self) -> CreateTaskErrResponseModel {
        match self {
            Self::TaskDescriptionLengthUnderflow {
                actual_length,
                min_length_required,
            } => CreateTaskErrResponseModel::TaskDescriptionLengthUnderflow {
                actual_length,
                min_length_required,
            },
            Self::TaskDescriptionLengthOverflow {
                actual_length,
                max_length_allowed,
            } => CreateTaskErrResponseModel::TaskDescriptionLengthOverflow {
                actual_length,
                max_length_allowed,
            },
        }
    }
}
