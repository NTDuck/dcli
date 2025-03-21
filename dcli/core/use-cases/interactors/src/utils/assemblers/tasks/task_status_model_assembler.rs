use axiom::behaviours::New;
use boundaries::tasks::models::TaskStatusModel;
use domain::tasks::TaskStatus;

#[derive(New)]
pub struct TaskStatusModelAssembler;

impl TaskStatusModelAssembler {
    pub fn assemble(&self, task_status: TaskStatus) -> TaskStatusModel {
        return match task_status {
            TaskStatus::Pending => TaskStatusModel::Pending,
            TaskStatus::InProgress => TaskStatusModel::InProgress,
            TaskStatus::Completed => TaskStatusModel::Completed,
        };
    }
}
