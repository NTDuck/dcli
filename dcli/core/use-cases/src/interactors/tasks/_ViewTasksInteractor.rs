use std::ops::Deref;

use axiom::behaviours::New;
use domain::Task;
use domain::TaskStatus;

use crate::boundaries::tasks::ViewTasksBoundary;
use crate::boundaries::tasks::ViewTasksErrorModel;
use crate::boundaries::tasks::ViewTasksRequestModel;
use crate::boundaries::tasks::ViewTasksResponseModel;
use crate::boundaries::tasks::ViewTasksTask;
use crate::boundaries::tasks::ViewTasksTaskStatus;
use crate::gateways::pointers::PointerHandle;
use crate::gateways::pointers::SharedPointer;
use crate::gateways::repositories::tasks::TaskRepository;
use crate::utils::dataclasses::pagination::PaginationResponse;

#[derive(New)]
pub struct ViewTasksInteractor<Handle: PointerHandle> {
    taskRepository: SharedPointer<Box<dyn TaskRepository>, Handle>,
}

impl<Handle: PointerHandle> ViewTasksBoundary for ViewTasksInteractor<Handle> {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel> {
        let paginationResponse = self.taskRepository.read()
            .showOrderedByCreatedAtDesc(request.paginationRequest);
        let paginationResponse = paginationResponse.into();

        return Ok(ViewTasksResponseModel {
            paginationResponse,
        });
    }
}

impl From<PaginationResponse<Task>> for PaginationResponse<ViewTasksTask> {
    fn from(paginationResponse: PaginationResponse<Task>) -> Self {
        return Self {
            items: paginationResponse.items
                .into_iter()
                .map(ViewTasksTask::from)
                .collect(),
            pageSize: paginationResponse.pageSize,
            maxPageSize: paginationResponse.maxPageSize,
            pageNumber: paginationResponse.pageNumber,
            maxPageNumber: paginationResponse.maxPageNumber,
        };
    }
}

impl From<Task> for ViewTasksTask {
    fn from(task: Task) -> Self {
        return Self {
            id: task.id.deref().clone(),
            description: task.description.deref().clone(),
            status: task.status.into(),
            createdAt: task.createdAt.deref().clone(),
        };
    }
}

impl From<TaskStatus> for ViewTasksTaskStatus {
    fn from(taskStatus: TaskStatus) -> Self {
        return match taskStatus {
            TaskStatus::Pending => Self::Pending,
            TaskStatus::InProgress => Self::InProgress,
            TaskStatus::Completed => Self::Completed,
        };
    }
}
