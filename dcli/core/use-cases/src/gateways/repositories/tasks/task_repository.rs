use domain::tasks::Task;
use domain::tasks::TaskId;
use domain::tasks::TaskStatus;

use crate::utils::dataclasses::pagination::PaginationRequest;
use crate::utils::dataclasses::pagination::PaginationResponse;
use crate::utils::interfaces::Gateway;

pub trait TaskRepository: Gateway {
    fn save(&mut self, task: Task);
    fn remove(&mut self, task_id: TaskId);

    fn get_by_id(&self, task_id: TaskId) -> Option<Task>;

    fn show_reverse_chronologically_ordered(
        &self,
        pagination_request: PaginationRequest,
    ) -> PaginationResponse<Task>;
    fn show_reverse_chronologically_ordered_by_status(
        &self,
        status: TaskStatus,
        pagination_request: PaginationRequest,
    ) -> PaginationResponse<Task>;

    fn contains(&self, task_id: TaskId) -> bool {
        return self.get_by_id(task_id).is_some();
    }

    fn clear(&mut self);
    fn clear_by_status(&mut self, status: TaskStatus);
}
