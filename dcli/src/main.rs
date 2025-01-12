namespace_mod!(core);

use crate::core::domain::{Task, TaskIdentifier};

use layout::namespace_mod;

fn main() {
    let task = create_task();
    assert!(task.is_active);

    println!(
        "Description {} of task {}",
        task.description, task.identifier.0
    );
}

fn create_task() -> core::domain::Task {
    let id = TaskIdentifier(4);

    return Task {
        identifier: id,
        description: String::from("tomfoolery"),
        is_active: true,
    };
}
