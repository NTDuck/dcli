use std::ops::Deref;

use domain::tasks::Task;

use crate::boundaries::tasks::TaskModel;
use crate::gateways::formatters::time::TimestampFormatter;
use crate::utils::interfaces::DeferredNewFrom;

impl<TimestampFormatterRef> DeferredNewFrom<Task, TimestampFormatterRef> for TaskModel
where
    TimestampFormatterRef: Deref<Target = Box<dyn TimestampFormatter>>,
{
    fn new(task: Task, timestamp_formatter: TimestampFormatterRef) -> Self {
        return Self {
            id: task.id.to_u64(),
            description: task.description.to_string(),
            status: task.status.into(),
            created_at: timestamp_formatter.format(task.id.get_timestamp()),
        };
    }
}
