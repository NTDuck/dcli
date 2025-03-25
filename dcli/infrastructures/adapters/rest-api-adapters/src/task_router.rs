use axum::Json;
use boundaries::tasks::CreateTaskInputBoundary;
use boundaries::tasks::CreateTaskOutputBoundary;
use boundaries::tasks::CreateTaskRequestModel;
use boundaries::tasks::CreateTaskResponseModel;
use boundaries::tasks::ViewTasksInputBoundary;
use boundaries::tasks::ViewTasksOutputBoundary;
use boundaries::tasks::ViewTasksRequestModel;
use boundaries::tasks::ViewTasksResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;

pub struct TaskRouter<Handle: PointerHandle> {
    create_task_boundary: SharedPointer<Box<dyn CreateTaskInputBoundary>, Handle>,
    view_tasks_boundary: SharedPointer<Box<dyn ViewTasksInputBoundary>, Handle>,

    cached_create_task_response_model: SharedPointer<Option<CreateTaskResponseModel>, Handle>,
    cached_view_tasks_response_model: SharedPointer<Option<ViewTasksResponseModel>, Handle>,
}

impl<Handle: PointerHandle> TaskRouter<Handle> {
    pub fn new(
        create_task_boundary: SharedPointer<Box<dyn CreateTaskInputBoundary>, Handle>,
        view_tasks_boundary: SharedPointer<Box<dyn ViewTasksInputBoundary>, Handle>,  
    ) -> Self {
        return Self {
            create_task_boundary: create_task_boundary.clone(),
            view_tasks_boundary: view_tasks_boundary.clone(),
            
            cached_create_task_response_model: SharedPointer::new(None),
            cached_view_tasks_response_model: SharedPointer::new(None),
        };
    }

    pub async fn create_task_post(&self, Json(request): Json<CreateTaskRequestModel>) {
        self.create_task_boundary.as_ref().accept(request);
    }
    
    pub async fn view_tasks_post(&self, Json(request): Json<ViewTasksRequestModel>) {
        self.view_tasks_boundary.as_ref().accept(request);
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

impl<Handle: PointerHandle> CreateTaskOutputBoundary for TaskRouter<Handle> {
    fn accept(&self, response: CreateTaskResponseModel) {
        *self.cached_create_task_response_model.as_mut() = Some(response);
    }
}

impl<Handle: PointerHandle> ViewTasksOutputBoundary for TaskRouter<Handle> {
    fn accept(&self, response: ViewTasksResponseModel) {
        *self.cached_view_tasks_response_model.as_mut() = Some(response);
    }
}
