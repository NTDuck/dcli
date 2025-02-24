use axiom::behaviours::New;
use domain::ids::Snowflake;
use domain::tasks::Task;
use domain::tasks::TaskDescription;
use domain::tasks::TaskDescriptionError;
use domain::tasks::TaskStatus;
use domain::time::Timestamp;

use crate::boundaries::tasks::CreateTaskBoundary;
use crate::boundaries::tasks::CreateTaskErrorModel;
use crate::boundaries::tasks::CreateTaskRequestModel;
use crate::boundaries::tasks::CreateTaskResponseModel;
use crate::gateways::pointers::PointerHandle;
use crate::gateways::pointers::SharedPointer;
use crate::gateways::providers::ids::SequenceNumberProvider;
use crate::gateways::providers::ids::WorkerNumberProvider;
use crate::gateways::repositories::tasks::TaskRepository;

#[derive(New)]
pub struct CreateTaskInteractor<Handle: PointerHandle> {
    taskRepository: SharedPointer<Box<dyn TaskRepository>, Handle>,
    workerNumberProvider: SharedPointer<Box<dyn WorkerNumberProvider>, Handle>,
    sequenceNumberProvider: SharedPointer<Box<dyn SequenceNumberProvider>, Handle>,
}

impl<Handle: PointerHandle> CreateTaskBoundary for CreateTaskInteractor<Handle> {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel> {
        let taskDescription = TaskDescription::try_from(request.taskDescription)
            .map_err(|error| self.mapTaskDescriptionErrorToErrorModel(error))?;
        
        let currentTimestamp = Timestamp::current();
        let workerNumber = self.workerNumberProvider.read()
            .getWorkerNumber();
        let sequenceNumber = self.sequenceNumberProvider.read()
            .getSequenceNumber();

        let snowflake = Snowflake::new(currentTimestamp, workerNumber, sequenceNumber);

        let task = Task {
            id: snowflake,
            description: taskDescription,
            status: TaskStatus::Pending,
        };

        self.taskRepository.write()
            .save(task);

        return Ok(CreateTaskResponseModel);
    }
}

impl<Handle: PointerHandle> CreateTaskInteractor<Handle> {
    fn mapTaskDescriptionErrorToErrorModel(&self, error: TaskDescriptionError) -> CreateTaskErrorModel {
        return match error {
            TaskDescriptionError::LengthUnderflow {
                actualLength,
                minLengthRequired,
            } => CreateTaskErrorModel::TaskDescriptionLengthUnderflow {
                actualLength,
                minLengthRequired,
            },
            TaskDescriptionError::LengthOverflow {
                actualLength,
                maxLengthAllowed,
            } => CreateTaskErrorModel::TaskDescriptionLengthOverflow {
                actualLength,
                maxLengthAllowed,
            },
        };
    }
}
