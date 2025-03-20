use std::cmp::Reverse;
use std::collections::BTreeMap;

use domain::tasks::Task;
use domain::tasks::TaskId;
use domain::tasks::TaskStatus;
use use_cases::gateways::repositories::tasks::TaskRepository;
use use_cases::utils::dataclasses::pagination::MIN_PAGE_SIZE;
use use_cases::utils::dataclasses::pagination::PaginationRange;
use use_cases::utils::dataclasses::pagination::PaginationRequest;
use use_cases::utils::dataclasses::pagination::PaginationResponse;

pub struct InMemoryTaskRepository {
    tasksByIds: BTreeMap<Reverse<TaskId>, Task>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        return Self {
            tasksByIds: BTreeMap::new(),
        };
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn save(&mut self, task: Task) {
        self.tasksByIds.insert(Reverse(task.id), task);
    }

    fn remove(&mut self, taskId: TaskId) {
        self.tasksByIds.remove(&Reverse(taskId));
    }

    fn get_by_id(&self, taskId: TaskId) -> Option<Task> {
        return self.tasksByIds.get(&Reverse(taskId)).cloned();
    }

    fn show_reverse_chronologically_ordered(&self, paginationRequest: PaginationRequest) -> PaginationResponse<Task> {
        let reverseChronologicallyOrderedTasks = self.tasksByIds.values();
        return Self::computePaginationResponse(reverseChronologicallyOrderedTasks, paginationRequest);
    }

    fn show_reverse_chronologically_ordered_by_status(&self, status: TaskStatus, paginationRequest: PaginationRequest) -> PaginationResponse<Task> {
        let reverseChronologicallyOrderedTasksByStatus = self.tasksByIds.values()
            .filter(|task| task.status == status);
        return Self::computePaginationResponse(reverseChronologicallyOrderedTasksByStatus, paginationRequest);
    }

    fn contains(&self, taskId: TaskId) -> bool {
        return self.tasksByIds.contains_key(&Reverse(taskId));
    }

    fn clear(&mut self) {
        self.tasksByIds.clear();
    }

    fn clear_by_status(&mut self, status: TaskStatus) {
        self.tasksByIds
            .retain(|_, task| task.status != status);
    }
}

impl InMemoryTaskRepository {
    fn computePaginationResponse<'repo>(unpaginatedTasks: impl Iterator<Item = &'repo Task>, paginationRequest: PaginationRequest) -> PaginationResponse<Task> {
        let paginationRange = PaginationRange::from(&paginationRequest);

        let unpaginatedTasksCount = Self::computeIteratorSize(&unpaginatedTasks);
        let maxPageNumber = Self::computeMaxPageNumber(unpaginatedTasksCount, paginationRequest.max_page_size);

        let paginatedTasks: Vec<_> = unpaginatedTasks
            .into_iter()
            .skip(paginationRange.offset)
            .take(paginationRange.limit)
            .cloned()
            .collect();
        let paginatedTasksCount = paginatedTasks.len();

        return PaginationResponse {
            items: paginatedTasks,
            page_size: paginatedTasksCount,
            max_page_size: paginationRequest.max_page_size,
            page_number: paginationRequest.page_number,
            max_page_number: maxPageNumber,
        };
    }

    fn computeIteratorSize<T>(iterator: &impl Iterator<Item = T>) -> usize {
        let (_, upperBound) = iterator.size_hint();
        return upperBound
            .expect("Iterator has no known upper bound");
    }

    fn computeMaxPageNumber(numberOfTasks: usize, maxPageSize: usize) -> usize {
        return match numberOfTasks {
            0 => MIN_PAGE_SIZE,
            _ => numberOfTasks.div_ceil(maxPageSize),
        };
    }
}
