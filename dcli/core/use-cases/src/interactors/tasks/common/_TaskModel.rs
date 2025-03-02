use std::ops::Deref;

use domain::tasks::Task;

use crate::boundaries::tasks::TaskModel;
use crate::gateways::formatters::time::TimestampFormatter;
use crate::utils::interfaces::FromUsing;

impl<Formatter> FromUsing<Task, Formatter> for TaskModel
where
    Formatter: Deref<Target = Box<dyn TimestampFormatter>>,
{
    fn from_using(task: Task, timestamp_formatter: Formatter) -> Self {
        return Self {
            id: task.id.to_u64(),
            description: task.description.to_string(),
            status: task.status.into(),
            created_at: timestamp_formatter
                .format(task.id.get_timestamp()),
        };
    }
}
