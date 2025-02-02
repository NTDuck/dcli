use domain::Task;
use domain::TaskDescription;
use domain::TaskDescriptionError;
use domain::TaskId;

use crate::dataproviders::gateways::tasks::TaskGateway;
use crate::utils::contracts::interactors::FallibleMutableConsumerInteractor;

pub struct UpdateTaskDescriptionInteractor<'deps> {
    task_gateway: &'deps mut dyn TaskGateway,
}

impl<'deps> FallibleMutableConsumerInteractor for UpdateTaskDescriptionInteractor<'deps> {
    fn consume(&mut self, request: Self::Request) -> Result<(), Self::Error> {
        let new_task_description = match TaskDescription::try_from(request.new_task_description) {
            Ok(description) => description,
            Err(error) => return Err(UpdateTaskDescriptionError::TaskDescription(error)),
        };

        let Some(existing_task) = self.task_gateway.get(request.task_id) else {
            return Err(UpdateTaskDescriptionError::TaskNotFound);
        };
        
        let new_task = Task {
            description: new_task_description,
            ..existing_task
        };

        self.task_gateway.save(&new_task);

        return Ok(());
    }

    type Request = UpdateTaskDescriptionRequest;
    type Error = UpdateTaskDescriptionError;
}

pub struct UpdateTaskDescriptionRequest {
    task_id: TaskId,
    new_task_description: String,
}

pub enum UpdateTaskDescriptionError {
    TaskNotFound,
    TaskDescription(TaskDescriptionError),
}
