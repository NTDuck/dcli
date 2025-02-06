use std::time::Instant;

use domain::Task;
use domain::TaskDescription;
use domain::TaskDescriptionError;
use domain::TaskStatus;
use lombok::New;

use crate::boundaries::tasks::CreateTaskBoundary;
use crate::boundaries::tasks::CreateTaskErrorModel;
use crate::boundaries::tasks::CreateTaskRequestModel;
use crate::boundaries::tasks::CreateTaskResponseModel;
use crate::gateways::repositories::tasks::TaskRepository;
use crate::gateways::factories::ids::UuidFactory;
use crate::utils::pointers::SharedPointer;

#[derive(New)]
pub struct CreateTaskInteractor {
    task_repository: SharedPointer<Box<dyn TaskRepository>>,
    uuid_factory: SharedPointer<Box<dyn UuidFactory>>,
}

impl CreateTaskBoundary for CreateTaskInteractor {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel> {
        let CreateTaskRequestModel {
            task_description,
        } = request;

        let task_description = match TaskDescription::try_from(task_description) {
            Ok(description) => description,
            Err(error) => match error {
                TaskDescriptionError::LengthUnderflow {
                    actual_length,
                    min_length_required,
                } => return Err(CreateTaskErrorModel::TaskDescriptionLengthUnderflow {
                    actual_length: actual_length,
                    min_length_required: min_length_required,
                }),
                TaskDescriptionError::LengthOverflow {
                    actual_length,
                    max_length_allowed,
                } => return Err(CreateTaskErrorModel::TaskDescriptionLengthOverflow {
                    actual_length: actual_length,
                    max_length_allowed: max_length_allowed,
                }),
            }
        };

        let uuid = self.uuid_factory.unwrap()
            .generate();

        let task = Task {
            id: uuid,
            description: task_description,
            status: TaskStatus::Pending,
            created_at: Instant::now(),
        };

        self.task_repository.unwrap_mut()
            .save(task);

        return Ok(CreateTaskResponseModel);
    }
}
