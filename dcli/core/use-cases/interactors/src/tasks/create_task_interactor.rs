use axiom::behaviours::New;
use boundaries::tasks::CreateTaskErrResponseModel;
use boundaries::tasks::CreateTaskInputBoundary;
use boundaries::tasks::CreateTaskOkResponseModel;
use boundaries::tasks::CreateTaskOutputBoundary;
use boundaries::tasks::CreateTaskRequestModel;
use boundaries::tasks::CreateTaskResponseModel;
use domain::tasks::Task;
use domain::tasks::TaskDescription;
use domain::tasks::TaskDescriptionError;
use domain::tasks::TaskId;
use domain::tasks::TaskStatus;
use gateways::pointers::PointerHandle;
use gateways::providers::ids::SnowflakeProvider;
use gateways::providers::time::TimestampProvider;
use gateways::repositories::tasks::TaskRepository;

use crate::utils::pointers::SharedPointer;

pub struct CreateTaskInteractor<Handle: PointerHandle> {
    output_boundary: SharedPointer<Box<dyn CreateTaskOutputBoundary>, Handle>,
    
    timestamp_provider: SharedPointer<Box<dyn TimestampProvider>, Handle>,
    snowflake_provider: SharedPointer<Box<dyn SnowflakeProvider>, Handle>,
    task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,

    response_model_assembler: CreateTaskResponseModelAssembler,
}

impl<Handle: PointerHandle> CreateTaskInteractor<Handle> {
    pub fn new(
        output_boundary: SharedPointer<Box<dyn CreateTaskOutputBoundary>, Handle>,
        timestamp_provider: SharedPointer<Box<dyn TimestampProvider>, Handle>,
        snowflake_provider: SharedPointer<Box<dyn SnowflakeProvider>, Handle>,
        task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,
    ) -> Self {
        return Self {
            output_boundary,
            timestamp_provider,
            snowflake_provider,
            task_repository,
            response_model_assembler: CreateTaskResponseModelAssembler::new(),
        };
    }
}

impl<Handle: PointerHandle> CreateTaskInputBoundary for CreateTaskInteractor<Handle> {
    fn accept(&self, request: CreateTaskRequestModel) {
        let task_description = match TaskDescription::try_from(request.task_description) {
            Ok(task_description) => task_description,
            Err(task_description_error) => {
                let response_model = self.response_model_assembler.assemble_from_task_description_error(task_description_error);
                self.output_boundary.as_ref().accept(response_model);
                return;
            },
        };

        let current_timestamp = self.timestamp_provider.as_ref().get_current_timestamp();

        let snowflake_worker_number = self.snowflake_provider.as_ref().get_worker_number();
        let snowflake_sequence_number = self.snowflake_provider.as_ref().get_sequence_number();
        
        let task_id = TaskId::new(current_timestamp, snowflake_worker_number, snowflake_sequence_number);
        let task = Task {
            id: task_id,
            description: task_description,
            status: TaskStatus::Pending,
        };

        self.task_repository.as_mut().save(task);

        let response_model = Ok(CreateTaskOkResponseModel);
        self.output_boundary.as_ref().accept(response_model);
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
