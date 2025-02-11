use domain::Task;
use domain::TaskId;
use fake::Faker;
use fake::Fake;
use use_cases::gateways::repositories::tasks::TaskRepository;

use crate::utils::tasks::MockTaskId;

use super::*;

#[test]
fn GivenRepositoryWithZeroTasks_WhenGettingAny_ExpectNone() {
    let task_repository = GivenRepositoryWithZeroTasks();
    let task = WhenGettingAny(&task_repository);        
    ExpectNone(task);
}

fn WhenGettingAny(task_repository: &impl TaskRepository) -> Option<Task> {
    let task_id = Faker.fake::<MockTaskId>().into();
    let task = WhenGettingTaskWithId(task_repository, task_id);

    return task;
}

fn WhenGettingTaskWithId(task_repository: &impl TaskRepository, task_id: TaskId) -> Option<Task> {
    return task_repository.get(task_id);
}

fn ExpectNone(task: Option<Task>) {
    assert!(task.is_none());
}

#[test]
fn GivenRepositoryWithOneTask_WhenGettingIt_ExpectCorrectTask() {
    let (task_repository, given_task) = GivenRepositoryWithOneTask();
    let retrieved_task = WhenGettingTaskWithId(&task_repository, given_task.id);
    ExpectCorrectTask(retrieved_task, given_task.id);
}

fn ExpectCorrectTask(retrieved_task: Option<Task>, given_task_id: TaskId) {
    assert!(retrieved_task.is_some());
    assert_eq!(retrieved_task.unwrap().id, given_task_id);
}

#[test]
fn GivenRepositoryWithOneTask_WhenGettingAnyOther_ExpectNone() {
    let (task_repository, given_task) = GivenRepositoryWithOneTask();
    let retrieved_task = WhenGettingAnyOther(&task_repository, given_task.id);
    ExpectNone(retrieved_task);
}

fn WhenGettingAnyOther(task_repository: &impl TaskRepository, given_task_id: TaskId) -> Option<Task> {
    let any_other_task_id = loop {
        let task_id = Faker.fake::<MockTaskId>().into();
        
        if task_id != given_task_id {
            break task_id;
        }
    };

    return WhenGettingTaskWithId(task_repository, any_other_task_id);
}

#[test]
fn GivenRepositoryWithManyTasks_WhenGettingOneAmongThem_ExpectCorrectTask() {
    let (task_repository, given_tasks) = GivenRepositoryWithManyTasks::<NUMBER_OF_TASKS>();
    let given_task_ids: [_; NUMBER_OF_TASKS] = std::array::from_fn(|index|
        given_tasks[index].id);
    let (retrieved_task, given_task_id) = WhenGettingOneAmongThem(&task_repository, given_task_ids);
    ExpectCorrectTask(retrieved_task, given_task_id);
}

fn WhenGettingOneAmongThem<const N: usize>(task_repository: &impl TaskRepository, given_task_ids: [TaskId; N]) -> (Option<Task>, TaskId) {
    let one_given_task_id = given_task_ids.first()
        .unwrap()
        .clone();
    let retrieved_task = WhenGettingTaskWithId(task_repository, one_given_task_id);
    return (retrieved_task, one_given_task_id);
}

#[test]
fn GivenRepositoryWithManyTasks_WhenGettingOneNotAmongThem_ExpectNone() {
    let (task_repository, given_tasks) = GivenRepositoryWithManyTasks::<NUMBER_OF_TASKS>();
    let given_task_ids: [_; NUMBER_OF_TASKS] = std::array::from_fn(|index|
        given_tasks[index].id);
    let (retrieved_task, _) = WhenGettingOneNotAmongThem(&task_repository, given_task_ids);
    ExpectNone(retrieved_task);
}

fn WhenGettingOneNotAmongThem<const N: usize>(task_repository: &impl TaskRepository, given_task_ids: [TaskId; N]) -> (Option<Task>, TaskId) {
    let any_other_task_id = loop {
        let task_id = Faker.fake::<MockTaskId>().into();
        
        if !given_task_ids.contains(&task_id) {
            break task_id;
        }
    };

    let retrieved_task = WhenGettingTaskWithId(task_repository, any_other_task_id);
    return (retrieved_task, any_other_task_id);
}
