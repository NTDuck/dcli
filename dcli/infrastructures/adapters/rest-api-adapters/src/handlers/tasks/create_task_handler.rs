use axum::extract::Query;
use axum::extract::State;
use axum::Json;
use boundaries::tasks::CreateTaskRequestModel;
use boundaries::tasks::CreateTaskResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;

use crate::states::TasksState;

pub async fn create_task<Handle: PointerHandle>(
    State(state): State<SharedPointer<TasksState<Handle>, Handle>>,
    Query(request): Query<CreateTaskRequestObject>,
) -> Json<CreateTaskViewModel> {
    let response = state.as_ref().create_task_boundary.as_ref().apply(request);
    
    return Json(response);
}

pub type CreateTaskRequestObject = CreateTaskRequestModel;
pub type CreateTaskViewModel = CreateTaskResponseModel;
