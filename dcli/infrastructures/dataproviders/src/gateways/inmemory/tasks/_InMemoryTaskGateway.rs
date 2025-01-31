use std::collections::HashMap;

use domain::Task;
use domain::TaskId;
use domain::TaskStatus;
use use_cases::dataproviders::gateways::common::PaginationParams;
use use_cases::dataproviders::gateways::tasks::TaskGateway;

// use rayon::prelude::*;

pub struct UnorderedInMemoryTaskGateway {
    tasks_by_ids: HashMap<TaskId, Task>,
}

impl UnorderedInMemoryTaskGateway {
    pub fn new() -> Self {
        return Self {
            tasks_by_ids: HashMap::new(),
        };
    }
}

impl TaskGateway for UnorderedInMemoryTaskGateway {
    fn save(&mut self, task: &Task) {
        self.tasks_by_ids
            .insert(task.id, task.clone());
    }

    fn remove(&mut self, task_id: TaskId) {
        self.tasks_by_ids
            .remove(&task_id);
    }

    fn get(&self, task_id: TaskId) -> Option<Task> {
        return self.tasks_by_ids
            .get(&task_id)
            .cloned();
    }

    fn show(&self, pagination_params: PaginationParams) -> Vec<Task> {
        return self.tasks_by_ids
            .values()
            .skip(pagination_params.offset)
            .take(pagination_params.limit)
            .cloned()
            .collect();
    }

    fn show_by_status(&self, status: TaskStatus, pagination_params: PaginationParams) -> Vec<Task> {
        return self.tasks_by_ids
            .values()
            .filter(|task| task.status == status)
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
            .retain(|_, task| task.status != status);
    }
}
