#![allow(non_snake_case)]

use std::usize;

use console::utils::io::IoGateway;
use gateways::factories::ids::UuidV4Factory;
use gateways::pointers::handles::StrategizedPointerHandle;
use gateways::pointers::strategies::triomphe::ArcRwLockSharedPointerStrategy;
use gateways::repositories::inmemory::tasks::InMemoryTaskRepository;
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
    let taskRepository: SharedPointer<Box<dyn TaskRepository>> =
        SharedPointer::new(Box::new(InMemoryTaskRepository::new()));
    let uuidFactory: SharedPointer<Box<dyn UuidFactory>> =
        SharedPointer::new(Box::new(UuidV4Factory::new()));

    // Interactors
    let createTaskInteractor =
        CreateTaskInteractor::new(taskRepository.clone(), uuidFactory.clone());
    let viewTasksInteractor = 
        ViewTasksInteractor::new(taskRepository.clone());

    // Controllers
    let createTaskController = CreateTaskController::new(&createTaskInteractor);
    let viewTasksController = ViewTasksController::new(&viewTasksInteractor);

    // I/O
    let ioGateway = IoGateway;

    // Main loop
    loop {
        ioGateway.write(
            "\
            Select a number:\n\
            [0] Exit\n\
            [1] Create a task\n\
            [2] View all tasks\n \
        ",
        );

        match ioGateway.readLine().trim() {
            "0" => {
                ioGateway.writeLine("Exit signal received.");
                break;
            },
            "1" => {
                ioGateway.write("Enter task description: ");
                let task_description = ioGateway.readLine();

                let request = CreateTaskRequestObject { taskDescription: task_description };
                let response = createTaskController.apply(request);

                match response {
                    Ok(_) => (),
                    Err(error) => match error {
                        CreateTaskErrorModel::TaskDescriptionLengthUnderflow {
                            actualLength,
                            minLengthRequired,
                        } => ioGateway.writeLine(
                            &format!("Error: Task description must be at least {} characters long, yours only has {}.",
                            minLengthRequired, actualLength,
                        )),
                        CreateTaskErrorModel::TaskDescriptionLengthOverflow {
                            actualLength,
                            maxLengthAllowed,
                        } => ioGateway.writeLine(&format!("Error: Task description must be at most {maxLengthAllowed} characters long, yours has {actualLength}.")),
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
                let response = viewTasksController.apply(request);

                match response {
                    Ok(ViewTasksViewModel {
                        tasks,
                        pageSize,
                        pageNumber,
                        maxPageNumber,
                        ..
                    }) => {
                        ioGateway.writeLine(&format!(
                            "Page {pageNumber} of {maxPageNumber}, found {pageSize} tasks:"
                        ));
                        tasks.into_iter().for_each(
                            |ViewableTask {
                                 description,
                                 createdAt: created_at,
                                 ..
                             }| {
                                ioGateway.writeLine(&format!(" - [ {created_at}] {description}"));
                            },
                        );
                    },
                    Err(_) => (),
                }
            },
            _ => {
                ioGateway.writeLine("Invalid number.");
                continue;
            },
        }
    }
}
