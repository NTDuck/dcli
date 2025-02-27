use axiom::behaviours::New;
use domain::tasks::Task;
use domain::tasks::TaskDescription;
use domain::tasks::TaskDescriptionError;
use domain::tasks::TaskStatus;

use crate::boundaries::tasks::CreateTaskBoundary;
use crate::boundaries::tasks::CreateTaskErrorModel;
use crate::boundaries::tasks::CreateTaskRequestModel;
use crate::boundaries::tasks::CreateTaskResponseModel;
use crate::gateways::pointers::PointerHandle;
use crate::gateways::pointers::SharedPointer;
use crate::gateways::providers::ids::SnowflakeProvider;
use crate::gateways::providers::time::TimestampProvider;
use crate::gateways::repositories::tasks::TaskRepository;

#[derive(New)]
pub struct CreateTaskInteractor<Handle: PointerHandle> {
    timestamp_provider: SharedPointer<Box<dyn TimestampProvider>, Handle>,
    snowflake_provider: SharedPointer<Box<dyn SnowflakeProvider>, Handle>,
    task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,
}

impl<Handle: PointerHandle> CreateTaskBoundary for CreateTaskInteractor<Handle> {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel> {
        let task_description = TaskDescription::try_from(request.task_description)
            .map_err(|error| self.map_task_description_error_to_error_model(error))?;
        
        let current_timestamp = self.timestamp_provider.as_ref()
            .get_current_timestamp();
        let task_id = self.snowflake_provider.as_ref()
            .new_snowflake_from_timestamp(current_timestamp);

        let task = Task {
            id: task_id,
            description: task_description,
            status: TaskStatus::Pending,
        };

        self.task_repository.as_mut()
            .save(task);

        return Ok(CreateTaskResponseModel);
    }
}

impl<Handle: PointerHandle> CreateTaskInteractor<Handle> {
    fn map_task_description_error_to_error_model(&self, error: TaskDescriptionError) -> CreateTaskErrorModel {
        return match error {
            TaskDescriptionError::LengthUnderflow {
                actual_length,
                min_length_required,
            } => CreateTaskErrorModel::TaskDescriptionLengthUnderflow {
                actual_length,
                min_length_required,
            },
            TaskDescriptionError::LengthOverflow {
                actual_length,
                max_length_allowed,
            } => CreateTaskErrorModel::TaskDescriptionLengthOverflow {
                actual_length,
                max_length_allowed,
            },
        };
    }
}
