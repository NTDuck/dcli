use cucumber::World;
use crate::worlds::repositories::tasks::InMemoryTaskRepositoryWorld;
use crate::steps::repositories::tasks::*;

pub mod parameters;
pub mod steps;
pub mod worlds;

#[tokio::test]
async fn main() {
    InMemoryTaskRepositoryWorld::run("tests/.features/repositories/tasks/task_repository.feature").await
}
