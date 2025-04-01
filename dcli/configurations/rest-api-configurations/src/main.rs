use axum::routing::get;
use axum::routing::post;
use axum::Router;
use gateways_impl::formatters::Rfc2822TimestampFormatter;
use gateways_impl::pointers::handles::PointerHandleWithStrategy;
use gateways_impl::pointers::strategies::triomphe::ArcRwLockSharedPointerStrategy;
use gateways_impl::providers::ids::CentralizedSnowflakeProvider;
use gateways_impl::providers::time::CentralizedSystemTimestampProvider;
use gateways_impl::repositories::tasks::InMemoryTaskRepository;
use rest_api_adapters::server::handlers::tasks::create_task;
use rest_api_adapters::server::handlers::tasks::view_tasks;
use rest_api_adapters::server::states::TasksState;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use use_cases::boundaries::tasks::CreateTaskBoundary;
use use_cases::boundaries::tasks::ViewTasksBoundary;
use use_cases::gateways::formatters::time::TimestampFormatter;
use use_cases::gateways::pointers::SharedPointer as SharedPointer_;
use use_cases::gateways::providers::ids::SnowflakeProvider;
use use_cases::gateways::providers::time::TimestampProvider;
use use_cases::gateways::repositories::tasks::TaskRepository;
use use_cases::interactors::tasks::CreateTaskInteractor;
use use_cases::interactors::tasks::ViewTasksInteractor;

#[tokio::main]
async fn main() {
    type SharedPointer<T> = SharedPointer_<T, PointerHandle>;
    type PointerHandle = PointerHandleWithStrategy<PointerStrategy>;
    type PointerStrategy = ArcRwLockSharedPointerStrategy;

    let timestamp_formatter: SharedPointer<Box<dyn TimestampFormatter>> =
        SharedPointer::new(Box::new(Rfc2822TimestampFormatter::new()));
    let timestamp_provider: SharedPointer<Box<dyn TimestampProvider>> =
        SharedPointer::new(Box::new(CentralizedSystemTimestampProvider::new()));
    let snowflake_provider: SharedPointer<Box<dyn SnowflakeProvider>> =
        SharedPointer::new(Box::new(CentralizedSnowflakeProvider::default()));
    let task_repository: SharedPointer<Box<dyn TaskRepository>> =
        SharedPointer::new(Box::new(InMemoryTaskRepository::new()));

    let create_task_interactor: SharedPointer<Box<dyn CreateTaskBoundary>> =
        SharedPointer::new(Box::new(CreateTaskInteractor::new(
            timestamp_provider.clone(),
            snowflake_provider.clone(),
            task_repository.clone(),
        )));
    let view_tasks_interactor: SharedPointer<Box<dyn ViewTasksBoundary>> =
        SharedPointer::new(Box::new(ViewTasksInteractor::new(
            timestamp_formatter.clone(),
            task_repository.clone(),
        )));

    let tasks_state: SharedPointer<TasksState<_>> =
        SharedPointer::new(TasksState {
            create_task_boundary: create_task_interactor.clone(),
            view_tasks_boundary: view_tasks_interactor.clone(),
        });

    let tasks_router = Router::new()
        .route("/create", post(create_task))
        .route("/view", get(view_tasks))
        .with_state(tasks_state);

    let router = Router::new()
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::new())
                .layer(CompressionLayer::new()),
        )
        .nest("/task", tasks_router);

    let listener = TcpListener::bind("127.0.0.1:4444").await.unwrap();

    tracing_subscriber::fmt()
        .compact()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    tracing::info!(
        "Running on {}://{}",
        "http",
        listener.local_addr().unwrap(),
    );

    axum::serve(listener, router).await.unwrap();
}
