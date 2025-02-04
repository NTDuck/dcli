use std::sync::Arc;
use std::sync::RwLock;

use console::utils::io::IoGateway;
use gateways::factories::ids::UuidV4Factory;
use gateways::repositories::inmemory::tasks::OrderedInMemoryTaskRepository;
use interface_adapters::controllers::tasks::CreateTaskController;
use interface_adapters::controllers::tasks::ViewTasksController;
use use_cases::gateways::factories::ids::UuidFactory;
use use_cases::gateways::repositories::tasks::TaskRepository;
use use_cases::interactors::tasks::CreateTaskInteractor;
use use_cases::interactors::tasks::ViewTasksInteractor;

fn main() {
    // Gateways
    let task_repository: Arc<RwLock<dyn TaskRepository>> = Arc::new(RwLock::new(OrderedInMemoryTaskRepository::new()));
    let uuid_factory: Arc<RwLock<dyn UuidFactory>> = Arc::new(RwLock::new(UuidV4Factory::new()));

    // Interactors
    let create_task_interactor = CreateTaskInteractor::new(Arc::clone(&task_repository), Arc::clone(&uuid_factory));
    let view_tasks_interactor = ViewTasksInteractor::new(Arc::clone(&task_repository));

    // Controllers
    let create_task_controller = CreateTaskController::new(&create_task_interactor);
    let view_tasks_controller = ViewTasksController::new(&view_tasks_interactor);

    // I/O
    let io_gateway = IoGateway;

    loop {
        io_gateway.write("\
            Select a number:\n\
            [0] Exit\n\
            [1] View all tasks\n\
            [2] Create a task\n \
        ");
        
        match io_gateway.read_line().trim() {
            "0" => {
                io_gateway.write_line("Exit signal received.");
                break;
            },
            "1" => {

            },
            "2" => {

            },
            _ => {
                io_gateway.write_line("Invalid number.");
                continue;
            },
        }
    }
}
