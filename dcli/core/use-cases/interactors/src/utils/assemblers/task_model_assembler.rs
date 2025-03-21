use axiom::behaviours::New;
use boundaries::tasks::models::TaskModel;
use domain::tasks::Task;
use gateways::formatters::time::TimestampFormatter;
use gateways::pointers::PointerHandle;

use crate::utils::pointers::SharedPointer;

#[derive(New)]
pub struct TaskModelAssembler<Handle: PointerHandle> {
    timestamp_formatter: SharedPointer<Box<dyn TimestampFormatter>, Handle>,
}

impl<Handle: PointerHandle> TaskModelAssembler<Handle> {
    pub fn assemble(&self, task: Task) -> TaskModel {
        let created_at = task.id.get_timestamp();

        return TaskModel {
            id: task.id.to_u64(),
            description: task.description.to_string(),
            status: task.status.into(),
            created_at: self.timestamp_formatter.format(created_at),
        };
    }
}
