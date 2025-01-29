use domain::Task;
use domain::TaskId;
use domain::TaskStatus;

use crate::dataproviders::gateways::tasks::TaskGateway;
use crate::interactors::utils::contracts::FallibleMutableConsumerInteractor;

pub struct UpdateTaskStatusInteractor<'deps> {
    task_gateway: &'deps mut dyn TaskGateway,
}

impl<'deps> FallibleMutableConsumerInteractor for UpdateTaskStatusInteractor<'deps> {
    fn consume(&mut self, request: Self::Request) -> Result<(), Self::Error> {
        let Some(existing_task) = self.task_gateway.get(request.task_id) else {
            return Err(UpdateTaskStatusError::TaskNotFound);
        };

        let new_status = match existing_task.status {
            TaskStatus::Pending => TaskStatus::InProgress,
            TaskStatus::InProgress => TaskStatus::Completed,
            _ => existing_task.status,
        };

        let new_task = Task {
            status: new_status,
            ..existing_task
        };

        self.task_gateway.save(&new_task);

        return Ok(());
    }

    type Request = UpdateTaskStatusRequest;
    type Error = UpdateTaskStatusError;
}

pub struct UpdateTaskStatusRequest {
    task_id: TaskId,
}

pub enum UpdateTaskStatusError {
    TaskNotFound,
}
