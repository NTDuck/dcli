use axiom::behaviours::New;
use domain::ids::Snowflake;
use domain::ids::SnowflakeWorkerNumber;
use domain::tasks::Task;
use domain::tasks::TaskDescription;
use domain::tasks::TaskDescriptionError;
use domain::tasks::TaskStatus;
use domain::time::Timestamp;

use crate::boundaries::tasks::CreateTaskBoundary;
use crate::boundaries::tasks::CreateTaskErrorModel;
use crate::boundaries::tasks::CreateTaskRequestModel;
use crate::boundaries::tasks::CreateTaskResponseModel;
use crate::gateways::encoders::ids::WorkerIdEncoder;
use crate::gateways::pointers::PointerHandle;
use crate::gateways::pointers::SharedPointer;
use crate::gateways::providers::ids::SnowflakeSequenceNumberProvider;
use crate::gateways::providers::ids::WorkerIdProvider;
use crate::gateways::repositories::tasks::TaskRepository;

#[derive(New)]
pub struct CreateTaskInteractor<Handle: PointerHandle> {
    workerIdEncoder: SharedPointer<Box<dyn WorkerIdEncoder>, Handle>,
    sequenceNumberProvider: SharedPointer<Box<dyn SnowflakeSequenceNumberProvider>, Handle>,
    workerIdProvider: SharedPointer<Box<dyn WorkerIdProvider>, Handle>,
    taskRepository: SharedPointer<Box<dyn TaskRepository>, Handle>,
}

impl<Handle: PointerHandle> CreateTaskBoundary for CreateTaskInteractor<Handle> {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel> {
        let taskDescription = TaskDescription::try_from(request.taskDescription)
            .map_err(|error| self.mapTaskDescriptionErrorToErrorModel(error))?;
        
        let currentTimestamp = Timestamp::current();
        let workerNumber = self.getWorkerNumber();
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

    fn getWorkerNumber(&self) -> SnowflakeWorkerNumber {
        let workerIdProvider = self.workerIdProvider.read();
        let workerId = workerIdProvider
            .getWorkerId();
        let workerNumber = self.workerIdEncoder.read()
            .encodeWorkerId(workerId);
        return workerNumber;
    }
}
