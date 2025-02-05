// use std::time::Instant;

// use domain::Task;
// use domain::TaskDescription;
// use domain::TaskId;
// use domain::Uuid;
// use fake::Fake;
// use fake::Faker;
// use gateways::repositories::inmemory::tasks::OrderedInMemoryTaskRepository;
// use use_cases::gateways::repositories::tasks::TaskRepository;
// use use_cases::utils::dataclasses::pagination::PaginationRequest;
// use use_cases::utils::dataclasses::pagination::PaginationResponse;

// #[test]
// fn GivenEmptyRepository_WhenSavingOneTask_ExpectCorrectPaginationResponse() {
//     let task: Task = Faker.fake();
// }

// fn GivenEmptyRepository() -> impl TaskRepository {
//     return OrderedInMemoryTaskRepository::new();
// }

// fn WhenSavingOneTask(task_repository: impl TaskRepository) -> PaginationResponse<Task> {
    
// }

// struct TaskMocker;

// impl TaskMocker {
//     fn mock(&self) -> Task {
//         return Task {
//             id: TaskId::from(Faker.fake::<u128>()),
//             description: unsafe { TaskDescription::try_from((TaskDescription::MIN_LENGTH..TaskDescription::MAX_LENGTH)
//                 .fake::<String>())
//                 .unwrap_unchecked()
//             },
//             status: todo!(),
//             created_at: Faker.fake::<Instant>(),
//         }
//     }
// }