use domain::utils::Timestamp;
use domain::Task;
use domain::TaskDescription;
use domain::TaskDescriptionError;
use domain::TaskStatus;

use crate::boundaries::tasks::CreateTaskBoundary;
use crate::boundaries::tasks::CreateTaskErrorModel;
use crate::boundaries::tasks::CreateTaskRequestModel;
use crate::boundaries::tasks::CreateTaskResponseModel;
use crate::gateways::pointers::handles::abc::PointerHandle;
use crate::gateways::pointers::SharedPointer;
use crate::gateways::repositories::tasks::TaskRepository;
use crate::gateways::factories::ids::UuidFactory;

pub struct CreateTaskInteractor<Handle: PointerHandle> {
    task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,
    uuid_factory: SharedPointer<Box<dyn UuidFactory>, Handle>,
}

impl<Handle: PointerHandle> CreateTaskInteractor<Handle> {
    pub const fn new(task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>, uuid_factory: SharedPointer<Box<dyn UuidFactory>, Handle>) -> Self {
        return Self {
            task_repository,
            uuid_factory,
        };
    }
}

impl<Handle: PointerHandle> CreateTaskBoundary for CreateTaskInteractor<Handle> {
    fn apply(&self, request: CreateTaskRequestModel) -> Result<CreateTaskResponseModel, CreateTaskErrorModel> {
        let CreateTaskRequestModel {
            task_description,
        } = request;

        let task_description = match TaskDescription::try_from(task_description) {
            Ok(description) => description,
            Err(error) => match error {
                TaskDescriptionError::LengthUnderflow {
                    actual_length,
                    min_length_required,
                } => return Err(CreateTaskErrorModel::TaskDescriptionLengthUnderflow {
                    actual_length: actual_length,
                    min_length_required: min_length_required,
                }),
                TaskDescriptionError::LengthOverflow {
                    actual_length,
                    max_length_allowed,
                } => return Err(CreateTaskErrorModel::TaskDescriptionLengthOverflow {
                    actual_length: actual_length,
                    max_length_allowed: max_length_allowed,
                }),
            }
        };

        let uuid = self.uuid_factory.unwrap()
            .generate();

        let task = Task {
            id: uuid,
            description: task_description,
            status: TaskStatus::Pending,
            created_at: Timestamp::now(),
        };

        self.task_repository.unwrap_mut()
            .save(task);

        return Ok(CreateTaskResponseModel);
    }
}
