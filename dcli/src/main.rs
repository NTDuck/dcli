#[allow(non_snake_case)]
mod core;

use core::domain::{Task, TaskIdentifier};

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
