use std::fmt::Debug;
use std::str::FromStr;

use cucumber::given;
use cucumber::then;
use cucumber::when;
use domain::tasks::Task;
use domain::tasks::TaskDescription;
use domain::tasks::TaskId;
use domain::tasks::TaskStatus;
use gateways_impl::repositories::tasks::InMemoryTaskRepository;
use use_cases::dataclasses::pagination::MIN_PAGE_NUMBER;
use use_cases::dataclasses::pagination::UNBOUNDED_PAGINATION_REQUEST;
use use_cases::gateways::repositories::tasks::TaskRepository;
use rayon::prelude::*;

use crate::utils::parameters::ids::SnowflakeParameter;
use crate::steps::repositories::tasks::TaskRepositoryWorld;
use crate::steps::repositories::tasks::TaskRepositoryWorldHandle;

#[given(expr = "a repository with {int} task(s)")]
pub fn setup<Handle: TaskRepositoryWorldHandle>(world: &mut TaskRepositoryWorld<Handle>, task_count: usize) {
    add(world, task_count);
}

#[when(expr = "adding {int} task(s)")]
pub fn add<Handle: TaskRepositoryWorldHandle>(world: &mut TaskRepositoryWorld<Handle>, task_count: usize) {
    (0..task_count)
        .map(|i| (i + world.task_count).to_string())
        .map(|s| SnowflakeParameter::from_str(&s).unwrap().into())
        .map(|snowflake| new_task_with_id(snowflake))
        .for_each(|task| world.task_repository.save(task));

    world.task_count += task_count;
}

#[when(expr = "removing {int} task(s)")]
pub fn remove<Handle: TaskRepositoryWorldHandle>(world: &mut TaskRepositoryWorld<Handle>, task_count: usize) {
    (0..task_count)
        .map(|i| (i + world.task_count).to_string())
        .map(|s| SnowflakeParameter::from_str(&s).unwrap().into())
        .for_each(|task_id| world.task_repository.remove(task_id));

    world.task_count -= task_count;
}

#[when(expr = "clearing all tasks")]
pub fn clear<Handle: TaskRepositoryWorldHandle>(world: &mut TaskRepositoryWorld<Handle>) {
    world.task_repository.clear();
    world.task_count = 0;
}

#[then(expr = "the repository has {int} task(s)")]
pub fn has<Handle: TaskRepositoryWorldHandle>(world: &mut TaskRepositoryWorld<Handle>, task_count: usize) {
    assert_eq!(world.task_count, task_count);

    let tasks = (0..task_count)
        .map(|i| i.to_string())
        .map(|s| SnowflakeParameter::from_str(&s).unwrap().into())
        .map(|snowflake| new_task_with_id(snowflake))
        .collect::<Vec<_>>();

    assert!(tasks
        .par_iter()
        .all(|task| world.task_repository.contains(task.id)));
    assert!(tasks
        .par_iter()
        .cloned()
        .all(|task| world.task_repository.get_by_id(task.id) == Some(task)));

    let pagination_request = UNBOUNDED_PAGINATION_REQUEST;
    let pagination_response = world.task_repository.show_reverse_chronologically_ordered(pagination_request);
    
    assert_eq!(pagination_response.items, tasks);
    assert_eq!(pagination_response.page_number, MIN_PAGE_NUMBER);
    assert_eq!(pagination_response.max_page_number, pagination_response.page_number);
    assert_eq!(pagination_response.page_size, task_count);
    assert_eq!(pagination_response.max_page_size, pagination_response.page_size);
}

fn new_task_with_id(task_id: TaskId) -> Task {
    new_task_with_id_and_status(task_id, TaskStatus::Pending)
}

fn new_task_with_id_and_status(task_id: TaskId, task_status: TaskStatus) -> Task {
    Task {
        id: task_id,
        description: TaskDescription::try_from("tomfoolery".to_owned()).unwrap(),
        status: task_status,
    }
}
