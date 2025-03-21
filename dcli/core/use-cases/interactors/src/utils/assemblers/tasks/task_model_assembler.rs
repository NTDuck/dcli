use boundaries::utils::dataclasses::tasks::TaskModel;
use domain::tasks::Task;
use gateways::formatters::time::TimestampFormatter;
use gateways::pointers::PointerHandle;

use crate::utils::assemblers::tasks::TaskStatusModelAssembler;
use crate::utils::pointers::SharedPointer;

pub struct TaskModelAssembler<Handle: PointerHandle> {
    timestamp_formatter: SharedPointer<Box<dyn TimestampFormatter>, Handle>,
    task_status_model_assembler: TaskStatusModelAssembler,
}

impl<Handle: PointerHandle> TaskModelAssembler<Handle> {
    pub fn new(
        timestamp_formatter: SharedPointer<Box<dyn TimestampFormatter>, Handle>,
    ) -> Self {
        return Self {
            timestamp_formatter,
            task_status_model_assembler: TaskStatusModelAssembler,
        };
    }

    pub fn assemble(&self, task: Task) -> TaskModel {
        let created_at = task.id.get_timestamp();

        return TaskModel {
            id: task.id.to_u64(),
            description: task.description.to_string(),
            status: self.task_status_model_assembler.assemble(task.status),
            created_at: self.timestamp_formatter.as_ref().format(created_at),
        };
    }
}
