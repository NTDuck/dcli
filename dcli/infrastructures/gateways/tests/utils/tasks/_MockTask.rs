use domain::utils::Timestamp;
use domain::Task;
use fake::Dummy;

use crate::utils::tasks::MockTaskId;
use crate::utils::tasks::MockTaskDescription;
use crate::utils::tasks::MockTaskStatus;

#[derive(Dummy)]
pub struct MockTask {
    id: MockTaskId,
    description: MockTaskDescription,
    status: MockTaskStatus,
}

impl Into<Task> for MockTask {
    fn into(self) -> Task {
        return Task {
            id: self.id.into(),
            description: self.description.into(),
            status: self.status.into(),
            created_at: Timestamp::now(),
        };
    }
}