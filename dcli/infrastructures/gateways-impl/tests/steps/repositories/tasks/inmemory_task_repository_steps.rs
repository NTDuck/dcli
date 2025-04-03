use std::str::FromStr;

use cucumber::given;
use cucumber::when;
use cucumber::then;
use domain::tasks::Task;
use use_cases::dataclasses::pagination::UNBOUNDED_PAGINATION_REQUEST;
use use_cases::gateways::repositories::tasks::TaskRepository;

use crate::parameters::tasks::TaskParameter;
use crate::worlds::repositories::tasks::InMemoryTaskRepositoryWorld as World;

#[given("an empty repository")]
pub fn given_empty(_world: &mut World) {
    
}

#[given(expr = "a repository containing tasks {task} to {task}")]
pub fn given_task_range(world: &mut World, task_start: TaskParameter, task_end: TaskParameter) {
    let idx_start = Task::from(task_start).id.to_u64();
    let idx_end = Task::from(task_end).id.to_u64();

    (idx_start..=idx_end)
        .map(|idx| idx.to_string())
        .map(|s| TaskParameter::from_str(&s).unwrap().into())
        .for_each(|task| world.task_repository.save(task));
}

#[when(expr = "adding task {task}")]
pub fn when_adding_task(world: &mut World, task: TaskParameter) {
    let task = Task::from(task);
    world.task_repository.save(task);
}

#[when(expr = "removing task {task}")]
pub fn when_removing_task(world: &mut World, task: TaskParameter) {
    let task = Task::from(task);
    world.task_repository.remove(task.id);
}

#[then("the repository is empty")]
pub fn then_empty(world: &mut World) {
    let pagination_request = UNBOUNDED_PAGINATION_REQUEST;
    let pagination_response = world.task_repository.show_reverse_chronologically_ordered(pagination_request);

    assert!(pagination_response.items.is_empty());
    assert!(pagination_response.page_size == 0);
}

#[then(expr = "the repository only contains task {task}")]
pub fn then_contains_only_task(world: &mut World, task: TaskParameter) {
    let task = Task::from(task);

    assert!(world.task_repository.contains(task.id));
    assert!(world.task_repository.get_by_id(task.id) == Some(task.clone()));

    let pagination_request = UNBOUNDED_PAGINATION_REQUEST;
    let pagination_response = world.task_repository.show_reverse_chronologically_ordered(pagination_request);

    assert!(pagination_response.items == vec![task]);
    assert!(pagination_response.page_size == 1);
}

#[then(expr = "the repository contains tasks {task} to {task} except {task}")]
pub fn then_contains_tasks_except(world: &mut World, task_start: TaskParameter, task_end: TaskParameter, task_except: TaskParameter) {
    let idx_start = Task::from(task_start).id.to_u64();
    let idx_end = Task::from(task_end).id.to_u64();
    let task_except = Task::from(task_except);

    let tasks = (idx_start..=idx_end)
        .map(|idx| idx.to_string())
        .map(|s| TaskParameter::from_str(&s).unwrap().into())
        .filter(|task| *task != task_except)
        .collect::<Vec<Task>>();

    assert!(tasks
        .iter()
        .map(|task| task.id)
        .all(|task_id| world.task_repository.contains(task_id)));
    assert!(tasks
        .iter()
        .cloned()
        .all(|task| world.task_repository.get_by_id(task.id) == Some(task)));

    let pagination_request = UNBOUNDED_PAGINATION_REQUEST;
    let pagination_response = world.task_repository.show_reverse_chronologically_ordered(pagination_request);

    assert!(pagination_response.items == tasks);
    assert!(pagination_response.page_size == tasks.len());
}
