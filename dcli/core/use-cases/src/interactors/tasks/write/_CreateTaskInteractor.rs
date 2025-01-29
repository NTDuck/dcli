use domain::Task;
use domain::TaskDescription;
use domain::TaskDescriptionError;
use domain::TaskStatus;

use crate::dataproviders::gateways::tasks::TaskGateway;
use crate::dataproviders::generators::UuidGenerator;
use crate::interactors::utils::contracts::FallibleMutableConsumerInteractor;

pub struct CreateTaskInteractor<'deps> {
    task_gateway: &'deps mut dyn TaskGateway,
    uuid_generator: &'deps dyn UuidGenerator,
}

impl<'deps> FallibleMutableConsumerInteractor for CreateTaskInteractor<'deps> {
    fn consume(&mut self, request: Self::Request) -> Result<(), Self::Error> {
        let description = match TaskDescription::try_from(request.description) {
            Ok(description) => description,
            Err(error) => return Err(CreateTaskError::TaskDescription(error)),
        };

        let uuid = self.uuid_generator.generate();

        let task = Task {
            id: uuid,
            description,
            status: TaskStatus::Pending,
        };

        self.task_gateway.save(&task);

        return Ok(());
    }

    type Request = CreateTaskRequest;
    type Error = CreateTaskError;
}

pub struct CreateTaskRequest {
    description: String,
}

pub enum CreateTaskError {
    TaskDescription(TaskDescriptionError),
}
