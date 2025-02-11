use std::time::Instant;

use domain::Task;
use domain::TaskDescription;
use domain::TaskId;
use domain::TaskStatus;
use fake::Fake;
use fake::Faker;
use gateways::repositories::inmemory::tasks::OrderedInMemoryTaskRepository;
use use_cases::gateways::repositories::tasks::TaskRepository;

mod test_get {
    use super::*;
    use super::utils::*;
    use super::config::*;

    #[test]
    fn GivenRepositoryWithZeroTasks_WhenGettingAny_ExpectNone() {
        let task_repository = GivenRepositoryWithZeroTasks();
        let task = WhenGettingAny(&task_repository);        
        ExpectNone(task);
    }

    fn WhenGettingAny(task_repository: &impl TaskRepository) -> Option<Task> {
        let task_id = generate_random_task_id();
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
            let task_id = generate_random_task_id();
            
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
            let task_id = generate_random_task_id();
            
            if !given_task_ids.contains(&task_id) {
                break task_id;
            }
        };

        let retrieved_task = WhenGettingTaskWithId(task_repository, any_other_task_id);
        return (retrieved_task, any_other_task_id);
    }
}

mod test_contains {
    #[test]
    fn GivenRepositoryWithZeroTasks_WhenCheckingIfAnyExists_ExpectFalse() {
        
    }

    #[test]
    fn GivenRepositoryWithOneTask_WhenCheckingIfItExists_ExpectTrue() {

    }

    #[test]
    fn GivenRepositoryWithOneTask_WhenCheckingIfAnyOtherExists_ExpectFalse() {

    }

    #[test]
    fn GivenRepositoryWithManyTasks_WhenCheckingIfOneAmongThemExists_ExpectTrue() {

    }

    #[test]
    fn GivenRepositoryWithManyTasks_WhenCheckingIfOneNotAmongThemExists_ExpectFalse() {

    }
}

mod utils {
    use super::*;

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
        let tasks = std::array::from_fn(|_| generate_random_task());

        tasks
            .iter()
            .map(|task| task.clone())
            .for_each(|task| task_repository.save(task));

        return (task_repository, tasks);
    }

    pub fn generate_random_task() -> Task {
        return Task {
            id: generate_random_task_id(),
            description: unsafe { TaskDescription::try_from((TaskDescription::MIN_LENGTH..TaskDescription::MAX_LENGTH)
                .fake::<String>())
                .unwrap_unchecked()
            },
            status: *[TaskStatus::Pending, TaskStatus::InProgress, TaskStatus::Completed]
                .get((0..2).fake::<usize>())
                .unwrap(),
            created_at: Instant::now(),
        };
    }

    pub fn generate_random_task_id() -> TaskId {
        return TaskId::from(Faker.fake::<u128>());
    }
}

mod config {
    pub const NUMBER_OF_TASKS: usize = 42;

    const _: () = assert!(NUMBER_OF_TASKS > 0);
}
