use std::fmt::Debug;

use use_cases::gateways::repositories::tasks::TaskRepository;

pub struct TaskRepositoryEnvironment<Handle: TaskRepositoryEnvironmentHandle> {
    pub task_repository: Handle::TaskRepository,
}

impl<Handle: TaskRepositoryEnvironmentHandle> TaskRepositoryEnvironment<Handle> {
    pub fn new() -> Self {
        Self {
            task_repository: Handle::new(),
        }
    }
}

impl<Handle: TaskRepositoryEnvironmentHandle> Clone for TaskRepositoryEnvironment<Handle> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<Handle: TaskRepositoryEnvironmentHandle> Debug for TaskRepositoryEnvironment<Handle> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(stringify!(Handle::TaskRepository))
            .finish()
    }
}

pub trait TaskRepositoryEnvironmentHandle {
    type TaskRepository: TaskRepository;

    fn new() -> Self::TaskRepository;
}
