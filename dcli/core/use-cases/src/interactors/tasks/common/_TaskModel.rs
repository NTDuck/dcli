use domain::tasks::Task;

use crate::boundaries::tasks::TaskModel;
use crate::boundaries::tasks::TaskStatusModel;

impl From<Task> for TaskModel {
    fn from(task: Task) -> Self {
        return Self {
            id: *task.id,
            description: task.description.as_string(),
            status: TaskStatusModel::from(task.status),
            created_at: ,
        }
    }
}
