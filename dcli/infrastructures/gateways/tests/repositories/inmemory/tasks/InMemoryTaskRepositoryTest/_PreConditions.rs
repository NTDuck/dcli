use domain::utils::dataclasses::time::Timestamp;
use domain::Task;
use domain::TaskDescription;
use domain::TaskId;
use domain::TaskStatus;
use gateways::repositories::inmemory::tasks::InMemoryTaskRepository;
use use_cases::gateways::repositories::tasks::TaskRepository;

pub fn GivenRepositoryContainingZeroTasks() -> impl TaskRepository {
    return GivenRepositoryContainingManyTasksWithIds([]);
}

pub fn GivenRepositoryContainingOneTaskWithId(taskId: u128) -> impl TaskRepository {
    return GivenRepositoryContainingManyTasksWithIds([taskId]);
}

pub fn GivenRepositoryContainingManyTasksWithIds<const N: usize>(taskIds: [u128; N]) -> impl TaskRepository {
    let mut taskRepository = InMemoryTaskRepository::new();

    let tasks = mockTasksWithIds(taskIds);
    tasks
        .into_iter()
        .for_each(|task| taskRepository.save(task));

    return taskRepository;
}

fn mockTasksWithIds<const N: usize>(taskIds: [TaskId; N]) -> [Task; N] {
    return std::array::from_fn(|i| mockTaskWithId(TaskId::from(taskIds[i])));
}

fn mockTaskWithId(taskId: TaskId) -> Task {
    return Task {
        id: taskId,
        description: TaskDescription::try_from("description".to_string()).unwrap(),
        status: TaskStatus::Pending,
        createdAt: Timestamp::now(),
    };
}

pub fn mockTaskId() -> TaskId {
    return TaskId::from(rand::random::<u128>());
}
