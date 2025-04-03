use std::fmt::Debug;

use cucumber::parser::Error;
use use_cases::gateways::repositories::tasks::TaskRepository;

pub struct TaskRepositoryWorld<Handle: TaskRepositoryWorldHandle> {
    pub task_repository: Handle::TaskRepositoryImpl,
    pub task_count: usize,
}

impl<Handle: TaskRepositoryWorldHandle + 'static> cucumber::World for TaskRepositoryWorld<Handle> {
    type Error = Error;

    async fn new() -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<Handle: TaskRepositoryWorldHandle> Debug for TaskRepositoryWorld<Handle> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TaskRepositoryWorld")
            .field("task_repository", &self.task_repository)
            .field("task_count", &self.task_count)
            .finish()
    }
}

impl<Handle: TaskRepositoryWorldHandle> Default for TaskRepositoryWorld<Handle> {
    fn default() -> Self {
        Self {
            task_repository: Handle::default(),
            task_count: Default::default(),
        }
    }
}

pub trait TaskRepositoryWorldHandle {
    type TaskRepositoryImpl: TaskRepository;

    fn default() -> Self::TaskRepositoryImpl;
}
