use axum::extract::State;
use axum::Json;
use boundaries::tasks::CreateTaskRequestModel;
use boundaries::tasks::CreateTaskResponseModel;
use boundaries::tasks::ViewTasksRequestModel;
use boundaries::tasks::ViewTasksResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;

use crate::states::TasksState;

pub async fn create_task_post<Handle: PointerHandle>(
    State(state): State<SharedPointer<TasksState<Handle>, Handle>>,
    Json(request): Json<CreateTaskRequestModel>,
) {
    *state.as_ref().cached_create_task_response_model.as_mut() =
        Some(state.as_ref().create_task_boundary.as_ref().apply(request));
}

pub async fn view_tasks_post<Handle: PointerHandle>(
    State(state): State<SharedPointer<TasksState<Handle>, Handle>>,
    Json(request): Json<ViewTasksRequestModel>,
) {
    *state.as_ref().cached_view_tasks_response_model.as_mut() =
        Some(state.as_ref().view_tasks_boundary.as_ref().apply(request));
}

pub async fn create_task_get<Handle: PointerHandle>(
    State(state): State<SharedPointer<TasksState<Handle>, Handle>>,
) -> Json<CreateTaskResponseModel> {
    let cached_response = state.as_ref().cached_create_task_response_model.as_mut().take();

    match cached_response {
        Some(response) => Json(response),
        None => panic!(),
    }
}

pub async fn view_tasks_get<Handle: PointerHandle>(
    State(state): State<SharedPointer<TasksState<Handle>, Handle>>,
) -> Json<ViewTasksResponseModel> {
    let cached_response = state.as_ref().cached_view_tasks_response_model.as_mut().take();

    match cached_response {
        Some(response) => Json(response),
        None => panic!(),
    }
}
