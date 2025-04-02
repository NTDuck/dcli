use cucumber::World;
use worlds::repositories::tasks::InMemoryTaskRepositoryWorld;

pub mod steps;
pub mod worlds;

mod utils;

#[tokio::test]
async fn main() {
    InMemoryTaskRepositoryWorld::run("tests/.features/repositories/tasks/task_repository.feature").await
}
