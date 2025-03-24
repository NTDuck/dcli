use std::sync::atomic::AtomicU16;

use axum::routing::post;
use axum::Router;
use interactors::tasks::CreateTaskInteractor;
use interactors::tasks::ViewTasksInteractor;

#[tokio::main]
async fn main() {


    let timestamp_provider = CentralizedSystemTimestampProvider::new();
    let snowflake_provider = CentralizedSnowflakeProvider::new(1, AtomicU16::new(0));
    let task_repository = InMemoryTaskRepository::new();
    let timestamp_formatter = Timestamp

    let create_task_interactor = CreateTaskInteractor::new(timestamp_provider, snowflake_provider, task_repository);
    let view_tasks_interactor = ViewTasksInteractor::new(timestamp_formatter, task_repository);

    // let router = Router::new()
        // .route("/tasks",)
}
