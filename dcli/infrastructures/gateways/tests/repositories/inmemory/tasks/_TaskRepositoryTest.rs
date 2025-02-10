mod save {

}

mod remove {

}

mod get {
    use domain::Task;
    use domain::TaskId;
    use gateways::repositories::inmemory::tasks::OrderedInMemoryTaskRepository;
    use use_cases::gateways::repositories::tasks::TaskRepository;

    use super::utils::generate_random_task_id;
    use super::utils::GivenRepositoryWithZeroTasks;

    #[test]
    fn GivenRepositoryWithZeroTasks_WhenGettingAny_ExpectNone() {
        let task_repository = GivenRepositoryWithZeroTasks();
        let task_id = generate_random_task_id();
        let task = WhenGettingTaskWithId(&task_repository, task_id);
        
        assert!(task.is_none());
    }

    #[test]
    fn GivenRepositoryWithOneTask_WhenGettingIt_ExpectMatchingSome() {
        
    }

    #[test]
    fn GivenRepositoryWithOneTask_WhenGettingAnyOther_ExpectNone() {

    }

    #[test]
    fn GivenRepositoryWithManyTasks_WhenGettingOneAmongThem_ExpectMatchingSome() {

    }

    #[test]
    fn GivenRepositoryWithManyTasks_WhenGettingOneNotAmongThem_ExpectNone() {

    }

    fn WhenGettingTaskWithId(task_repository: &OrderedInMemoryTaskRepository, task_id: TaskId) -> Option<Task> {
        return task_repository.get(task_id);
    }
}

mod show {

}

mod show_by_status {

}

mod contains {
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

mod clear {

}

mod clear_by_status {

}

mod utils {
    use std::time::Instant;

    use domain::Task;
    use domain::TaskDescription;
    use domain::TaskId;
    use domain::TaskStatus;
    use fake::Fake;
    use fake::Faker;
    use gateways::repositories::inmemory::tasks::OrderedInMemoryTaskRepository;
    use use_cases::gateways::repositories::tasks::TaskRepository;

    const NUMBER_OF_TASKS: usize = 42;

    pub fn GivenRepositoryWithZeroTasks() -> OrderedInMemoryTaskRepository {
        return GivenRepositoryWithManyTasks::<0>();
    }

    pub fn GivenRepositoryWithOneTask() -> OrderedInMemoryTaskRepository {
        return GivenRepositoryWithManyTasks::<1>();
    }

    pub fn GivenRepositoryWithManyTasks<const N: usize>() -> OrderedInMemoryTaskRepository {
        let mut task_repository = OrderedInMemoryTaskRepository::new();

        (0..N)
            .into_iter()
            .for_each(|_| {
                let task = generate_random_task();
                task_repository.save(task);
            });

        return task_repository;
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
