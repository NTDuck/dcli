use std::usize;

use console::utils::io::IoGateway;
use gateways::factories::ids::UuidV4Factory;
use gateways::pointers::handles::StrategizedPointerHandle;
use gateways::pointers::strategies::triomphe::ArcRwLockSharedPointerStrategy;
use gateways::repositories::inmemory::tasks::OrderedInMemoryTaskRepository;
use interface_adapters::controllers::tasks::CreateTaskController;
use interface_adapters::controllers::tasks::CreateTaskRequestObject;
use interface_adapters::controllers::tasks::ViewTasksController;
use interface_adapters::controllers::tasks::ViewTasksRequestObject;
use interface_adapters::controllers::tasks::ViewTasksViewModel;
use interface_adapters::controllers::tasks::ViewableTask;
use use_cases::boundaries::tasks::CreateTaskErrorModel;
use use_cases::gateways::factories::ids::UuidFactory;
use use_cases::gateways::pointers::SharedPointer as ParameterizedSharedPointer;
use use_cases::gateways::repositories::tasks::TaskRepository;
use use_cases::interactors::tasks::CreateTaskInteractor;
use use_cases::interactors::tasks::ViewTasksInteractor;
use use_cases::utils::dataclasses::pagination::PaginationRequest;

fn main() {
    // Shared pointer type
    type SharedPointer<T> = ParameterizedSharedPointer<T, PointerHandle>;
    type PointerHandle = StrategizedPointerHandle<PointerStrategy>;
    type PointerStrategy = ArcRwLockSharedPointerStrategy;
    
    // Gateways
    let task_repository: SharedPointer<Box<dyn TaskRepository>> =
        SharedPointer::new(Box::new(OrderedInMemoryTaskRepository::new()));
    let uuid_factory: SharedPointer<Box<dyn UuidFactory>> =
        SharedPointer::new(Box::new(UuidV4Factory::new()));

    // Interactors
    let create_task_interactor =
        CreateTaskInteractor::new(task_repository.clone(), uuid_factory.clone());
    let view_tasks_interactor = 
        ViewTasksInteractor::new(task_repository.clone());

    // Controllers
    let create_task_controller = CreateTaskController::new(&create_task_interactor);
    let view_tasks_controller = ViewTasksController::new(&view_tasks_interactor);

    // I/O
    let io_gateway = IoGateway;

    // Main loop
    loop {
        io_gateway.write(
            "\
            Select a number:\n\
            [0] Exit\n\
            [1] Create a task\n\
            [2] View all tasks\n \
        ",
        );

        match io_gateway.read_line().trim() {
            "0" => {
                io_gateway.write_line("Exit signal received.");
                break;
            },
            "1" => {
                io_gateway.write("Enter task description: ");
                let task_description = io_gateway.read_line();

                let request = CreateTaskRequestObject { taskDescription: task_description };
                let response = create_task_controller.apply(request);

                match response {
                    Ok(_) => (),
                    Err(error) => match error {
                        CreateTaskErrorModel::TaskDescriptionLengthUnderflow {
                            actualLength: actual_length,
                            minLengthRequired: min_length_required,
                        } => io_gateway.write_line(
                            &format!("Error: Task description must be at least {} characters long, yours only has {}.",
                            min_length_required, actual_length,
                        )),
                        CreateTaskErrorModel::TaskDescriptionLengthOverflow {
                            actualLength: actual_length,
                            maxLengthAllowed: max_length_allowed,
                        } => io_gateway.write_line(&format!("Error: Task description must be at most {max_length_allowed} characters long, yours has {actual_length}.")),
                    },
                }
            },
            "2" => {
                let request = ViewTasksRequestObject {
                    paginationRequest: PaginationRequest {
                        pageNumber: 1,
                        maxPageSize: usize::MAX,
                    },
                };
                let response = view_tasks_controller.apply(request);

                match response {
                    Ok(ViewTasksViewModel {
                        tasks,
                        page_size,
                        page_number,
                        max_page_number,
                        ..
                    }) => {
                        io_gateway.write_line(&format!(
                            "Page {page_number} of {max_page_number}, found {page_size} tasks:"
                        ));
                        tasks.into_iter().for_each(
                            |ViewableTask {
                                 description,
                                 created_at,
                                 ..
                             }| {
                                io_gateway.write_line(&format!(" - [ {created_at}] {description}"));
                            },
                        );
                    },
                    Err(_) => (),
                }
            },
            _ => {
                io_gateway.write_line("Invalid number.");
                continue;
            },
        }
    }
}
