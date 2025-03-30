use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;

use crate::utils::models::tasks::ViewTasksRequestObject;
use crate::utils::models::tasks::ViewTasksViewModel;
use crate::server::states::TasksState;

pub async fn view_tasks<Handle: PointerHandle>(
    State(state): State<SharedPointer<TasksState<Handle>, Handle>>,
    Json(request): Json<ViewTasksRequestObject>,
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
