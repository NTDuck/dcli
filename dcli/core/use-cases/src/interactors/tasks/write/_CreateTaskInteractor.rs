use std::time::Instant;

use domain::Task;
use domain::TaskDescription;
use domain::TaskDescriptionError;
use domain::TaskStatus;

use crate::dataproviders::gateways::tasks::TaskGateway;
use crate::dataproviders::factories::ids::UuidFactory;
use crate::utils::contracts::interactors::FallibleMutableConsumerInteractor;

pub struct CreateTaskInteractor<'deps> {
    task_gateway: &'deps mut dyn TaskGateway,
    uuid_factory: &'deps dyn UuidFactory,
}

impl<'deps> FallibleMutableConsumerInteractor for CreateTaskInteractor<'deps> {
    fn consume(&mut self, request: Self::Request) -> Result<(), Self::Error> {
        let task_description = match TaskDescription::try_from(request.task_description) {
            Ok(description) => description,
            Err(error) => return Err(CreateTaskError::TaskDescription(error)),
        };

        let task_id = self.uuid_factory.generate();

        let task = Task {
            id: task_id,
            description: task_description,
            status: TaskStatus::Pending,
            created_at: Instant::now(),
        };

        self.task_gateway.save(&task);

        return Ok(());
    }

    type Request = CreateTaskRequestModel;
    type Error = CreateTaskError;
}

pub struct CreateTaskRequestModel {
    task_description: String,
}

pub enum CreateTaskError {
    TaskDescription(TaskDescriptionError),
}
