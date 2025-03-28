use boundaries::tasks::CreateTaskBoundary;
use boundaries::tasks::CreateTaskResponseModel;
use boundaries::tasks::ViewTasksBoundary;
use boundaries::tasks::ViewTasksResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;

pub struct TasksState<Handle: PointerHandle> {
    pub(crate) create_task_boundary: SharedPointer<Box<dyn CreateTaskBoundary>, Handle>,
    pub(crate) view_tasks_boundary: SharedPointer<Box<dyn ViewTasksBoundary>, Handle>,

    pub(crate) cached_create_task_response_model: SharedPointer<Option<CreateTaskResponseModel>, Handle>,
    pub(crate) cached_view_tasks_response_model: SharedPointer<Option<ViewTasksResponseModel>, Handle>,
}

impl<Handle: PointerHandle> TasksState<Handle> {
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
}
