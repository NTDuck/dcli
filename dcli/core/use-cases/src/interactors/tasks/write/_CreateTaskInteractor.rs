use std::time::Instant;

use domain::Task;
use domain::TaskDescription;
use domain::TaskDescriptionError;
use domain::TaskStatus;

use crate::gateways::repositories::tasks::TaskRepository;
use crate::gateways::factories::ids::UuidFactory;
use crate::utils::contracts::interactors::FallibleMutableConsumerInteractor;

pub struct CreateTaskInteractor<'deps> {
    task_repository: &'deps mut dyn TaskRepository,
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

        self.task_repository.save(&task);

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
