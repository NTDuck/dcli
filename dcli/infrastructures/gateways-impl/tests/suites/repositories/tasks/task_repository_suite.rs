use domain::ids::Snowflake;
use domain::tasks::Task;
use domain::tasks::TaskStatus;
use domain::time::Timestamp;
use rspec::given;
use rspec::suite::Suite;
use use_cases::dataclasses::pagination::UNBOUNDED_PAGINATION_REQUEST;
use use_cases::gateways::repositories::tasks::TaskRepository;

use crate::environments::repositories::tasks::TaskRepositoryEnvironment;
use crate::environments::repositories::tasks::TaskRepositoryEnvironmentHandle;

pub struct TaskRepositorySuite;

impl TaskRepositorySuite {
    pub fn with<Handle: TaskRepositoryEnvironmentHandle + 'static>(env: TaskRepositoryEnvironment<Handle>) -> Suite<TaskRepositoryEnvironment<Handle>> {
        rspec::suite(std::any::type_name::<Handle::TaskRepository>().rsplit("::").next().unwrap(), env.clone(), |_| {
            given("an empty repository", env.clone(), |ctx| {
                ctx.before(given_empty_repository);

                ctx.when("adding task 0", |ctx| {
                    ctx.before(|env| when_adding_task(env, 0));
                    ctx.then("the repository contains task 0", |env| then_contains_only_task(env, 0));
                });
            });

            given("an empty repository", env.clone(), |ctx| {
                ctx.before(given_empty_repository);

                ctx.when("removing task 0", |ctx| {
                    ctx.then("the repository is empty", |env| then_empty(env));
                });
            });

            given("a repository containing tasks 0 to 1024", env.clone(), |ctx| {
                ctx.before(|env| given_repository_containing_tasks_range(env, 0..=1024));

                ctx.when("removing task 444", |ctx| {
                    ctx.before(|env| when_removing_task(env, 444));
                    ctx.then("the repository contains task 0 to 1024 except 444", |env| then_contains_tasks_except(env, 0..=1024, 444));
                });
            });
        })
    }
}

fn given_empty_repository<Handle: TaskRepositoryEnvironmentHandle>(_: &mut TaskRepositoryEnvironment<Handle>) {
        
}

fn given_repository_containing_tasks_range<Handle: TaskRepositoryEnvironmentHandle>(env: &mut TaskRepositoryEnvironment<Handle>, range: std::ops::RangeInclusive<usize>) {
    range
        .map(|idx| new_task_from_idx(idx))
        .for_each(|task| env.task_repository.save(task));
}

fn when_adding_task<Handle: TaskRepositoryEnvironmentHandle>(env: &mut TaskRepositoryEnvironment<Handle>, idx: usize) {
    let task = new_task_from_idx(idx);
    env.task_repository.save(task);
}

fn when_removing_task<Handle: TaskRepositoryEnvironmentHandle>(env: &mut TaskRepositoryEnvironment<Handle>, idx: usize) {
    let task = new_task_from_idx(idx);
    env.task_repository.remove(task.id);
}

fn then_empty<Handle: TaskRepositoryEnvironmentHandle>(env: &TaskRepositoryEnvironment<Handle>) {
    let pagination_request = UNBOUNDED_PAGINATION_REQUEST;
    let pagination_response = env.task_repository.show_reverse_chronologically_ordered(pagination_request);

    assert!(pagination_response.items.is_empty());
    assert!(pagination_response.page_size == 0);
}

fn then_contains_only_task<Handle: TaskRepositoryEnvironmentHandle>(env: &TaskRepositoryEnvironment<Handle>, idx: usize) {
    let task = new_task_from_idx(idx);

    assert!(env.task_repository.contains(task.id));
    assert!(env.task_repository.get_by_id(task.id) == Some(task.clone()));

    let pagination_request = UNBOUNDED_PAGINATION_REQUEST;
    let pagination_response = env.task_repository.show_reverse_chronologically_ordered(pagination_request);

    assert!(pagination_response.items == vec![task]);
    assert!(pagination_response.page_size == 1);
}

fn then_contains_tasks_except<Handle: TaskRepositoryEnvironmentHandle>(env: &TaskRepositoryEnvironment<Handle>, range: std::ops::RangeInclusive<usize>, idx_except: usize) {
    let task_except = new_task_from_idx(idx_except);

    let tasks = range
        .map(|idx| new_task_from_idx(idx))
        .filter(|task| *task != task_except)
        .collect::<Vec<Task>>();

    assert!(tasks
        .iter()
        .map(|task| task.id)
        .all(|task_id| env.task_repository.contains(task_id)));
    assert!(tasks
        .iter()
        .cloned()
        .all(|task| env.task_repository.get_by_id(task.id) == Some(task)));

    let pagination_request = UNBOUNDED_PAGINATION_REQUEST;
    let pagination_response = env.task_repository.show_reverse_chronologically_ordered(pagination_request);

    assert!(pagination_response.items == tasks);
    assert!(pagination_response.page_size == tasks.len());
}

fn new_task_from_idx(idx: usize) -> Task {
    Task {
        id: Snowflake::new(
            Timestamp::from_millis_since_epoch(idx as i64),
            0, 0,
        ),
        description: "description".try_into().unwrap(),
        status: TaskStatus::Pending,
    }
}
