use axiom::behaviours::New;
use boundaries::tasks::CreateTaskBoundary;
use boundaries::tasks::ViewTasksBoundary;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;

#[derive(New)]
pub struct TasksState<Handle: PointerHandle> {
    pub(crate) create_task_boundary: SharedPointer<Box<dyn CreateTaskBoundary>, Handle>,
    pub(crate) view_tasks_boundary: SharedPointer<Box<dyn ViewTasksBoundary>, Handle>,
}
