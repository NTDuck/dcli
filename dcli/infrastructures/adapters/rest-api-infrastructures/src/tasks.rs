use axum::Json;
use boundaries::tasks::CreateTaskBoundary;
use boundaries::tasks::CreateTaskRequestModel;
use boundaries::tasks::CreateTaskResponseModel;
use boundaries::tasks::ViewTasksBoundary;
use boundaries::tasks::ViewTasksRequestModel;
use boundaries::tasks::ViewTasksResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;

pub struct TaskController<Handle: PointerHandle> {
    create_task_boundary: SharedPointer<Box<dyn CreateTaskBoundary>, Handle>,
    view_tasks_boundary: SharedPointer<Box<dyn ViewTasksBoundary>, Handle>,
}

impl<Handle: PointerHandle> TaskController<Handle> {
    pub async fn create_task(&self, Json(request): Json<CreateTaskRequestModel>) -> Json<CreateTaskResponseModel> {
        let response = self.create_task_boundary.as_ref().apply(request);
        return Json(response);
    }
    
    pub async fn view_tasks(&self, Json(request): Json<ViewTasksRequestModel>) -> Json<ViewTasksResponseModel> {
        let response = self.view_tasks_boundary.as_ref().apply(request);
        return Json(response);
    }
}
