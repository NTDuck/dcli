namespace_mod!(core);

use crate::core::domain::{utils::UUID, Task, TaskIdentifier};
use crate::core::dataproviders::InMemoryTaskRepository;

use ddd::{Entity, ReadRepository, Repository};
use layout::namespace_mod;

fn main() {
    let task_repository = InMemoryTaskRepository::new();
    let task = create_task();
    
    task_repository.save(task.clone());
    assert!(task_repository.contains(task.get_id().clone()));
    assert!(task_repository.get_by_id(task.get_id().clone()).is_some());
    assert!(task_repository.size() == 1);
    assert!(task_repository.show(0, 1) == vec![task.clone()]);
    
    task_repository.delete(task.get_id().clone());
    assert!(!task_repository.contains(task.get_id().clone()));
    assert!(task_repository.get_by_id(task.clone().get_id().clone()).is_none());
    assert!(task_repository.size() == 0);
    assert!(task_repository.show(0, 1) == vec![]);

    println!("Hello from tomfoolery!");
}

fn create_task() -> core::domain::Task {
    let id = TaskIdentifier(UUID(4));

    return Task {
        identifier: id,
        description: String::from("tomfoolery"),
        is_active: true,
    };
}
