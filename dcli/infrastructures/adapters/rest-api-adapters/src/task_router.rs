use axum::Json;
use boundaries::tasks::CreateTaskBoundary;
use boundaries::tasks::CreateTaskRequestModel;
use boundaries::tasks::CreateTaskResponseModel;
use boundaries::tasks::ViewTasksBoundary;
use boundaries::tasks::ViewTasksRequestModel;
use boundaries::tasks::ViewTasksResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;

pub struct TaskRouter<Handle: PointerHandle> {
    create_task_boundary: SharedPointer<Box<dyn CreateTaskBoundary>, Handle>,
    view_tasks_boundary: SharedPointer<Box<dyn ViewTasksBoundary>, Handle>,

    cached_create_task_response_model: SharedPointer<Option<CreateTaskResponseModel>, Handle>,
    cached_view_tasks_response_model: SharedPointer<Option<ViewTasksResponseModel>, Handle>,
}

impl<Handle: PointerHandle> TaskRouter<Handle> {
    pub fn new(
        create_task_boundary: SharedPointer<Box<dyn CreateTaskBoundary>, Handle>,
        view_tasks_boundary: SharedPointer<Box<dyn ViewTasksBoundary>, Handle>,  
    ) -> Self {
        return Self {
            create_task_boundary: create_task_boundary.clone(),
            view_tasks_boundary: view_tasks_boundary.clone(),
            
            cached_create_task_response_model: SharedPointer::new(None),
            cached_view_tasks_response_model: SharedPointer::new(None),
        };
    }

    pub async fn create_task_post(&self, Json(request): Json<CreateTaskRequestModel>) {
        *self.cached_create_task_response_model.as_mut() =
            Some(self.create_task_boundary.as_ref().apply(request));
    }
    
    pub async fn view_tasks_post(&self, Json(request): Json<ViewTasksRequestModel>) {
        *self.cached_view_tasks_response_model.as_mut() =
            Some(self.view_tasks_boundary.as_ref().apply(request));
    }

    pub async fn create_task_get(&self) -> Json<CreateTaskResponseModel> {
        let cached_response = self.cached_create_task_response_model.as_mut().take();

        match cached_response {
            Some(response) => Json(response),
            None => panic!(),
        }
    }

    pub async fn view_tasks_get(&self) -> Json<ViewTasksResponseModel> {
        let cached_response = self.cached_view_tasks_response_model.as_mut().take();

        match cached_response {
            Some(response) => Json(response),
            None => panic!(),
        }
    }
}
