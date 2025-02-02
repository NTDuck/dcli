use domain::Task;
use domain::TaskStatus;
use use_cases::contracts::interactors::FunctionInteractor;
use use_cases::interactors::tasks::ViewTasksInteractor;
use use_cases::interactors::tasks::ViewTasksRequestModel;
use use_cases::interactors::tasks::ViewTasksResponseModel;

use crate::utils::adapters::TimestampAdapter;

pub struct ViewTasksController<'int, 'deps> {
    interactor: &'int ViewTasksInteractor<'deps>,
}

impl<'int, 'deps> ViewTasksController<'int, 'deps> {
    pub fn apply(&self, request_object: ViewTasksRequestObject) -> ViewTasksViewModel {
        let request_model = request_object;
        let response_model = self.interactor.apply(request_model);
        let view_model = ViewTasksViewModel::from(response_model);

        return view_model;
    }
}

pub type ViewTasksRequestObject = ViewTasksRequestModel;

pub struct ViewTasksViewModel {
    tasks: Vec<ViewableTask>,
}

pub struct ViewableTask {
    pub id: u128,
    pub description: String,
    pub status: ViewableTaskStatus,
    pub created_at: String,
}

pub type ViewableTaskStatus = TaskStatus;

impl From<ViewTasksResponseModel> for ViewTasksViewModel {
    fn from(response_model: ViewTasksResponseModel) -> Self {
        return Self {
            tasks: response_model.tasks
                .into_iter()
                .map(|task| ViewableTask::from(task))
                .collect(),
        };
    }
}

impl From<Task> for ViewableTask {
    fn from(task: Task) -> Self {
        return ViewableTask {
            id: *task.id,
            description: task.description.to_string(),
            status: task.status,
            created_at: TimestampAdapter::format(task.created_at),
        };
    }
}
