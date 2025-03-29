use axiom::interfaces::DataTransferObjectWithoutSerde;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use boundaries::tasks::CreateTaskErrResponseModel;
use boundaries::tasks::CreateTaskOkResponseModel;
use boundaries::tasks::CreateTaskRequestModel;
use boundaries::tasks::CreateTaskResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;
use serde::Deserialize;
use serde::Serialize;

use crate::states::TasksState;

pub async fn create_task<Handle: PointerHandle>(
    State(state): State<SharedPointer<TasksState<Handle>, Handle>>,
    Query(request): Query<CreateTaskRequestObject>,
) -> impl IntoResponse {
    let request = request.into();
    let response = state.as_ref().create_task_boundary.as_ref().apply(request);
    let response = response.into();

    let status_code = match response {
        CreateTaskViewModel::Ok(_) => StatusCode::OK,
        CreateTaskViewModel::Err(_) => StatusCode::BAD_REQUEST,
    };

    return (status_code, Json(response));
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct CreateTaskRequestObject {
    pub task_description: String,
}

impl Into<CreateTaskRequestModel> for CreateTaskRequestObject {
    fn into(self) -> CreateTaskRequestModel {
        return CreateTaskRequestModel {
            task_description: self.task_description,
        };
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
#[serde(untagged)]
enum CreateTaskViewModel {
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

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
struct CreateTaskOkViewModel;

impl From<CreateTaskOkResponseModel> for CreateTaskOkViewModel {
    fn from(_: CreateTaskOkResponseModel) -> Self {
        return Self;
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
enum CreateTaskErrViewModel {
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
