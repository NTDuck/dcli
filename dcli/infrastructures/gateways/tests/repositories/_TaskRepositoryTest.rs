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
fn GivenEmtpyRepository_WhenSavingOneTask_ExpectPresentTask() {
    let (mut task_repository, task) = GivenEmptyRepositoryAndOneTask();
    WhenSavingOneTask(&mut task_repository, &task);
    ExpectPresentTask(&task_repository, &task);
}

#[test]
fn GivenEmptyRepository_WhenSavingOneTask_ExpectCorrectPaginationResponse() {
    let (mut task_repository, task) = GivenEmptyRepositoryAndOneTask();
    WhenSavingOneTask(&mut task_repository, &task);
    ExpectCorrectPaginationResponse(&task_repository, &task);
}

fn GivenEmptyRepositoryAndOneTask() -> (impl TaskRepository, Task) {
    return (
        GivenEmptyRepository(),
        GivenOneTask(),
    );
}

fn GivenEmptyRepository() -> impl TaskRepository {
    return OrderedInMemoryTaskRepository::new();
}

fn GivenOneTask() -> Task {
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

fn WhenSavingOneTask(task_repository: &mut impl TaskRepository, task: &Task) {
    task_repository.save(task.clone());
}

fn ExpectPresentTask(task_repository: &impl TaskRepository, task: &Task) {
    assert!(task_repository.get(task.id).is_some());
    assert!(task_repository.contains(task.id));
}

fn ExpectCorrectPaginationResponse(task_repository: &impl TaskRepository, task: &Task) {
    let pagination_request = PaginationRequest {
        page_number: 1,
        max_page_size: 5,
    };
    let pagination_response = task_repository.show(pagination_request);

    assert!(pagination_response.items.contains(&task));
    assert_eq!(pagination_response.page_size, 1);
    assert_eq!(pagination_response.max_page_size, 5);
    assert_eq!(pagination_response.page_number, 1);
    assert_eq!(pagination_response.max_page_number, 1);
}
