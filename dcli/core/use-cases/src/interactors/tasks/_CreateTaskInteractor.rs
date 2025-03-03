use std::ops::Deref;

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
use crate::utils::interfaces::DeferredNewFrom;

#[derive(New)]
pub struct CreateTaskInteractor<Handle: PointerHandle> {
    timestamp_provider: SharedPointer<Box<dyn TimestampProvider>, Handle>,
    snowflake_provider: SharedPointer<Box<dyn SnowflakeProvider>, Handle>,
    task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,
}

impl<Handle: PointerHandle> CreateTaskBoundary for CreateTaskInteractor<Handle> {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel> {
        let task_description = TaskDescription::try_from(request.task_description)
            .map_err(CreateTaskErrorModel::from)?;
        
        let task = Task::new_from(task_description)
            .using((self.timestamp_provider.as_ref(), self.snowflake_provider.as_ref()));

        self.task_repository.as_mut().save(task);

        return Ok(CreateTaskResponseModel);
    }
}

impl<TimestampProviderRef, SnowflakeProviderRef> DeferredNewFrom<TaskDescription, (TimestampProviderRef, SnowflakeProviderRef)> for Task
where
    TimestampProviderRef: Deref<Target = Box<dyn TimestampProvider>>,
    SnowflakeProviderRef: Deref<Target = Box<dyn SnowflakeProvider>>,
{
    fn new(task_description: TaskDescription, (timestamp_provider, snowflake_provider): (TimestampProviderRef, SnowflakeProviderRef)) -> Self {
        let current_timestamp = timestamp_provider.get_current_timestamp();
        let snowflake = snowflake_provider.new_snowflake_from_timestamp(current_timestamp);

        return Self {
            id: snowflake,
            description: task_description,
            status: TaskStatus::Pending,
        };
    }
}

impl From<TaskDescriptionError> for CreateTaskErrorModel {
    fn from(error: TaskDescriptionError) -> Self {
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

