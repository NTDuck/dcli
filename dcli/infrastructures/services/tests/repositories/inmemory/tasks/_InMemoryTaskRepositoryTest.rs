use domain::tasks::Task;
use domain::tasks::TaskDescription;
use domain::tasks::TaskId;
use domain::tasks::TaskStatus;
use domain::time::Timestamp;
use gateways::repositories::inmemory::tasks::InMemoryTaskRepository;
use use_cases::gateways::repositories::tasks::TaskRepository;

#[test]
fn GivenEmptyRepository_WhenSaveTask_ThenContainsTask() {
    let mut repository = InMemoryTaskRepository::new();

    repository.save(new_task_with_id(42));
    
    assert!(repository.contains(new_task_id(42)));
}

#[test]
fn GivenRepositoryWithTask_WhenRemoveTask_ThenDoesNotContainTask() {
    let mut repository = InMemoryTaskRepository::new();
    repository.save(new_task_with_id(42));

    repository.remove(new_task_id(42));

    assert!(!repository.contains(new_task_id(42)));
}

#[test]
fn GivenRepositoryWithTask_WhenGetById_ThenReturnsCorrectTask() {
    let mut repository = InMemoryTaskRepository::new();
    repository.save(new_task_with_id(42));

    let retrievedTask = repository.get_by_id(new_task_id(42));

    assert!(retrievedTask.is_some());
    assert_eq!(retrievedTask.unwrap(), new_task_with_id(42));
}

#[test]
fn GivenEmptyRepository_WhenGetById_ThenReturnsNone() {
    let repository = InMemoryTaskRepository::new();

    let retrieved_task = repository.get_by_id(new_task_id(42));

    assert!(retrieved_task.is_none());
}

#[test]
fn GivenRepositoryWithMultipleTasks_WhenClear_ThenRepositoryIsEmpty() {
    let mut repository = InMemoryTaskRepository::new();
    repository.save(new_task_with_id(42));
    repository.save(new_task_with_id(43));

    repository.clear();

    assert!(!repository.contains(new_task_id(42)));
    assert!(!repository.contains(new_task_id(43)));
}

#[test]
fn GivenRepositoryWithDifferentStatuses_WhenClearByStatus_ThenRemovesOnlyMatchingStatus() {
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
