use clap::builder::Styles;
use clap::Arg;
use clap::Command;
use models::pagination::MIN_PAGE_NUMBER;
use models::tasks::TaskStatusModel;
use rest_api_adapters::models::tasks::CreateTaskErrViewModel;
use rest_api_adapters::models::tasks::CreateTaskRequestObject;
use rest_api_adapters::models::tasks::CreateTaskViewModel;
use rest_api_adapters::models::tasks::ViewTasksRequestObject;
use rest_api_adapters::models::tasks::ViewTasksViewModel;
use ureq::Agent;

fn main() {
    let agent: Agent = Agent::config_builder()
        .build()
        .into();

    let command = Command::new("dcli")
        .bin_name("dcli")
        .styles(Styles::default())
        .subcommand(Command::new("task")
            .subcommand(Command::new("create")
                .arg(Arg::new("task-description")
                    .long("task-description")
                    .short('d')))
            .subcommand(Command::new("view")
                .arg(Arg::new("page-number")
                    .long("page-number")
                    .short('p'))));

    match command.get_matches().subcommand() {
        Some(("task", matches)) => match matches.subcommand() {
            Some(("create", matches)) => {
                let task_description = matches.get_one::<String>("task-description")
                    .expect("Error: Missing required argument `task-description`");

                let request = CreateTaskRequestObject {
                    task_description: task_description.to_owned(),
                };
                let query = serde_qs::to_string(&request).unwrap();
                println!("url is {}", format!("{}/task/create?{}", ROOT_URI, query));
                let response = agent.get(format!("{}/task/create?{}", ROOT_URI, query))
                    .call().unwrap()
                    .body_mut()
                    .read_json::<CreateTaskViewModel>().unwrap();
                
                match response.into() {
                    Ok(_) => {},
                    Err(response) => match response {
                        CreateTaskErrViewModel::TaskDescriptionLengthUnderflow {
                            actual_length,
                            min_length_required,
                        } => {
                            println!("Error: Expected task description length >= {}, found {}.", actual_length, min_length_required);
                        },
                        CreateTaskErrViewModel::TaskDescriptionLengthOverflow {
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
                
                let request = ViewTasksRequestObject {
                    page_number,
                    max_page_size: MAX_PAGE_SIZE,
                };
                let query = serde_qs::to_string(&request).unwrap();
                let response = agent.get(format!("{}/task/view?{}", ROOT_URI, query))
                    .call().unwrap()
                    .body_mut()
                    .read_json::<ViewTasksViewModel>().unwrap();

                match response.into() {
                    Ok(response) => {
                        println!("Page {} of {} ...", response.page_number, response.max_page_number);
                        response.tasks
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

    const ROOT_URI: &str = "http://127.0.0.1:4444";
    const MAX_PAGE_SIZE: usize = 10;
}
