use axiom::behaviours::New;
use domain::tasks::TaskStatus;

use crate::utils::dataclasses::tasks::TaskStatusModel;

#[derive(New)]
pub struct TaskStatusModelAssembler;

impl TaskStatusModelAssembler {
    pub fn assemble(&self, task_status: TaskStatus) -> TaskStatusModel {
        match task_status {
            TaskStatus::Pending => TaskStatusModel::Pending,
            TaskStatus::InProgress => TaskStatusModel::InProgress,
            TaskStatus::Completed => TaskStatusModel::Completed,
        }
    }
}
