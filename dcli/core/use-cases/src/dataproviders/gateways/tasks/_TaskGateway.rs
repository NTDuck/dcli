use domain::Task;
use domain::TaskId;
use domain::TaskStatus;

use crate::dataproviders::gateways::common::PaginationParams;

pub trait TaskGateway {
    fn save(&mut self, task: &Task);
    fn remove(&mut self, task_id: TaskId);

    fn show(&self, pagination_params: PaginationParams);
    fn show_by_status(&self, status: TaskStatus, pagination_params: PaginationParams);

    fn clear(&mut self);
    fn clear_by_status(&mut self, status: TaskStatus);
}
