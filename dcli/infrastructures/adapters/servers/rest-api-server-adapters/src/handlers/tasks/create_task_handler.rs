use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;

use crate::models::tasks::CreateTaskRequestObject;
use crate::models::tasks::CreateTaskViewModel;
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
