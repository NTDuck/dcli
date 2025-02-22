use axiom::behaviours::New;
use domain::utils::dataclasses::ids::Uuid;
use domain::utils::dataclasses::time::Timestamp;
use domain::Task;
use domain::TaskDescription;
use domain::TaskDescriptionError;
use domain::TaskStatus;

use crate::boundaries::tasks::CreateTaskBoundary;
use crate::boundaries::tasks::CreateTaskErrorModel;
use crate::boundaries::tasks::CreateTaskRequestModel;
use crate::boundaries::tasks::CreateTaskResponseModel;
use crate::gateways::pointers::PointerHandle;
use crate::gateways::pointers::SharedPointer;
use crate::gateways::repositories::tasks::TaskRepository;
use crate::gateways::factories::ids::UuidFactory;

#[derive(New)]
pub struct CreateTaskInteractor<Handle: PointerHandle> {
    taskRepository: SharedPointer<Box<dyn TaskRepository>, Handle>,
    uuidFactory: SharedPointer<Box<dyn UuidFactory>, Handle>,
}

impl<Handle: PointerHandle> CreateTaskBoundary for CreateTaskInteractor<Handle> {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel> {
        let taskDescription = TaskDescription::try_from(request.taskDescription)
            .map_err(TaskDescriptionError::from)?;
        
        let uuid = self.uuidFactory.read()
            .generate();
        let task = createTaskFromIdAndDescription(uuid, taskDescription);

        self.taskRepository.write()
            .save(task);

        return Ok(CreateTaskResponseModel);
    }
}

impl From<TaskDescriptionError> for CreateTaskErrorModel {
    fn from(error: TaskDescriptionError) -> Self {
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

fn createTaskFromIdAndDescription(id: Uuid, description: TaskDescription) -> Task {
    return Task {
        id,
        description,
        status: TaskStatus::Pending,
        createdAt: Timestamp::now(),
    };
}