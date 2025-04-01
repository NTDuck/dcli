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
    fn from(model: CreateTaskRequestModel) -> Self {
        Self {
            task_description: model.task_description,
        }
    }
}

#[cfg(feature = "server")]
impl From<CreateTaskRequestObject> for CreateTaskRequestModel {
    fn from(model: CreateTaskRequestObject) -> Self {
        CreateTaskRequestModel {
            task_description: model.task_description,
        }
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTaskViewModel {
    Ok(CreateTaskOkViewModel),
    Err(CreateTaskErrViewModel),
}

#[cfg(feature = "server")]
impl From<CreateTaskResponseModel> for CreateTaskViewModel {
    fn from(model: CreateTaskResponseModel) -> Self {
        let model = model
            .map(CreateTaskOkViewModel::from)
            .map_err(CreateTaskErrViewModel::from);

        match model {
            Ok(model) => Self::Ok(model),
            Err(model) => Self::Err(model),
        }
    }
}

#[cfg(feature = "client")]
impl From<CreateTaskViewModel> for CreateTaskResponseModel {
    fn from(model: CreateTaskViewModel) -> Self {
        match model {
            CreateTaskViewModel::Ok(model) => Ok(model.into()),
            CreateTaskViewModel::Err(model) => Err(model.into()),
        }
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct CreateTaskOkViewModel;

#[cfg(feature = "server")]
impl From<CreateTaskOkResponseModel> for CreateTaskOkViewModel {
    fn from(_: CreateTaskOkResponseModel) -> Self {
        Self
    }
}

#[cfg(feature = "client")]
impl From<CreateTaskOkViewModel> for CreateTaskOkResponseModel {
    fn from(_: CreateTaskOkViewModel) -> Self {
        CreateTaskOkResponseModel
    }
}

#[allow(clippy::enum_variant_names)]
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

#[cfg(feature = "server")]
impl From<CreateTaskErrResponseModel> for CreateTaskErrViewModel {
    fn from(model: CreateTaskErrResponseModel) -> Self {
        match model {
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

#[cfg(feature = "client")]
impl From<CreateTaskErrViewModel> for CreateTaskErrResponseModel {
    fn from(model: CreateTaskErrViewModel) -> Self {
        match model {
            CreateTaskErrViewModel::TaskDescriptionLengthUnderflow {
                actual_length,
                min_length_required,
            } => CreateTaskErrResponseModel::TaskDescriptionLengthUnderflow {
                actual_length,
                min_length_required,
            },
            CreateTaskErrViewModel::TaskDescriptionLengthOverflow {
                actual_length,
                max_length_allowed,
            } => CreateTaskErrResponseModel::TaskDescriptionLengthOverflow {
                actual_length,
                max_length_allowed,
            },
        }
    }
}
