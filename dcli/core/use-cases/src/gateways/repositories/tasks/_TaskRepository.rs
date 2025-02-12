use domain::Task;
use domain::TaskId;
use domain::TaskStatus;

use crate::utils::dataclasses::pagination::PaginationProperties;
use crate::utils::dataclasses::pagination::PaginationRequest;
use crate::utils::dataclasses::pagination::PaginationResponse;

pub trait TaskRepository {
    fn save(&mut self, task: Task);
    fn remove(&mut self, task_id: TaskId);

    fn get(&self, task_id: TaskId) -> Option<Task>;

    fn show_most_recent(&self, pagination_request: PaginationRequest) -> PaginationResponse<Task>;
    fn show_most_recent_by_status(&self, status: TaskStatus, pagination_request: PaginationRequest) -> PaginationResponse<Task>;

    fn size(&self) -> usize {
        let pagination_request = PaginationRequest {
            page_number: PaginationProperties::MIN_PAGE_SIZE,
            max_page_size: usize::MAX,
        };
        
        return self.show_most_recent(pagination_request)
            .page_size;
    }

    fn contains(&self, task_id: TaskId) -> bool;

    fn clear(&mut self);
    fn clear_by_status(&mut self, status: TaskStatus);
}
