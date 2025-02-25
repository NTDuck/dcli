use domain::tasks::Task;
use domain::tasks::TaskDescription;
use domain::tasks::TaskId;
use domain::tasks::TaskStatus;
use domain::time::Epoch;
use domain::time::Interval;
use gateways::repositories::inmemory::tasks::InMemoryTaskRepository;
use use_cases::gateways::repositories::tasks::TaskRepository;

#[test]
fn GivenEmptyRepository_WhenSaveTask_ThenContainsTask() {
    let mut repository = InMemoryTaskRepository::new();

    repository.save(newTaskWithId(42));
    
    assert!(repository.contains(newTaskId(42)));
}

#[test]
fn GivenRepositoryWithTask_WhenRemoveTask_ThenDoesNotContainTask() {
    let mut repository = InMemoryTaskRepository::new();
    repository.save(newTaskWithId(42));

    repository.remove(newTaskId(42));

    assert!(!repository.contains(newTaskId(42)));
}

#[test]
fn GivenRepositoryWithTask_WhenGetById_ThenReturnsCorrectTask() {
    let mut repository = InMemoryTaskRepository::new();
    repository.save(newTaskWithId(42));

    let retrievedTask = repository.getById(newTaskId(42));

    assert!(retrievedTask.is_some());
    assert_eq!(retrievedTask.unwrap(), newTaskWithId(42));
}

#[test]
fn GivenEmptyRepository_WhenGetById_ThenReturnsNone() {
    let repository = InMemoryTaskRepository::new();

    let retrievedTask = repository.getById(newTaskId(42));

    assert!(retrievedTask.is_none());
}

#[test]
fn GivenRepositoryWithMultipleTasks_WhenClear_ThenRepositoryIsEmpty() {
    let mut repository = InMemoryTaskRepository::new();
    repository.save(newTaskWithId(42));
    repository.save(newTaskWithId(43));

    repository.clear();

    assert!(!repository.contains(newTaskId(42)));
    assert!(!repository.contains(newTaskId(43)));
}

#[test]
fn GivenRepositoryWithDifferentStatuses_WhenClearByStatus_ThenRemovesOnlyMatchingStatus() {
    let mut repository = InMemoryTaskRepository::new();
    repository.save(newTaskWithIdAndStatus(42, TaskStatus::Pending));
    repository.save(newTaskWithIdAndStatus(43, TaskStatus::Pending));
    repository.save(newTaskWithIdAndStatus(123, TaskStatus::InProgress));
    repository.save(newTaskWithIdAndStatus(124, TaskStatus::InProgress));
    repository.save(newTaskWithIdAndStatus(125, TaskStatus::InProgress));
    repository.save(newTaskWithIdAndStatus(1234, TaskStatus::Completed));
    repository.save(newTaskWithIdAndStatus(1235, TaskStatus::Completed));
    repository.save(newTaskWithIdAndStatus(1236, TaskStatus::Completed));
    repository.save(newTaskWithIdAndStatus(1237, TaskStatus::Completed));
    
    repository.clearByStatus(TaskStatus::Completed);

    assert!(repository.contains(newTaskId(42)));
    assert!(repository.contains(newTaskId(43)));
    assert!(repository.contains(newTaskId(123)));
    assert!(repository.contains(newTaskId(124)));
    assert!(repository.contains(newTaskId(125)));
    assert!(!repository.contains(newTaskId(1234)));
    assert!(!repository.contains(newTaskId(1235)));
    assert!(!repository.contains(newTaskId(1236)));
    assert!(!repository.contains(newTaskId(1237)));
}

fn newTaskWithId(value: u64) -> Task {
    return newTaskWithIdAndStatus(value, TaskStatus::Pending);
}

fn newTaskWithIdAndStatus(value: u64, status: TaskStatus) -> Task {
    return Task {
        id: newTaskId(value),
        description: TaskDescription::try_from("tomfoolery".to_owned())
            .expect("Invalid description length"),
        status,
    };
}

fn newTaskId(value: u64) -> TaskId {
    return TaskId::new(
        Epoch.checkedAdd(Interval::fromSeconds(value))
            .expect("Timestamp earlier than Epoch"),
        Default::default(), Default::default(),
    );
}
