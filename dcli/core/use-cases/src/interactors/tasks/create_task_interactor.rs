use axiom::behaviours::New;
use domain::ids::Snowflake;
use domain::tasks::Task;
use domain::tasks::TaskDescription;
use domain::tasks::TaskDescriptionError;
use domain::tasks::TaskStatus;

use crate::boundaries::tasks::CreateTaskBoundary;
use crate::boundaries::tasks::CreateTaskErrResponseModel;
use crate::boundaries::tasks::CreateTaskOkResponseModel;
use crate::boundaries::tasks::CreateTaskRequestModel;
use crate::boundaries::tasks::CreateTaskResponseModel;
use crate::gateways::pointers::PointerHandle;
use crate::gateways::pointers::SharedPointer;
use crate::gateways::providers::ids::SnowflakeProvider;
use crate::gateways::providers::time::TimestampProvider;
use crate::gateways::repositories::tasks::TaskRepository;

pub struct CreateTaskInteractor<Handle: PointerHandle> {
    timestamp_provider: SharedPointer<Box<dyn TimestampProvider>, Handle>,
    snowflake_provider: SharedPointer<Box<dyn SnowflakeProvider>, Handle>,
    task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,

    response_model_assembler: CreateTaskResponseModelAssembler,
}

impl<Handle: PointerHandle> CreateTaskInteractor<Handle> {
    pub fn new(
        timestamp_provider: SharedPointer<Box<dyn TimestampProvider>, Handle>,
        snowflake_provider: SharedPointer<Box<dyn SnowflakeProvider>, Handle>,
        task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,
    ) -> Self {
        Self {
            timestamp_provider: timestamp_provider.clone(),
            snowflake_provider: snowflake_provider.clone(),
            task_repository: task_repository.clone(),
            response_model_assembler: CreateTaskResponseModelAssembler::new(),
        }
    }
}

impl<Handle: PointerHandle> CreateTaskBoundary
    for CreateTaskInteractor<Handle>
{
    fn apply(
        &self,
        request: CreateTaskRequestModel,
    ) -> CreateTaskResponseModel {
        let task_description = TaskDescription::try_from(
            request.task_description,
        )
        .map_err(|task_description_error| {
            self.response_model_assembler
                .assemble_from_task_description_error(task_description_error)
        })?;

        let current_timestamp =
            self.timestamp_provider.as_ref().get_current_timestamp();

        let snowflake_worker_number =
            self.snowflake_provider.as_ref().get_worker_number();
        let snowflake_sequence_number =
            self.snowflake_provider.as_ref().get_sequence_number();
        let snowflake = Snowflake::new(
            current_timestamp,
            snowflake_worker_number,
            snowflake_sequence_number,
        );

        let task = Task {
            id: snowflake,
            description: task_description,
            status: TaskStatus::Pending,
        };

        self.task_repository.as_mut().save(task);

        let response_model = Ok(CreateTaskOkResponseModel);
        response_model
    }
}

#[derive(New)]
struct CreateTaskResponseModelAssembler;

impl CreateTaskResponseModelAssembler {
    pub fn assemble_from_task_description_error(
        &self,
        task_description_error: TaskDescriptionError,
    ) -> CreateTaskErrResponseModel {
        match task_description_error {
            TaskDescriptionError::LengthUnderflow {
                actual_length,
                min_length_required,
            } => CreateTaskErrResponseModel::TaskDescriptionLengthUnderflow {
                actual_length,
                min_length_required,
            },
            TaskDescriptionError::LengthOverflow {
                actual_length,
                max_length_allowed,
            } => CreateTaskErrResponseModel::TaskDescriptionLengthOverflow {
                actual_length,
                max_length_allowed,
            },
        }
    }
}
