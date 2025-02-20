use axiom::behaviours::New;
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
        let CreateTaskRequestModel {
            taskDescription,
        } = request;

        let taskDescription = match TaskDescription::try_from(taskDescription) {
            Ok(taskDescription) => taskDescription,
            Err(error) => match error {
                TaskDescriptionError::LengthUnderflow {
                    actualLength,
                    minLengthRequired,
                } => return Err(CreateTaskErrorModel::TaskDescriptionLengthUnderflow {
                    actualLength,
                    minLengthRequired,
                }),
                TaskDescriptionError::LengthOverflow {
                    actualLength,
                    maxLengthAllowed,
                } => return Err(CreateTaskErrorModel::TaskDescriptionLengthOverflow {
                    actualLength,
                    maxLengthAllowed,
                }),
            }
        };

        let uuid = self.uuidFactory.read()
            .generate();

        let task = Task {
            id: uuid,
            description: taskDescription,
            status: TaskStatus::Pending,
            createdAt: Timestamp::now(),
        };

        self.taskRepository.write()
            .save(task);

        return Ok(CreateTaskResponseModel);
    }
}
