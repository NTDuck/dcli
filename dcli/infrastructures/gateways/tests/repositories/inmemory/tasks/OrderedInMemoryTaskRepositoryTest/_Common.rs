use domain::Task;
use fake::Fake;
use fake::Faker;
use gateways::repositories::inmemory::tasks::OrderedInMemoryTaskRepository;
use use_cases::gateways::repositories::tasks::TaskRepository;

use crate::utils::tasks::MockTask;

pub fn GivenRepositoryWithZeroTasks() -> impl TaskRepository {
    let (task_repository, _) = GivenRepositoryWithManyTasks::<0>();
    return task_repository;
}

pub fn GivenRepositoryWithOneTask() -> (impl TaskRepository, Task) {
    let (task_repository, tasks) = GivenRepositoryWithManyTasks::<1>();
    return (task_repository, tasks[0].clone());
}

pub fn GivenRepositoryWithManyTasks<const N: usize>() -> (impl TaskRepository, [Task; N]) {
    let mut task_repository = OrderedInMemoryTaskRepository::new();

    let tasks: [Task; N] = std::array::from_fn(|_|
        Faker.fake::<MockTask>().into());

    tasks
        .iter()
        .map(|task| task.clone())
        .for_each(|task| task_repository.save(task));

    return (task_repository, tasks);
}
