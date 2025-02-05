use std::time::Instant;

use domain::Task;
use domain::TaskDescription;
use domain::TaskId;
use domain::TaskStatus;
use fake::Fake;
use fake::Faker;
use gateways::repositories::inmemory::tasks::OrderedInMemoryTaskRepository;
use use_cases::gateways::repositories::tasks::TaskRepository;
use use_cases::utils::dataclasses::pagination::PaginationRequest;

#[test]
fn GivenEmptyRepository_WhenSavingOneTask_ExpectCorrectPaginationResponse() {
    // GivenEmptyRepository
    let mut task_repository = OrderedInMemoryTaskRepository::new();

    // WhenSavingOneTask
    let task = TaskMocker::mock();
    task_repository.save(task.clone());

    // Uhh what
    assert!(task_repository.get(task.id).is_some());
    assert!(task_repository.contains(task.id));

    // ExpectCorrectPaginationResponse
    let pagination_request = PaginationRequest {
        page_number: 1,
        max_page_size: 5,
    };
    let pagination_response = task_repository.show(pagination_request);

    dbg!(&pagination_response);

    assert!(pagination_response.items.contains(&task));
    debug_assert_eq!(pagination_response.page_size, 1);
    debug_assert_eq!(pagination_response.max_page_size, 5);
    debug_assert_eq!(pagination_response.page_number, 1);
    debug_assert_eq!(pagination_response.max_page_number, 1);
}

struct TaskMocker;

impl TaskMocker {
    fn mock() -> Task {
        return Task {
            id: TaskId::from(Faker.fake::<u128>()),
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
}