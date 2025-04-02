use cucumber::World;
use repositories::tasks::inmemory_task_repository_test;

mod repositories;
mod utils;

#[tokio::main]
async fn main() {
    inmemory_task_repository_test::World::run("tests/.features/tasks/task_repository.feature").await
}
