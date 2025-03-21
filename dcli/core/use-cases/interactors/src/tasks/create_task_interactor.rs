use std::ops::Deref;

use axiom::behaviours::New;
use boundaries::tasks::CreateTaskErrResponseModel;
use boundaries::tasks::CreateTaskInputBoundary;
use boundaries::tasks::CreateTaskOkResponseModel;
use boundaries::tasks::CreateTaskOutputBoundary;
use boundaries::tasks::CreateTaskResponseModel;
use domain::tasks::Task;
use domain::tasks::TaskDescription;
use domain::tasks::TaskDescriptionError;
use domain::tasks::TaskStatus;
use gateways::pointers::PointerHandle;
use gateways::providers::ids::SnowflakeProvider;
use gateways::providers::time::TimestampProvider;
use gateways::repositories::tasks::TaskRepository;

use crate::utils::pointers::SharedPointer;

#[derive(New)]
pub struct CreateTaskInteractor<Handle: PointerHandle> {
    output_boundary: SharedPointer<Box<dyn CreateTaskOutputBoundary>, Handle>,
    
    timestamp_provider: SharedPointer<Box<dyn TimestampProvider>, Handle>,
    snowflake_provider: SharedPointer<Box<dyn SnowflakeProvider>, Handle>,
    task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,

    task_assembler: TaskAssembler<Handle>,
    response_model_assembler: CreateTaskResponseModelAssembler,
}

impl<Handle: PointerHandle> CreateTaskInputBoundary for CreateTaskInteractor<Handle> {
    fn accept(&self, request: boundaries::tasks::CreateTaskRequestModel) {
        let task_description = TaskDescription::try_from(request.task_description)
            .map_err(|err| self.response_model_assembler.assemble_from_task_description_error(err));

        let task = self.task_assembler.assemble(task_description);

        self.task_repository.as_mut().save(task);

        self.output_boundary.as_ref().accept(Ok(CreateTaskOkResponseModel));
    }    
}

#[derive(New)]
struct TaskAssembler<Handle: PointerHandle> {
    timestamp_provider: SharedPointer<Box<dyn TimestampProvider>, Handle>,
    snowflake_provider: SharedPointer<Box<dyn SnowflakeProvider>, Handle>,
}

impl<Handle: PointerHandle> TaskAssembler<Handle> {
    pub fn assemble(&self, task_description: TaskDescription) -> Task {
        let current_timestamp = self.timestamp_provider.as_ref().get_current_timestamp();
        let snowflake = self.snowflake_provider.as_ref().new_snowflake_from_timestamp(current_timestamp);

        return Task {
            id: snowflake,
            description: task_description,
            status: TaskStatus::Pending,
        };
    }
}

#[derive(New)]
struct CreateTaskResponseModelAssembler;

impl CreateTaskResponseModelAssembler {
    pub fn assemble_from_task_description_error(&self, task_description_error: TaskDescriptionError) -> CreateTaskResponseModel {
        return match task_description_error {
            TaskDescriptionError::LengthUnderflow {
                actual_length,
                min_length_required,
            } => Err(CreateTaskErrResponseModel::TaskDescriptionLengthUnderflow {
                actual_length,
                min_length_required,
            }),
            TaskDescriptionError::LengthOverflow {
                actual_length,
                max_length_allowed,
            } => Err(CreateTaskErrResponseModel::TaskDescriptionLengthOverflow {
                actual_length,
                max_length_allowed,
            }),
        };
    }
}
