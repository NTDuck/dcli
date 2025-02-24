use domain::tasks::Task;
use domain::tasks::TaskId;
use domain::tasks::TaskStatus;

use crate::utils::dataclasses::pagination::PaginationRequest;
use crate::utils::dataclasses::pagination::PaginationResponse;

pub trait TaskRepository {
    fn save(&mut self, task: Task);
    fn remove(&mut self, taskId: TaskId);

    fn getById(&self, taskId: TaskId) -> Option<Task>;

    fn showChronologicallyOrdered(&self, paginationRequest: PaginationRequest) -> PaginationResponse<Task>;
    fn showChronologicallyOrderedByStatus(&self, status: TaskStatus, paginationRequest: PaginationRequest) -> PaginationResponse<Task>;

    fn contains(&self, taskId: TaskId) -> bool {
        return self.getById(taskId).is_some();
    }

    fn clear(&mut self);
    fn clearByStatus(&mut self, status: TaskStatus);
}
