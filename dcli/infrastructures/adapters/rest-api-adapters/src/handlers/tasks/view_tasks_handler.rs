use axiom::interfaces::DataTransferObjectWithoutSerde;
use axum::extract::Query;
use axum::extract::State;
use axum::Json;
use boundaries::tasks::ViewTasksRequestModel;
use boundaries::tasks::ViewTasksResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;
use models::pagination::PaginationRequest;
use serde::Deserialize;
use serde::Serialize;

use crate::states::TasksState;

pub async fn view_tasks<Handle: PointerHandle>(
    State(state): State<SharedPointer<TasksState<Handle>, Handle>>,
    Query(request): Query<ViewTasksRequestObject>,
) -> Json<ViewTasksResponseModel> {
    let request = request.into();
    let response = state.as_ref().view_tasks_boundary.as_ref().apply(request);
    
    return Json(response);
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksRequestObject {
    pub page_number: usize,
    pub max_page_size: usize,
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

pub type ViewTasksViewModel = ViewTasksResponseModel;
