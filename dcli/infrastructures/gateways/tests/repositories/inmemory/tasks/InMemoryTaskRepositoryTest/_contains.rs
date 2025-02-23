use domain::utils::dataclasses::ids::Uuid;
use use_cases::gateways::repositories::tasks::TaskRepository;

use crate::repositories::inmemory::tasks::InMemoryTaskRepositoryTest::*;

#[test]
fn GivenRepositoryContainingZeroTasks_WhenCheckingIfContainsAny_ExpectFalse() {
    let taskRepository = GivenRepositoryContainingZeroTasks();
    let exists = WhenCheckingIfContainsId(&taskRepository, 1234);
    ExpectFalse(exists);
}

#[test]
fn GivenRepositoryContainingOneTask_WhenCheckingIfContainsIt_ExpectTrue() {
    let taskRepository = GivenRepositoryContainingOneTaskWithId(0);
    let exists = WhenCheckingIfContainsId(&taskRepository, 0);
    ExpectTrue(exists);
}

#[test]
fn GivenRepositoryContainingOneTask_WhenCheckingIfContainsAnyOther_ExpectFalse() {
    let taskRepository = GivenRepositoryContainingOneTaskWithId(0);
    let exists = WhenCheckingIfContainsId(&taskRepository, 1234);
    ExpectFalse(exists);
}

#[test]
fn GivenRepositoryContainingManyTasks_WhenCheckingIfContainsAnExistingOne_ExpectTrue() {
    let taskRepository = GivenRepositoryContainingManyTasksWithIds([0, 1, 2, 3, 4]);
    let exists = WhenCheckingIfContainsId(&taskRepository, 0);
    ExpectTrue(exists);
}

#[test]
fn GivenRepositoryContainingManyTasks_WhenCheckingIfContainsANotExistingOne_ExpectFalse() {
    let taskRepository = GivenRepositoryContainingManyTasksWithIds([0, 1, 2, 3, 4]);
    let exists = WhenCheckingIfContainsId(&taskRepository, 1234);
    ExpectFalse(exists);
}

fn WhenCheckingIfContainsId(taskRepository: &impl TaskRepository, taskId: u128) -> bool {
    let taskId = Uuid::new(taskId);
    return taskRepository.contains(taskId);
}
