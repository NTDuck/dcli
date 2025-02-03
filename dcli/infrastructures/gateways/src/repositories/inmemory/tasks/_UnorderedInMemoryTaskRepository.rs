use std::collections::HashMap;

use domain::Task;
use domain::TaskId;
use domain::TaskStatus;
use use_cases::gateways::repositories::tasks::TaskRepository;
use use_cases::utils::dataclasses::pagination::PaginationRange;
use use_cases::utils::dataclasses::pagination::PaginationRequest;
use use_cases::utils::dataclasses::pagination::PaginationResult;

use crate::utils::repositories::tasks::HashableTaskId;

pub struct UnorderedInMemoryTaskRepository {
    tasks_by_ids: HashMap<HashableTaskId, Task>,
}

impl UnorderedInMemoryTaskRepository {
    pub fn new() -> Self {
        return Self {
            tasks_by_ids: HashMap::new(),
        };
    }
}

impl TaskRepository for UnorderedInMemoryTaskRepository {
    fn save(&mut self, task: &Task) {
        let task_id = HashableTaskId::from(task.id);
        self.tasks_by_ids.insert(task_id, task.clone());
    }

    fn remove(&mut self, task_id: TaskId) {
        let task_id = HashableTaskId::from(task_id);
        self.tasks_by_ids
            .remove(&task_id);
    }

    fn get(&self, task_id: TaskId) -> Option<Task> {
        let task_id = HashableTaskId::from(task_id);
        return self.tasks_by_ids
            .get(&task_id)
            .cloned();
    }

    fn show(&self, pagination_request: PaginationRequest) -> PaginationResult<Task> {
        let pagination_range = PaginationRange::from(&pagination_request);

        let tasks = self.tasks_by_ids
            .values()
            .skip(pagination_range.offset)
            .take(pagination_range.limit)
            .cloned()
            .collect();

        return PaginationResult {
            items: tasks,
            page_size: pagination_range.limit,
            max_page_size: pagination_request.max_page_size,
            page_number: pagination_request.page_number,
            max_page_number: self.calc_max_page_number(pagination_request.max_page_size),
        };
    }

    fn show_by_status(&self, status: TaskStatus, pagination_request: PaginationRequest) -> PaginationResult<Task> {
        let pagination_range = PaginationRange::from(&pagination_request);

        let tasks = self.tasks_by_ids
            .values()
            .filter(|task| task.status == status)
            .skip(pagination_range.offset)
            .take(pagination_range.limit)
            .cloned()
            .collect();
    
        return PaginationResult {
            items: tasks,
            page_size: pagination_range.limit,
            max_page_size: pagination_request.max_page_size,
            page_number: pagination_request.page_number,
            max_page_number: self.calc_max_page_number(pagination_request.max_page_size),
        };
    }

    fn contains(&self, task_id: TaskId) -> bool {
        let task_id = HashableTaskId::from(task_id);
        return self.tasks_by_ids
            .contains_key(&task_id);
    }

    fn clear(&mut self) {
        self.tasks_by_ids.clear();
    }

    fn clear_by_status(&mut self, status: TaskStatus) {
        self.tasks_by_ids
            .retain(|_, task| task.status != status);
    }
}

impl UnorderedInMemoryTaskRepository {
    fn calc_max_page_number(&self, max_page_size: usize) -> usize {
        let total_items_count = self.tasks_by_ids.len();
        return total_items_count.div_ceil(max_page_size);
    }
}
