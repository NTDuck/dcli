use domain::Task;
use domain::TaskStatus;
use use_cases::boundaries::tasks::ViewTasksBoundary;
use use_cases::boundaries::tasks::ViewTasksErrorModel;
use use_cases::boundaries::tasks::ViewTasksRequestModel;
use use_cases::boundaries::tasks::ViewTasksResponseModel;

use crate::utils::formatters::TimestampFormatter;

pub struct ViewTasksController<'bdrs> {
    interactor: &'bdrs dyn ViewTasksBoundary,
}

impl<'bdrs> ViewTasksController<'bdrs> {
    pub const fn new(interactor: &'bdrs dyn ViewTasksBoundary) -> Self {
        return Self {
            interactor,
        };
    }
}

impl<'bdrs> ViewTasksController<'bdrs> {
    pub fn apply(&self, request: ViewTasksRequestObject) -> Result<ViewTasksViewModel, ViewTasksErrorViewModel> {
        let request = request.into();
        return match self.interactor.apply(request) {
            Ok(response) => Ok(ViewTasksViewModel::from(response)),
            Err(error) => Err(ViewTasksErrorViewModel::from(error)),
        };
    }
}

pub type ViewTasksRequestObject = ViewTasksRequestModel;

pub struct ViewTasksViewModel {
    pub tasks: Vec<ViewableTask>,
    pub page_size: usize,
    pub max_page_size: usize,
    pub page_number: usize,
    pub max_page_number: usize,
}

pub struct ViewableTask {
    pub id: u128,
    pub description: String,
    pub status: ViewableTaskStatus,
    pub created_at: String,
}

pub type ViewableTaskStatus = TaskStatus;

impl From<ViewTasksResponseModel> for ViewTasksViewModel {
    fn from(response: ViewTasksResponseModel) -> Self {
        return Self {
            tasks: response.pagination_response.items
                .into_iter()
                .map(|task| ViewableTask::from(task))
                .collect(),
            page_size: response.pagination_response.page_size,
            max_page_size: response.pagination_response.max_page_size,
            page_number: response.pagination_response.page_number,
            max_page_number: response.pagination_response.max_page_number,
        };
    }
}

impl From<Task> for ViewableTask {
    fn from(task: Task) -> Self {
        return ViewableTask {
            id: *task.id,
            description: task.description.to_string(),
            status: ViewableTaskStatus::from(task.status),
            created_at: TimestampFormatter::format(task.createdAt),
        };
    }
}

pub type ViewTasksErrorViewModel = ViewTasksErrorModel;