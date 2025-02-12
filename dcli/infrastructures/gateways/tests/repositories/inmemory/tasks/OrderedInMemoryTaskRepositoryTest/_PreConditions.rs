use domain::Task;
use gateways::repositories::inmemory::tasks::InMemoryTaskRepository;
use use_cases::gateways::repositories::tasks::TaskRepository;

pub fn GivenRepositoryWithZeroTasks() -> impl TaskRepository {
    let (taskRepository, _) = GivenRepositoryWithManyTasks::<0>();
    return taskRepository;
}

pub fn GivenRepositoryWithOneTask() -> (impl TaskRepository, Task) {
    let (taskRepository, tasks) = GivenRepositoryWithManyTasks::<1>();
    let task = tasks[0].clone();
    return (taskRepository, task);
}

pub fn GivenRepositoryWithManyTasks() -> (impl TaskRepository, [Task; N]) {
    let mut taskRepository = InMemoryTaskRepository::new();

    let tasks: [Task; N] = std::array::from_fn(|_| GivenTask());

    tasks
        .iter()
        .map(|task| task.clone())
        .for_each(|task| taskRepository.save(task));

    return (taskRepository, tasks);
}
