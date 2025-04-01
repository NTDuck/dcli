use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use use_cases::gateways::pointers::PointerHandle;
use use_cases::gateways::pointers::SharedPointer;

use crate::server::states::TasksState;
use crate::utils::dataclasses::tasks::ViewTasksRequestObject;
use crate::utils::dataclasses::tasks::ViewTasksViewModel;

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

    (status_code, Json(response))
}
