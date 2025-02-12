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

pub struct OrderedInMemoryTaskRepository {
    tasks_by_ids: IndexMap<TaskId, Task>,
}

impl OrderedInMemoryTaskRepository {
    pub fn new() -> Self {
        return Self {
            tasks_by_ids: IndexMap::new(),
        };
    }
}

impl TaskRepository for OrderedInMemoryTaskRepository {
    fn save(&mut self, task: Task) {
        self.tasks_by_ids.insert(task.id, task);
    }

    fn remove(&mut self, task_id: TaskId) {
        self.tasks_by_ids.shift_remove(&task_id);
    }

    fn getById(&self, task_id: TaskId) -> Option<Task> {
        return self.tasks_by_ids.get(&task_id).cloned();
    }

    fn showOrderedByCreatedAtDesc(&self, pagination_request: PaginationRequest) -> PaginationResponse<Task> {
        let pagination_range = PaginationRange::from(&pagination_request);

        let tasks: Vec<_> = self.tasks_by_ids
            .values()
            .rev()   // Sort by insertion order
            .skip(pagination_range.offset)
            .take(pagination_range.limit)
            .cloned()
            .collect();

        let page_size = tasks.len();

        return PaginationResponse {
            items: tasks,
            pageSize: page_size,
            maxPageSize: pagination_request.maxPageSize,
            pageNumber: pagination_request.pageNumber,
            maxPageNumber: match self.tasks_by_ids.len() {
                0 => PaginationProperties::MinPageSize,
                _ => self.tasks_by_ids.len()
                    .div_ceil(pagination_request.maxPageSize),
            },
        };
    }

    fn showByStatusOrderedByCreatedAtDesc(&self, status: TaskStatus, pagination_request: PaginationRequest) -> PaginationResponse<Task> {
        let pagination_range = PaginationRange::from(&pagination_request);

        let tasks: Vec<_> = self.tasks_by_ids
            .values()
            .filter(|task| task.status == status)
            .rev()   // Sort by insertion order
            .skip(pagination_range.offset)
            .take(pagination_range.limit)
            .cloned()
            .collect();

        let page_size = tasks.len();

        return PaginationResponse {
            items: tasks,
            pageSize: page_size,
            maxPageSize: pagination_request.maxPageSize,
            pageNumber: pagination_request.pageNumber,
            maxPageNumber: {
                let total_items_count = self.tasks_by_ids
                    .values()
                    .filter(|task| task.status == status)
                    .count();
                match total_items_count {
                    0 => PaginationProperties::MinPageSize,
                    _ => total_items_count.div_ceil(pagination_request.maxPageSize),
                }
            },
        };
    }

    fn contains(&self, task_id: TaskId) -> bool {
        return self.tasks_by_ids
            .contains_key(&task_id);
    }

    fn clear(&mut self) {
        self.tasks_by_ids.clear();
    }

    fn clearByStatus(&mut self, status: TaskStatus) {
        self.tasks_by_ids
            .retain2(|_, task| task.status != status);
    }
}
