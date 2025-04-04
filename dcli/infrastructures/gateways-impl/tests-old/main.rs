use cucumber::World;
use crate::worlds::repositories::tasks::InMemoryTaskRepositoryWorld;

pub mod parameters;
pub mod steps;
pub mod worlds;

#[tokio::test(flavor = "multi_thread")]
async fn main() {
    InMemoryTaskRepositoryWorld::run("tests/.features/repositories/tasks/task_repository.feature").await
}
