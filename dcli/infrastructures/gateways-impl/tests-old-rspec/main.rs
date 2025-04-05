use environments::repositories::tasks::{TaskRepositoryEnvironment, TaskRepositoryEnvironmentHandle};
use gateways_impl::repositories::tasks::InMemoryTaskRepository;
use suites::repositories::tasks::TaskRepositorySuite;

mod environments;
mod suites;

pub struct InMemoryTaskRepositoryEnvironmentHandle;

impl TaskRepositoryEnvironmentHandle for InMemoryTaskRepositoryEnvironmentHandle {
    type TaskRepository = InMemoryTaskRepository;

    fn new() -> Self::TaskRepository {
        InMemoryTaskRepository::new()
    }
}

pub type InMemoryTaskRepositoryEnvironment = TaskRepositoryEnvironment<InMemoryTaskRepositoryEnvironmentHandle>;

#[test]
fn main() {
    rspec::run(&TaskRepositorySuite::with(InMemoryTaskRepositoryEnvironment::new()));
}
