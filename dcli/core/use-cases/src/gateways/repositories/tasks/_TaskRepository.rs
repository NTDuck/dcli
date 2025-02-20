use domain::Task;
use domain::TaskStatus;

use crate::utils::dataclasses::pagination::PaginationProperties;
use crate::utils::dataclasses::pagination::PaginationRequest;
use crate::utils::dataclasses::pagination::PaginationResponse;

pub trait TaskRepository {
    fn save(&mut self, task: Task);
    fn remove(&mut self, taskId: TaskId);

    fn getById(&self, taskId: TaskId) -> Option<Task>;

    fn showOrderedByCreatedAtDesc(&self, paginationRequest: PaginationRequest) -> PaginationResponse<Task>;
    fn showByStatusOrderedByCreatedAtDesc(&self, status: TaskStatus, paginationRequest: PaginationRequest) -> PaginationResponse<Task>;

    fn size(&self) -> usize {
        return self.showOrderedByCreatedAtDesc(PaginationProperties::UnboundedPaginationRequest)
            .pageSize;
    }

    fn contains(&self, taskId: TaskId) -> bool {
        return self.getById(taskId)
            .is_some();
    }

    fn clear(&mut self);
    fn clearByStatus(&mut self, status: TaskStatus);
}
