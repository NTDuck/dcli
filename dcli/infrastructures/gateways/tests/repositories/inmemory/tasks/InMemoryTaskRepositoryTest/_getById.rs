use domain::{utils::dataclasses::ids::Uuid, Task};
use use_cases::gateways::repositories::tasks::TaskRepository;

use crate::repositories::inmemory::tasks::InMemoryTaskRepositoryTest::*;

#[test]
fn GivenRepositoryContainingZeroTasks_WhenGettingAny_ExpectNone() {
    let taskRepository = GivenRepositoryContainingZeroTasks();
    let retrievedTask = WhenGettingTaskById(&taskRepository, 1234);
    ExpectNone(retrievedTask);
}

#[test]
fn GivenRepositoryContainingOneTask_WhenGettingIt_ExpectCorrectTask() {
    let taskRepository = GivenRepositoryContainingOneTaskWithId(0);
    let retrievedTask = WhenGettingTaskById(&taskRepository, 0);
    ExpectCorrectTask(retrievedTask, 0);
}

#[test]
fn GivenRepositoryContainingOneTask_WhenGettingAnyOther_ExpectNone() {
    let taskRepository = GivenRepositoryContainingOneTaskWithId(0);
    let retrievedTask = WhenGettingTaskById(&taskRepository, 1234);
    ExpectNone(retrievedTask);
}

#[test]
fn GivenRepositoryContainingManyTasks_WhenGettingAnExistingOne_ExpectCorrectTask() {
    let taskRepository = GivenRepositoryContainingManyTasksWithIds([0, 1, 2, 3, 4]);
    let retrievedTask = WhenGettingTaskById(&taskRepository, 0);
    ExpectCorrectTask(retrievedTask, 0);
}

#[test]
fn GivenRepositoryContainingManyTasks_WhenGettingANotExistingOne_ExpectNone() {
    let taskRepository = GivenRepositoryContainingManyTasksWithIds([0, 1, 2, 3, 4]);
    let retrievedTask = WhenGettingTaskById(&taskRepository, 1234);
    ExpectNone(retrievedTask);
}

fn WhenGettingTaskById(taskRepository: &impl TaskRepository, taskId: u128) -> Option<Task> {
    let taskId = Uuid::new(taskId);
    return taskRepository.getById(taskId);
}
