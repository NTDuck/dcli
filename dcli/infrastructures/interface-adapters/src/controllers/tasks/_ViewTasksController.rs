use std::ops::Deref;

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
    pub pageSize: usize,
    pub maxPageSize: usize,
    pub pageNumber: usize,
    pub maxPageNumber: usize,
}

pub struct ViewableTask {
    pub id: u128,
    pub description: String,
    pub status: ViewableTaskStatus,
    pub createdAt: String,
}

pub type ViewableTaskStatus = TaskStatus;

impl From<ViewTasksResponseModel> for ViewTasksViewModel {
    fn from(response: ViewTasksResponseModel) -> Self {
        return Self {
            tasks: response.paginationResponse.items
                .into_iter()
                .map(|task| ViewableTask::from(task))
                .collect(),
            pageSize: response.paginationResponse.pageSize,
            maxPageSize: response.paginationResponse.maxPageSize,
            pageNumber: response.paginationResponse.pageNumber,
            maxPageNumber: response.paginationResponse.maxPageNumber,
        };
    }
}

impl From<Task> for ViewableTask {
    fn from(task: Task) -> Self {
        return ViewableTask {
            id: task.id.deref().clone(),
            description: task.description.deref().clone(),
            status: ViewableTaskStatus::from(task.status),
            createdAt: TimestampFormatter::format(task.createdAt),
        };
    }
}

pub type ViewTasksErrorViewModel = ViewTasksErrorModel;