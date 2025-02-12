use domain::Task;
use fake::Dummy;
use fake::Fake;
use fake::Faker;

use crate::utils::mocks::tasks::MockTaskId;
use crate::utils::mocks::tasks::MockTaskDescription;
use crate::utils::mocks::tasks::MockTaskStatus;
use crate::utils::mocks::time::mockTimestamp;

pub fn mockTask() -> Task {
    return Faker.fake::<MockTask>()
        .into();
}

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
            createdAt: mockTimestamp(),
        };
    }
}
