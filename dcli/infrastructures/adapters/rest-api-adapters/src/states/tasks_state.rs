use boundaries::tasks::CreateTaskBoundary;
use boundaries::tasks::ViewTasksBoundary;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;

pub struct TasksState<Handle: PointerHandle> {
    pub create_task_boundary: SharedPointer<Box<dyn CreateTaskBoundary>, Handle>,
    pub view_tasks_boundary: SharedPointer<Box<dyn ViewTasksBoundary>, Handle>,
}
