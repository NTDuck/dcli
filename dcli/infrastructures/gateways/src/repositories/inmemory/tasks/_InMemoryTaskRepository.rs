use std::cmp::Reverse;
use std::collections::BTreeMap;

use domain::tasks::Task;
use domain::tasks::TaskId;
use domain::tasks::TaskStatus;
use use_cases::gateways::repositories::tasks::TaskRepository;
use use_cases::utils::dataclasses::pagination::MinPageSize;
use use_cases::utils::dataclasses::pagination::PaginationRange;
use use_cases::utils::dataclasses::pagination::PaginationRequest;
use use_cases::utils::dataclasses::pagination::PaginationResponse;

pub struct InMemoryTaskRepository {
    tasksByIds: BTreeMap<TaskId, Task>,
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
        self.tasksByIds.remove(&taskId);
    }

    fn getById(&self, taskId: TaskId) -> Option<Task> {
        return self.tasksByIds.get(&taskId).cloned();
    }

    fn showReverseChronologicallyOrdered(&self, paginationRequest: PaginationRequest) -> PaginationResponse<Task> {
        let reverseChronologicallyOrderedTasks = self.tasksByIds
            .values();
        return Self::computePaginationResponse(reverseChronologicallyOrderedTasks, paginationRequest);
    }

    fn showReverseChronologicallyOrderedByStatus(&self, status: TaskStatus, paginationRequest: PaginationRequest) -> PaginationResponse<Task> {
        let reverseChronologicallyOrderedTasksByStatus = self.tasksByIds
            .values()
            .filter(|task| task.status == status);
        return Self::computePaginationResponse(reverseChronologicallyOrderedTasksByStatus, paginationRequest);
    }

    fn contains(&self, taskId: TaskId) -> bool {
        return self.tasksByIds.contains_key(&taskId);
    }

    fn clear(&mut self) {
        self.tasksByIds.clear();
    }

    fn clearByStatus(&mut self, status: TaskStatus) {
        self.tasksByIds
            .retain(|_, task| task.status != status);
    }
}

impl InMemoryTaskRepository {
    fn computePaginationResponse(unpaginatedTasks: impl Iterator<Item = Task>, paginationRequest: PaginationRequest) -> PaginationResponse<Task> {
        let paginationRange = PaginationRange::from(&paginationRequest);

        let unpaginatedTasksCount = unpaginatedTasks.len();

        let paginatedTasks: Vec<_> = unpaginatedTasks
            .into_iter()
            .skip(paginationRange.offset)
            .take(paginationRange.limit)
            .cloned()
            .collect();
        let paginatedTasksCount = paginatedTasks.len();

        let maxPageNumber = Self::computeMaxPageNumber(unpaginatedTasksCount, paginationRequest.maxPageSize);

        return PaginationResponse {
            items: paginatedTasks,
            pageSize: paginatedTasksCount,
            maxPageSize: paginationRequest.maxPageSize,
            pageNumber: paginationRequest.pageNumber,
            maxPageNumber,
        };
    }

    fn computeMaxPageNumber(numberOfTasks: usize, maxPageSize: usize) -> usize {
        return match numberOfTasks {
            0 => MinPageSize,
            _ => numberOfTasks.div_ceil(maxPageSize),
        };
    }
}
