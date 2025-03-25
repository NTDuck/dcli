use std::sync::atomic::AtomicU16;

use axum::routing::post;
use axum::Router;
use boundaries::tasks::CreateTaskBoundary;
use boundaries::tasks::ViewTasksBoundary;
use chrono_gateways_impl::formatters::Rfc2822TimestampFormatter;
use gateways::formatters::time::TimestampFormatter;
use gateways::pointers::SharedPointer;
use gateways::providers::ids::SnowflakeProvider;
use gateways::providers::time::TimestampProvider;
use gateways::repositories::tasks::TaskRepository;
use interactors::tasks::CreateTaskInteractor;
use interactors::tasks::ViewTasksInteractor;
use rest_api_adapters::task_router::TaskRouter;
use std_gateways_impl::pointers::handles::PointerHandleWithStrategy;
use std_gateways_impl::pointers::strategies::ArcRwLockSharedPointerStrategy;
use std_gateways_impl::providers::ids::CentralizedSnowflakeProvider;
use std_gateways_impl::providers::time::CentralizedSystemTimestampProvider;
use std_gateways_impl::repositories::tasks::InMemoryTaskRepository;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    type Pointer<T> = SharedPointer<T, PointerHandle>;
    type PointerHandle = PointerHandleWithStrategy<PointerStrategy>;
    type PointerStrategy = ArcRwLockSharedPointerStrategy;
    
    let timestamp_formatter: Pointer<Box<dyn TimestampFormatter>> =
        Pointer::new(Box::new(Rfc2822TimestampFormatter::new()));
    let timestamp_provider: Pointer<Box<dyn TimestampProvider>> =
        Pointer::new(Box::new(CentralizedSystemTimestampProvider::new()));
    let snowflake_provider: Pointer<Box<dyn SnowflakeProvider>> =
        Pointer::new(Box::new(CentralizedSnowflakeProvider::new(1, AtomicU16::new(0))));
    let task_repository: Pointer<Box<dyn TaskRepository>> =
        Pointer::new(Box::new(InMemoryTaskRepository::new()));

    let create_task_interactor: Pointer<Box<dyn CreateTaskBoundary>> =
        Pointer::new(Box::new(CreateTaskInteractor::new(timestamp_provider.clone(), snowflake_provider.clone(), task_repository.clone())));
    let view_tasks_interactor: Pointer<Box<dyn ViewTasksBoundary>> =
        Pointer::new(Box::new(ViewTasksInteractor::new(timestamp_formatter.clone(), task_repository.clone())));

    let task_controller = TaskRouter::new(create_task_interactor.clone(), view_tasks_interactor.clone());

    // Now go back to segregated boundaries...
    let router = Router::new()
        .route("/tasks", post(|request| task_controller.create_task_get(request)))
        .route("/tasks/view", post(|request| task_controller.view_tasks_get(request)));
    
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
