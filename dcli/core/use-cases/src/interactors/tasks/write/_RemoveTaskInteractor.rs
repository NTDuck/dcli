use domain::TaskId;

use crate::dataproviders::gateways::tasks::TaskGateway;
use crate::interactors::utils::contracts::FallibleMutableConsumerInteractor;

pub struct RemoveTaskInteractor<'deps> {
    task_gateway: &'deps mut dyn TaskGateway,
}

impl<'deps> FallibleMutableConsumerInteractor for RemoveTaskInteractor<'deps> {
    fn consume(&mut self, request: Self::Request) -> Result<(), Self::Error> {
        if self.task_gateway.contains(request.task_id) {
            return Err(RemoveTaskError::TaskNotFound);
        }

        self.task_gateway.remove(request.task_id);

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
