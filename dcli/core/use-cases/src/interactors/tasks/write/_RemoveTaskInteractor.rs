use domain::TaskId;

use crate::gateways::repositories::tasks::TaskRepository;
use crate::utils::contracts::interactors::FallibleMutableConsumerInteractor;

pub struct RemoveTaskInteractor<'deps> {
    task_repository: &'deps mut dyn TaskRepository,
}

impl<'deps> FallibleMutableConsumerInteractor for RemoveTaskInteractor<'deps> {
    fn consume(&mut self, request: Self::Request) -> Result<(), Self::Error> {
        if self.task_repository.contains(request.task_id) {
            return Err(RemoveTaskError::TaskNotFound);
        }

        self.task_repository.remove(request.task_id);

        return Ok(());
    }

    type Request = RemoveTaskRequest;
    type Error = RemoveTaskError;
}

pub struct RemoveTaskRequest {
    task_id: TaskId,
}

pub enum RemoveTaskError {
    TaskNotFound,
}
