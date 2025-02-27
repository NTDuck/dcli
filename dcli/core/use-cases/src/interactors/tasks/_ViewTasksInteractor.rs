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
        let pagination_response = self.taskRepository.as_ref()
            .show_reverse_chronologically_ordered(request.pagination_request);
        let response_model = self.map_pagination_response_to_response_model(pagination_response);

        return Ok(response_model);
    }
}

impl<Handle: PointerHandle> ViewTasksInteractor<Handle> {
    fn map_pagination_response_to_response_model(&self, pagination_response: PaginationResponse<Task>) -> ViewTasksResponseModel {
        return ViewTasksResponseModel {
            pagination_response: PaginationResponse {
                items: pagination_response.items
                    .into_iter()
                    .map(|task| self.mapTaskToTaskModel(task))
                    .collect(),

                page_size: pagination_response.page_size,
                max_page_size: pagination_response.max_page_size,
                page_number: pagination_response.page_number,
                max_page_number: pagination_response.max_page_number,
            },
        };
    }

    fn mapTaskToTaskModel(&self, task: Task) -> ViewTasksTaskModel {
        return ViewTasksTaskModel {
            id: *task.id.deref(),
            description: task.description.deref().clone(),
            status: self.mapTaskStatusToTaskStatusModel(task.status),
            created_at: task.id.get_timestamp().as_system_time(),
        };
    }

    fn mapTaskStatusToTaskStatusModel(&self, task_status: TaskStatus) -> ViewTasksTaskStatusModel {
        return match task_status {
            TaskStatus::Pending => ViewTasksTaskStatusModel::Pending,
            TaskStatus::InProgress => ViewTasksTaskStatusModel::InProgress,
            TaskStatus::Completed => ViewTasksTaskStatusModel::Completed,
        };
    }
}
