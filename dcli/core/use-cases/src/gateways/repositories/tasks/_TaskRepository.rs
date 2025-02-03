use domain::Task;
use domain::TaskId;
use domain::TaskStatus;

use crate::utils::dataclasses::pagination::PaginationRequest;
use crate::utils::dataclasses::pagination::PaginationResult;

pub trait TaskRepository {
    fn save(&mut self, task: &Task);
    fn remove(&mut self, task_id: TaskId);

    fn get(&self, task_id: TaskId) -> Option<Task>;

    fn show(&self, pagination_request: PaginationRequest) -> PaginationResult<Task>;
    fn show_by_status(&self, status: TaskStatus, pagination_request: PaginationRequest) -> PaginationResult<Task>;

    fn contains(&self, task_id: TaskId) -> bool;

    fn clear(&mut self);
    fn clear_by_status(&mut self, status: TaskStatus);
}
