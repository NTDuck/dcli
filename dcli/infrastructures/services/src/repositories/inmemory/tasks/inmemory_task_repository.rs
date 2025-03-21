use std::cmp::Reverse;
use std::collections::BTreeMap;

use domain::tasks::Task;
use domain::tasks::TaskId;
use domain::tasks::TaskStatus;
use gateways::repositories::tasks::TaskRepository;
use models::pagination::PaginationRequest;
use models::pagination::PaginationResponse;

use crate::utils::configurations::pagination::MIN_PAGE_SIZE;
use crate::utils::dataclasses::pagination::PaginationRange;

pub struct InMemoryTaskRepository {
    tasks_by_ids: BTreeMap<Reverse<TaskId>, Task>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        return Self {
            tasks_by_ids: BTreeMap::new(),
        };
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn save(&mut self, task: Task) {
        self.tasks_by_ids.insert(Reverse(task.id), task);
    }

    fn remove(&mut self, task_id: TaskId) {
        self.tasks_by_ids.remove(&Reverse(task_id));
    }

    fn get_by_id(&self, task_id: TaskId) -> Option<Task> {
        return self.tasks_by_ids.get(&Reverse(task_id)).cloned();
    }

    fn show_reverse_chronologically_ordered(&self, pagination_request: PaginationRequest) -> PaginationResponse<Task> {
        let reverse_chronologically_ordered_tasks = self.tasks_by_ids.values();
        return Self::compute_pagination_response(reverse_chronologically_ordered_tasks, pagination_request);
    }

    fn show_reverse_chronologically_ordered_by_status(&self, status: TaskStatus, pagination_request: PaginationRequest) -> PaginationResponse<Task> {
        let reverse_chronologically_ordered_tasks_by_status = self.tasks_by_ids.values()
            .filter(|task| task.status == status);
        return Self::compute_pagination_response(reverse_chronologically_ordered_tasks_by_status, pagination_request);
    }

    fn contains(&self, task_id: TaskId) -> bool {
        return self.tasks_by_ids.contains_key(&Reverse(task_id));
    }

    fn clear(&mut self) {
        self.tasks_by_ids.clear();
    }

    fn clear_by_status(&mut self, status: TaskStatus) {
        self.tasks_by_ids
            .retain(|_, task| task.status != status);
    }
}

impl InMemoryTaskRepository {
    fn compute_pagination_response<'repo>(unpaginated_tasks: impl Iterator<Item = &'repo Task>, pagination_request: PaginationRequest) -> PaginationResponse<Task> {
        let pagination_range = PaginationRange::from(&pagination_request);

        let unpaginated_tasks_count = Self::compute_iterator_size(&unpaginated_tasks);
        let max_page_number = Self::compute_max_page_number(unpaginated_tasks_count, pagination_request.max_page_size);

        let paginated_tasks: Vec<_> = unpaginated_tasks
            .into_iter()
            .skip(pagination_range.offset)
            .take(pagination_range.limit)
            .cloned()
            .collect();
        let paginated_tasks_count = paginated_tasks.len();

        return PaginationResponse {
            items: paginated_tasks,
            page_size: paginated_tasks_count,
            max_page_size: pagination_request.max_page_size,
            page_number: pagination_request.page_number,
            max_page_number,
        };
    }

    fn compute_iterator_size<T>(iterator: &impl Iterator<Item = T>) -> usize {
        let (_, upper_bound) = iterator.size_hint();
        return upper_bound
            .expect("Iterator has no known upper bound");
    }

    fn compute_max_page_number(number_of_tasks: usize, max_page_size: usize) -> usize {
        return match number_of_tasks {
            0 => MIN_PAGE_SIZE,
            _ => number_of_tasks.div_ceil(max_page_size),
        };
    }
}
