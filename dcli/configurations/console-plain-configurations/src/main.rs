use clap::builder::Styles;
use clap::Arg;
use clap::Command;

fn main() {
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

    println!("dcli :3");

    match command.get_matches().subcommand() {
        Some(("task", matches)) => match matches.subcommand() {
            Some(("create", matches)) => {
                let task_description = matches.get_one::<String>("task-description");
                println!("{:?}", task_description);
            },
            Some(("view", matches)) => {
                let page_number = matches.get_one::<usize>("page-number");
                println!("{:?}", page_number);
            },
            _ => {},
        },
        _ => {},
    }
}
