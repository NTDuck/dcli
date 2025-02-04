use gateways::factories::ids::UuidV4Factory;
use gateways::repositories::inmemory::tasks::OrderedInMemoryTaskRepository;
use interface_adapters::controllers::tasks::CreateTaskController;
use use_cases::gateways::factories::ids::UuidFactory;
use use_cases::gateways::repositories::tasks::TaskRepository;
use use_cases::interactors::tasks::CreateTaskInteractor;

fn main() {
    let mut task_repository = OrderedInMemoryTaskRepository::new();
    let uuid_factory = UuidV4Factory::new();

    let _create_task_interactor = CreateTaskInteractor::new(
        &mut task_repository,
        &uuid_factory,
    );

    // let create_task_controller = CreateTaskController {
    //     interactor: create_task_interactor,
    // };

    loop {
        break;   
    }
}
