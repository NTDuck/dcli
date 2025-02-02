use domain::Task;
use domain::TaskId;
use domain::TaskStatus;
use indexmap::map::MutableKeys;
use indexmap::IndexMap;
use use_cases::gateways::gateways::common::PaginationParams;
use use_cases::gateways::gateways::tasks::TaskGateway;

pub struct OrderedInMemoryTaskGateway {
    tasks_by_ids: IndexMap<TaskId, Task>,
}

impl OrderedInMemoryTaskGateway {
    pub fn new() -> Self {
        return Self {
            tasks_by_ids: IndexMap::new(),
        };
    }
}

impl TaskGateway for OrderedInMemoryTaskGateway {
    fn save(&mut self, task: &Task) {
        self.tasks_by_ids.insert(task.id, task.clone());
    }

    fn remove(&mut self, task_id: TaskId) {
        self.tasks_by_ids.shift_remove(&task_id);
    }

    fn get(&self, task_id: TaskId) -> Option<Task> {
        return self.tasks_by_ids
            .get(&task_id)
            .cloned();
    }

    fn show(&self, pagination_params: PaginationParams) -> Vec<Task> {
        return self.tasks_by_ids
            .values()
            .rev()   // Sort by insertion order
            .skip(pagination_params.offset)
            .take(pagination_params.limit)
            .cloned()
            .collect();
    }

    fn show_by_status(&self, status: TaskStatus, pagination_params: PaginationParams) -> Vec<Task> {
        return self.tasks_by_ids
            .values()
            .filter(|task| task.status == status)
            .rev()   // Sort by insertion order
            .skip(pagination_params.offset)
            .take(pagination_params.limit)
            .cloned()
            .collect();
    }

    fn contains(&self, task_id: TaskId) -> bool {
        return self.tasks_by_ids
            .contains_key(&task_id);
    }

    fn clear(&mut self) {
        self.tasks_by_ids.clear();
    }

    fn clear_by_status(&mut self, status: TaskStatus) {
        self.tasks_by_ids
            .retain2(|_, task| task.status != status);
    }
}