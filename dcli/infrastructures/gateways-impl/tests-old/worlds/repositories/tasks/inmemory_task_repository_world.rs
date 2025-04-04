use gateways_impl::repositories::tasks::InMemoryTaskRepository;

#[derive(cucumber::World, Debug)]
pub struct InMemoryTaskRepositoryWorld {
    pub task_repository: InMemoryTaskRepository,
    pub task_count: usize,
}

impl Default for InMemoryTaskRepositoryWorld {
    fn default() -> Self {
        Self {
            task_repository: InMemoryTaskRepository::new(),
            task_count: 0,
        }
    }
}
