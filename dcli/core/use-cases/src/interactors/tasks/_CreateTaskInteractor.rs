use axiom::behaviours::New;
use domain::tasks::Task;
use domain::tasks::TaskDescription;
use domain::tasks::TaskDescriptionError;
use domain::tasks::TaskStatus;

use crate::boundaries::tasks::CreateTaskBoundary;
use crate::boundaries::tasks::CreateTaskErrorModel;
use crate::boundaries::tasks::CreateTaskRequestModel;
use crate::boundaries::tasks::CreateTaskResponseModel;
use crate::gateways::factories::ids::MachineIdFactory;
use crate::gateways::factories::ids::SnowflakeFactory;
use crate::gateways::factories::time::TimestampFactory;
use crate::gateways::pointers::PointerHandle;
use crate::gateways::pointers::SharedPointer;
use crate::gateways::repositories::tasks::TaskRepository;

#[derive(New)]
pub struct CreateTaskInteractor<Handle: PointerHandle> {
    taskRepository: SharedPointer<Box<dyn TaskRepository>, Handle>,
    snowflakeFactory: SharedPointer<Box<dyn SnowflakeFactory>, Handle>,
    timestampFactory: SharedPointer<Box<dyn TimestampFactory>, Handle>,
    machineIdFactory: SharedPointer<Box<dyn MachineIdFactory>, Handle>,
}

impl<Handle: PointerHandle> CreateTaskBoundary for CreateTaskInteractor<Handle> {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel> {
        let taskDescription = TaskDescription::try_from(request.taskDescription)
            .map_err(|error| self.mapTaskDescriptionErrorToErrorModel(error))?;
        
        let timestamp = self.timestampFactory.read()
            .currentTimestamp();
        let machineId = self.machineIdFactory.read()
            .getMachineId();
        let snowflake = self.snowflakeFactory.read()
            .newSnowflake(timestamp, machineId);

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
