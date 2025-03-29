use axiom::interfaces::DataTransferObjectWithoutSerde;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use boundaries::tasks::ViewTasksErrResponseModel;
use boundaries::tasks::ViewTasksOkResponseModel;
use boundaries::tasks::ViewTasksRequestModel;
use boundaries::tasks::ViewTasksResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;
use models::pagination::PaginationRequest;
use models::tasks::TaskModel;
use serde::Deserialize;
use serde::Serialize;

use crate::states::TasksState;

pub async fn view_tasks<Handle: PointerHandle>(
    State(state): State<SharedPointer<TasksState<Handle>, Handle>>,
    Query(request): Query<ViewTasksRequestObject>,
) -> impl IntoResponse {
    let request = request.into();
    let response = state.as_ref().view_tasks_boundary.as_ref().apply(request);
    let response = response.into();

    let status_code = match response {
        ViewTasksViewModel::Ok(_) => StatusCode::OK,
        ViewTasksViewModel::Err(_) => StatusCode::BAD_REQUEST,
    };
    
    return (status_code, Json(response));
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksRequestObject {
    page_number: usize,
    max_page_size: usize,
}

impl Into<ViewTasksRequestModel> for ViewTasksRequestObject {
    fn into(self) -> ViewTasksRequestModel {
        return ViewTasksRequestModel {
            pagination_request: PaginationRequest {
                page_number: self.page_number,
                max_page_size: self.max_page_size,
            },
        };
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
#[serde(untagged)]
enum ViewTasksViewModel {
    Ok(ViewTasksOkViewModel),
    Err(ViewTasksErrViewModel),
}

impl From<ViewTasksResponseModel> for ViewTasksViewModel {
    fn from(response: ViewTasksResponseModel) -> Self {
        let response = response
            .map(ViewTasksOkViewModel::from)
            .map_err(ViewTasksErrViewModel::from);

        match response {
            Ok(response) => Self::Ok(response),
            Err(response) => Self::Err(response),
        }
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
struct ViewTasksOkViewModel {
    tasks: Vec<TaskModel>,
    
    page_size: usize,
    max_page_size: usize,
    page_number: usize,
    max_page_number: usize,
}

impl From<ViewTasksOkResponseModel> for ViewTasksOkViewModel {
    fn from(response: ViewTasksOkResponseModel) -> Self {
        let pagination_response = response.pagination_response;

        return Self {
            tasks: pagination_response.items,

            page_size: pagination_response.page_size,
            max_page_size: pagination_response.max_page_size,
            page_number: pagination_response.page_number,
            max_page_number: pagination_response.max_page_number,
        };
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
struct ViewTasksErrViewModel;

impl From<ViewTasksErrResponseModel> for ViewTasksErrViewModel {
    fn from(_: ViewTasksErrResponseModel) -> Self {
        return Self;
    }
}
