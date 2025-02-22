use domain::utils::dataclasses::ids::Uuid;
use domain::Task;
use domain::TaskStatus;

use crate::utils::dataclasses::pagination::PaginationProperties;
use crate::utils::dataclasses::pagination::PaginationRequest;
use crate::utils::dataclasses::pagination::PaginationResponse;

pub trait TaskRepository {
    fn save(&mut self, task: Task);
    fn remove(&mut self, taskId: Uuid);

    fn getById(&self, taskId: Uuid) -> Option<Task>;

    fn showOrderedByCreatedAtDesc(&self, paginationRequest: PaginationRequest) -> PaginationResponse<Task>;
    fn showByStatusOrderedByCreatedAtDesc(&self, status: TaskStatus, paginationRequest: PaginationRequest) -> PaginationResponse<Task>;

    fn size(&self) -> usize {
        let paginationResponse = self.showOrderedByCreatedAtDesc(PaginationProperties::UnboundedPaginationRequest);
        return paginationResponse.pageSize;
    }

    fn contains(&self, taskId: Uuid) -> bool {
        return self.getById(taskId).is_some();
    }

    fn clear(&mut self);
    fn clearByStatus(&mut self, status: TaskStatus);
}
