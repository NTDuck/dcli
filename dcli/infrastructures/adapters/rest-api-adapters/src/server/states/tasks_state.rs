use use_cases::boundaries::tasks::CreateTaskBoundary;
use use_cases::boundaries::tasks::ViewTasksBoundary;
use use_cases::gateways::pointers::PointerHandle;
use use_cases::gateways::pointers::SharedPointer;

pub struct TasksState<Handle: PointerHandle> {
    pub create_task_boundary:
        SharedPointer<Box<dyn CreateTaskBoundary>, Handle>,
    pub view_tasks_boundary: SharedPointer<Box<dyn ViewTasksBoundary>, Handle>,
}
