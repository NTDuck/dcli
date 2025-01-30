use std::collections::HashMap;
use std::collections::HashSet;

use domain::Task;
use domain::TaskId;
use domain::TaskStatus;
use use_cases::dataproviders::gateways::common::PaginationParams;
use use_cases::dataproviders::gateways::tasks::TaskGateway;

// use rayon::prelude::*;

pub struct InMemoryTaskGateway {
    tasks_by_ids: HashMap<TaskId, Task>,
    task_id_sets_by_statuses: HashMap<TaskStatus, HashSet<TaskId>>,
}

impl InMemoryTaskGateway {
    pub fn new() -> Self {
        return Self {
            tasks_by_ids: HashMap::new(),
            task_id_sets_by_statuses: Self::create_task_id_sets_by_statuses(),
        };
    }

    fn create_task_id_sets_by_statuses() -> HashMap<TaskStatus, HashSet<TaskId>> {
        return [TaskStatus::Pending, TaskStatus::InProgress, TaskStatus::Completed]
            .into_iter()
            .map(|status| (status, HashSet::new()))
            .collect();
    }
}

impl TaskGateway for InMemoryTaskGateway {
    fn save(&mut self, new_task: &Task) {
        if let Some(existing_task) = self.tasks_by_ids.get(&new_task.id) {
            self.task_id_sets_by_statuses
                .get_mut(&existing_task.status)
                .unwrap()
                .remove(&new_task.id);
        }

        self.tasks_by_ids.insert(new_task.id, new_task.clone());
        self.task_id_sets_by_statuses
            .get_mut(&new_task.status)
            .unwrap()
            .insert(new_task.id);
    }

    fn remove(&mut self, task_id: TaskId) {
        let Some(existing_task) = self.tasks_by_ids.remove(&task_id) else { return; };

        self.task_id_sets_by_statuses
            .get_mut(&existing_task.status)
            .unwrap()
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
        return self.tasks_by_ids.contains_key(&task_id);
    }

    fn clear(&mut self) {
        self.tasks_by_ids.clear();
        self.task_id_sets_by_statuses.clear();
    }

    fn clear_by_status(&mut self, status: TaskStatus) {
        if let Some(task_id_set) = self.task_id_sets_by_statuses.remove(&status) {
            for task_id in task_id_set {
                self.tasks_by_ids.remove(&task_id);
            }
        }
    }
}
