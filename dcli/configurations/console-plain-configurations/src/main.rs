use clap::builder::Styles;
use clap::value_parser;
use clap::Arg;
use clap::Command;
use gateways_impl::pointers::handles::PointerHandleWithStrategy;
use gateways_impl::pointers::strategies::std::RcRefCellPointerStrategy;
use rest_api_adapters::client::interactors::tasks::CreateTaskRemoteInteractor;
use rest_api_adapters::client::interactors::tasks::ViewTasksRemoteInteractor;
use ureq::Agent;
use use_cases::boundaries::tasks::CreateTaskBoundary;
use use_cases::boundaries::tasks::CreateTaskErrResponseModel;
use use_cases::boundaries::tasks::CreateTaskRequestModel;
use use_cases::boundaries::tasks::ViewTasksBoundary;
use use_cases::boundaries::tasks::ViewTasksRequestModel;
use use_cases::dataclasses::pagination::PaginationRequest;
use use_cases::dataclasses::pagination::MIN_PAGE_NUMBER;
use use_cases::dataclasses::tasks::TaskStatusModel;
use use_cases::gateways::pointers::SharedPointer as SharedPointer_;

fn main() {
    type SharedPointer<T> = SharedPointer_<T, PointerHandle>;
    type PointerHandle = PointerHandleWithStrategy<PointerStrategy>;
    type PointerStrategy = RcRefCellPointerStrategy;

    let agent: SharedPointer<Agent> =
        SharedPointer::new(Agent::config_builder()
            .build()
            .into());

    let create_task_interactor: SharedPointer<Box<dyn CreateTaskBoundary>> =
        SharedPointer::new(Box::new(CreateTaskRemoteInteractor::new(agent.clone(), "http://127.0.0.1:4444/task/create")));
    let view_tasks_interactor: SharedPointer<Box<dyn ViewTasksBoundary>> =
        SharedPointer::new(Box::new(ViewTasksRemoteInteractor::new(agent.clone(), "http://127.0.0.1:4444/task/view")));

    let command = Command::new("dcli")
        .bin_name("dcli")
        .styles(Styles::default())
        .subcommand(Command::new("task")
            .subcommand(Command::new("create")
                .arg(Arg::new("task-description")
                    .long("task-description")
                    .short('d')
                    .value_parser(value_parser!(String))))
            .subcommand(Command::new("view")
                .arg(Arg::new("page-number")
                    .long("page-number")
                    .short('p')
                    .value_parser(value_parser!(usize)))));

    match command.get_matches().subcommand() {
        Some(("task", matches)) => match matches.subcommand() {
            Some(("create", matches)) => {
                let task_description = matches.get_one::<String>("task-description")
                    .expect("Error: Missing required argument `task-description`");

                let request = CreateTaskRequestModel {
                    task_description: task_description.to_owned(),
                };
                let response = create_task_interactor.as_ref().apply(request);
                
                match response {
                    Ok(_) => {},
                    Err(response) => match response {
                        CreateTaskErrResponseModel::TaskDescriptionLengthUnderflow {
                            actual_length,
                            min_length_required,
                        } => {
                            println!("Error: Expected task description length >= {}, found {}.", actual_length, min_length_required);
                        },
                        CreateTaskErrResponseModel::TaskDescriptionLengthOverflow {
                            actual_length,
                            max_length_allowed,
                        } => {
                            println!("Error: Expected task description length <= {}, found {}.", actual_length, max_length_allowed);
                        },
                    },
                }
            },
            Some(("view", matches)) => {
                let page_number = matches.get_one::<usize>("page-number")
                    .cloned()
                    .unwrap_or(MIN_PAGE_NUMBER);
                
                let request = ViewTasksRequestModel {
                    pagination_request: PaginationRequest {
                        page_number,
                        max_page_size: MAX_PAGE_SIZE,
                    },
                };
                let response = view_tasks_interactor.as_ref().apply(request);

                match response.into() {
                    Ok(response) => {
                        let pagination_response = response.pagination_response;
                        println!("Page {} of {} ...", pagination_response.page_number, pagination_response.max_page_number);
                        pagination_response.items
                            .iter()
                            .for_each(|task| {
                                let task_status = match task.status {
                                    TaskStatusModel::Pending => "pending",
                                    TaskStatusModel::InProgress => "in-progress",
                                    TaskStatusModel::Completed => "completed",
                                };
                                println!("- [{}] ({}) ({})", task.id, task.created_at, task_status);
                                println!("   {}", task.description);
                            });
                    },
                    Err(_) => {},
                }
            },
            _ => {},
        },
        _ => {},
    }

    const MAX_PAGE_SIZE: usize = 10;
}
