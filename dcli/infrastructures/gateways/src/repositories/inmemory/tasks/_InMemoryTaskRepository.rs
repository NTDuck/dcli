use domain::Task;
use domain::TaskId;
use domain::TaskStatus;
use indexmap::map::MutableKeys;
use indexmap::IndexMap;
use use_cases::gateways::repositories::tasks::TaskRepository;
use use_cases::utils::dataclasses::pagination::PaginationProperties;
use use_cases::utils::dataclasses::pagination::PaginationRange;
use use_cases::utils::dataclasses::pagination::PaginationRequest;
use use_cases::utils::dataclasses::pagination::PaginationResponse;

pub struct InMemoryTaskRepository {
    tasksByIds: IndexMap<TaskId, Task>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        return Self {
            tasksByIds: IndexMap::new(),
        };
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn save(&mut self, task: Task) {
        self.tasksByIds.insert(task.id, task);
    }

    fn remove(&mut self, taskId: TaskId) {
        self.tasksByIds.shift_remove(&taskId);
    }

    fn getById(&self, taskId: TaskId) -> Option<Task> {
        return self.tasksByIds.get(&taskId).cloned();
    }

    fn showOrderedByCreatedAtDesc(&self, paginationRequest: PaginationRequest) -> PaginationResponse<Task> {
        let paginationRange = PaginationRange::from(&paginationRequest);

        let tasks: Vec<_> = self.tasksByIds
            .values()
            .rev()   // Sort by insertion order
            .skip(paginationRange.offset)
            .take(paginationRange.limit)
            .cloned()
            .collect();

        let pageSize = tasks.len();

        return PaginationResponse {
            items: tasks,
            pageSize,
            maxPageSize: paginationRequest.maxPageSize,
            pageNumber: paginationRequest.pageNumber,
            maxPageNumber: match self.tasksByIds.len() {
                0 => PaginationProperties::MinPageSize,
                _ => self.tasksByIds.len()
                    .div_ceil(paginationRequest.maxPageSize),
            },
        };
    }

    fn showByStatusOrderedByCreatedAtDesc(&self, status: TaskStatus, paginationRequest: PaginationRequest) -> PaginationResponse<Task> {
        let paginationRange = PaginationRange::from(&paginationRequest);

        let tasks: Vec<_> = self.tasksByIds
            .values()
            .filter(|task| task.status == status)
            .rev()   // Sort by insertion order
            .skip(paginationRange.offset)
            .take(paginationRange.limit)
            .cloned()
            .collect();

        let pageSize = tasks.len();

        return PaginationResponse {
            items: tasks,
            pageSize,
            maxPageSize: paginationRequest.maxPageSize,
            pageNumber: paginationRequest.pageNumber,
            maxPageNumber: {
                let totalItemsCount = self.tasksByIds
                    .values()
                    .filter(|task| task.status == status)
                    .count();
                match totalItemsCount {
                    0 => PaginationProperties::MinPageSize,
                    _ => totalItemsCount.div_ceil(paginationRequest.maxPageSize),
                }
            },
        };
    }

    fn contains(&self, taskId: TaskId) -> bool {
        return self.tasksByIds
            .contains_key(&taskId);
    }

    fn clear(&mut self) {
        self.tasksByIds.clear();
    }

    fn clearByStatus(&mut self, status: TaskStatus) {
        self.tasksByIds
            .retain2(|_, task| task.status != status);
    }
}
