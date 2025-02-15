use domain::utils::dataclasses::time::Timestamp;
use domain::Task;
use domain::TaskDescription;
use domain::TaskId;
use domain::TaskStatus;
use gateways::repositories::inmemory::tasks::InMemoryTaskRepository;
use use_cases::gateways::repositories::tasks::TaskRepository;

pub(crate) fn GivenRepositoryContainingZeroTasks() -> impl TaskRepository {
    return GivenRepositoryContainingManyTasksWithIds([]);
}

pub(crate) fn GivenRepositoryContainingOneTaskWithId(taskId: u128) -> impl TaskRepository {
    return GivenRepositoryContainingManyTasksWithIds([taskId]);
}

pub(crate) fn GivenRepositoryContainingManyTasksWithIds<const N: usize>(taskIds: [u128; N]) -> impl TaskRepository {
    let mut taskRepository = InMemoryTaskRepository::new();

    let taskIds: [_; N] = std::array::from_fn(|i| TaskId::from(taskIds[i]));
    let tasks = mockTasksWithIds(taskIds);
    tasks
        .into_iter()
        .for_each(|task| taskRepository.save(task));

    return taskRepository;
}

fn mockTasksWithIds<const N: usize>(taskIds: [TaskId; N]) -> [Task; N] {
    return std::array::from_fn(|i| mockTaskWithId(taskIds[i]));
}

fn mockTaskWithId(taskId: TaskId) -> Task {
    return Task {
        id: taskId,
        description: TaskDescription::try_from("description".to_string()).unwrap(),
        status: TaskStatus::Pending,
        createdAt: Timestamp::now(),
    };
}

