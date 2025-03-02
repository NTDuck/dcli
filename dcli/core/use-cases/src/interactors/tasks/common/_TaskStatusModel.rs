use domain::tasks::TaskStatus;

use crate::boundaries::tasks::TaskStatusModel;

impl From<TaskStatus> for TaskStatusModel {
    fn from(task_status: TaskStatus) -> Self {
        return match task_status {
            TaskStatus::Pending => TaskStatusModel::Pending,
            TaskStatus::InProgress => TaskStatusModel::InProgress,
            TaskStatus::Completed => TaskStatusModel::Completed,
        };
    }
}
