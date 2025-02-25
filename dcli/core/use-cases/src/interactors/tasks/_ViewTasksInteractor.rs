use std::ops::Deref;

use axiom::behaviours::New;
use domain::tasks::Task;
use domain::tasks::TaskStatus;

use crate::boundaries::tasks::ViewTasksBoundary;
use crate::boundaries::tasks::ViewTasksErrorModel;
use crate::boundaries::tasks::ViewTasksRequestModel;
use crate::boundaries::tasks::ViewTasksResponseModel;
use crate::boundaries::tasks::ViewTasksTaskModel;
use crate::boundaries::tasks::ViewTasksTaskStatusModel;
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
            .showReverseChronologicallyOrdered(request.paginationRequest);
        let responseModel = self.mapPaginationResponseToResponseModel(paginationResponse);

        return Ok(responseModel);
    }
}

impl<Handle: PointerHandle> ViewTasksInteractor<Handle> {
    fn mapPaginationResponseToResponseModel(&self, paginationResponse: PaginationResponse<Task>) -> ViewTasksResponseModel {
        return ViewTasksResponseModel {
            paginationResponse: PaginationResponse {
                items: paginationResponse.items
                    .into_iter()
                    .map(|task| self.mapTaskToTaskModel(task))
                    .collect(),

                pageSize: paginationResponse.pageSize,
                maxPageSize: paginationResponse.maxPageSize,
                pageNumber: paginationResponse.pageNumber,
                maxPageNumber: paginationResponse.maxPageNumber,
            },
        };
    }

    fn mapTaskToTaskModel(&self, task: Task) -> ViewTasksTaskModel {
        return ViewTasksTaskModel {
            id: *task.id.deref(),
            description: task.description.deref().clone(),
            status: self.mapTaskStatusToTaskStatusModel(task.status),
            createdAt: task.id.getTimestamp().asSystemTime(),
        };
    }

    fn mapTaskStatusToTaskStatusModel(&self, taskStatus: TaskStatus) -> ViewTasksTaskStatusModel {
        return match taskStatus {
            TaskStatus::Pending => ViewTasksTaskStatusModel::Pending,
            TaskStatus::InProgress => ViewTasksTaskStatusModel::InProgress,
            TaskStatus::Completed => ViewTasksTaskStatusModel::Completed,
        };
    }
}
