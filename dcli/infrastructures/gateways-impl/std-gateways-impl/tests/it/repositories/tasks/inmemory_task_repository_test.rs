use domain::tasks::Task;
use domain::tasks::TaskDescription;
use domain::tasks::TaskId;
use domain::tasks::TaskStatus;
use domain::time::Timestamp;
use gateways::repositories::tasks::TaskRepository;

use std_gateways_impl::repositories::tasks::InMemoryTaskRepository;

#[test]
fn given_empty_repository_when_save_task_then_contains_task() {
    let mut repository = InMemoryTaskRepository::new();

    repository.save(new_task_with_id(42));
    
    assert!(repository.contains(new_task_id(42)));
}

#[test]
fn given_repository_with_task_when_remove_task_then_does_not_contain_task() {
    let mut repository = InMemoryTaskRepository::new();
    repository.save(new_task_with_id(42));

    repository.remove(new_task_id(42));

    assert!(!repository.contains(new_task_id(42)));
}

#[test]
fn given_repository_with_task_when_get_by_id_then_returns_correct_task() {
    let mut repository = InMemoryTaskRepository::new();
    repository.save(new_task_with_id(42));

    let retrieved_task = repository.get_by_id(new_task_id(42));

    assert!(retrieved_task.is_some());
    assert_eq!(retrieved_task.unwrap(), new_task_with_id(42));
}

#[test]
fn given_empty_repository_when_get_by_id_then_returns_none() {
    let repository = InMemoryTaskRepository::new();

    let retrieved_task = repository.get_by_id(new_task_id(42));

    assert!(retrieved_task.is_none());
}

#[test]
fn given_repository_with_multiple_tasks_when_clear_then_repository_is_empty() {
    let mut repository = InMemoryTaskRepository::new();
    repository.save(new_task_with_id(42));
    repository.save(new_task_with_id(43));

    repository.clear();

    assert!(!repository.contains(new_task_id(42)));
    assert!(!repository.contains(new_task_id(43)));
}

#[test]
fn given_repository_with_different_statuses_when_clear_by_status_then_removes_only_matching_status() {
    let mut repository = InMemoryTaskRepository::new();
    repository.save(new_task_with_id_and_status(42, TaskStatus::Pending));
    repository.save(new_task_with_id_and_status(43, TaskStatus::Pending));
    repository.save(new_task_with_id_and_status(123, TaskStatus::InProgress));
    repository.save(new_task_with_id_and_status(124, TaskStatus::InProgress));
    repository.save(new_task_with_id_and_status(125, TaskStatus::InProgress));
    repository.save(new_task_with_id_and_status(1234, TaskStatus::Completed));
    repository.save(new_task_with_id_and_status(1235, TaskStatus::Completed));
    repository.save(new_task_with_id_and_status(1236, TaskStatus::Completed));
    repository.save(new_task_with_id_and_status(1237, TaskStatus::Completed));
    
    repository.clear_by_status(TaskStatus::Completed);

    assert!(repository.contains(new_task_id(42)));
    assert!(repository.contains(new_task_id(43)));
    assert!(repository.contains(new_task_id(123)));
    assert!(repository.contains(new_task_id(124)));
    assert!(repository.contains(new_task_id(125)));
    assert!(!repository.contains(new_task_id(1234)));
    assert!(!repository.contains(new_task_id(1235)));
    assert!(!repository.contains(new_task_id(1236)));
    assert!(!repository.contains(new_task_id(1237)));
}

fn new_task_with_id(id: u64) -> Task {
    return new_task_with_id_and_status(id, TaskStatus::Pending);
}

fn new_task_with_id_and_status(id: u64, status: TaskStatus) -> Task {
    return Task {
        id: new_task_id(id),
        description: TaskDescription::try_from("tomfoolery".to_owned())
            .expect("Invalid description length"),
        status,
    };
}

fn new_task_id(id: u64) -> TaskId {
    return TaskId::new(
        Timestamp::from_millis_since_epoch(id as i64),
        Default::default(), Default::default(),
    );
}
