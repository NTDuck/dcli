use domain::Task;
use domain::TaskId;
use domain::TaskStatus;

use crate::dataproviders::gateways::common::PaginationParams;

pub trait TaskGateway {
    fn save(&mut self, task: &Task);
    fn remove(&mut self, task_id: TaskId);

    fn get(&self, task_id: TaskId) -> Option<Task>;

    fn show(&self, pagination_params: PaginationParams) -> Option<Vec<Task>>;
    fn show_by_status(&self, status: TaskStatus, pagination_params: PaginationParams) -> Option<Vec<Task>>;

    fn contains(&self, task_id: TaskId) -> bool;

    fn clear(&mut self);
    fn clear_by_status(&mut self, status: TaskStatus);
}
