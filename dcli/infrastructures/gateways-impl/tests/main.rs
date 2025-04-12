use gateways_impl::repositories::tasks::InMemoryTaskRepository;
use suites::repositories::tasks::task_repository_suite;
use worlds::repositories::tasks::TaskRepositoryWorldHandle;

mod params;
mod suites;
mod worlds;

pub struct InMemoryTaskRepositoryWorldHandle;

impl TaskRepositoryWorldHandle for InMemoryTaskRepositoryWorldHandle {
    type TaskRepository = InMemoryTaskRepository;

    fn new() -> Self::TaskRepository { InMemoryTaskRepository::new() }
}

task_repository_suite::suite!(handle = InMemoryTaskRepositoryWorldHandle);
