use domain::utils::dataclasses::ids::Uuid;
use domain::Task;
use domain::TaskId;
use use_cases::gateways::repositories::tasks::TaskRepository;

use crate::repositories::inmemory::tasks::OrderedInMemoryTaskRepositoryTest::*;

#[test]
fn GivenRepositoryContainingZeroTasks_WhenGettingAny_ExpectNone() {
    let taskRepository = GivenRepositoryContainingZeroTasks();
    let retrievedTask = WhenGettingAny(&taskRepository);
    ExpectNone(retrievedTask);
}

#[test]
fn GivenRepositoryContainingOneTask_WhenGettingIt_ExpectCorrectTask() {
    let givenTaskId = Uuid::from(0);
    let taskRepository = GivenRepositoryContainingOneTaskWithId(givenTaskId);
    let retrievedTask = WhenGettingById(&taskRepository, givenTaskId);
    ExpectCorrectTask(retrievedTask, givenTaskId);
}

#[test]
fn GivenRepositoryContainingOneTask_WhenGettingAnyOther_ExpectNone() {
    let givenTaskId = Uuid::from(0);
    let taskRepository = GivenRepositoryContainingOneTaskWithId(givenTaskId);

    let anotherTaskId = Uuid::from(1);
    let retrievedTask = WhenGettingById(&taskRepository, anotherTaskId);

    ExpectNone(retrievedTask);
}

#[test]
fn GivenRepositoryContainingManyTasks_WhenGettingAnExistingOne_ExpectCorrectTask() {
    let givenTaskIds = [0, 1, 2, 3, 4];
    let givenTaskIds: [_; 5] =
        std::array::from_fn(|index| TaskId::from(givenTaskIds[index]));
    let taskRepository = GivenRepositoryContainingManyTasksWithIds(givenTaskIds);

    let existingTaskId = TaskId::from(0);
    let retrievedTask = WhenGettingById(&taskRepository, existingTaskId);

    ExpectCorrectTask(retrievedTask, existingTaskId);
}

#[test]
fn GivenRepositoryContainingManyTasks_WhenGettingANotExistingOne_ExpectNone() {
    let givenTaskIds = [0, 1, 2, 3, 4];
    let givenTaskIds: [_; 5] =
        std::array::from_fn(|index| TaskId::from(givenTaskIds[index]));
    let taskRepository = GivenRepositoryContainingManyTasksWithIds(givenTaskIds);

    let notExistingTaskId = TaskId::from(6);
    let retrievedTask = WhenGettingById(&taskRepository, notExistingTaskId);

    ExpectNone(retrievedTask);
}

fn WhenGettingAny(taskRepository: &impl TaskRepository) -> Option<Task> {
    return WhenGettingById(taskRepository, mockTaskId());
}

fn WhenGettingById(taskRepository: &impl TaskRepository, taskId: TaskId) -> Option<Task> {
    return taskRepository.getById(taskId);
}

fn ExpectNone(retrievedTask: Option<Task>) {
    assert!(retrievedTask.is_none());
}

fn ExpectCorrectTask(retrievedTask: Option<Task>, givenTaskId: TaskId) {
    assert!(retrievedTask.is_some());
    assert_eq!(retrievedTask.unwrap().id, givenTaskId);
}
