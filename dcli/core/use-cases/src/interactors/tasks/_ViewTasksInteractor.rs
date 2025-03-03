use std::ops::Deref;

use axiom::behaviours::New;
use domain::tasks::Task;

use crate::boundaries::tasks::TaskModel;
use crate::boundaries::tasks::ViewTasksBoundary;
use crate::boundaries::tasks::ViewTasksErrorModel;
use crate::boundaries::tasks::ViewTasksRequestModel;
use crate::boundaries::tasks::ViewTasksResponseModel;
use crate::gateways::formatters::time::TimestampFormatter;
use crate::gateways::pointers::PointerHandle;
use crate::gateways::pointers::SharedPointer;
use crate::gateways::repositories::tasks::TaskRepository;
use crate::utils::dataclasses::pagination::PaginationResponse;
use crate::utils::interfaces::DeferredNewFrom;

#[derive(New)]
pub struct ViewTasksInteractor<Handle: PointerHandle> {
    timestamp_formatter: SharedPointer<Box<dyn TimestampFormatter>, Handle>,
    task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,
}

impl<Handle: PointerHandle> ViewTasksBoundary for ViewTasksInteractor<Handle> {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel> {
        let pagination_response = self.task_repository.as_ref().show_reverse_chronologically_ordered(request.pagination_request);
        let response_model = ViewTasksResponseModel::new_from(pagination_response)
            .using(self.timestamp_formatter.as_ref());

        return Ok(response_model);
    }
}

impl<TimestampFormatterRef> DeferredNewFrom<PaginationResponse<Task>, TimestampFormatterRef> for ViewTasksResponseModel
where
    TimestampFormatterRef: Deref<Target = Box<dyn TimestampFormatter>>,
{
    fn new(pagination_response: PaginationResponse<Task>, timestamp_formatter: TimestampFormatterRef) -> Self {
        return Self {
            pagination_response: PaginationResponse {
                items: pagination_response.items
                    .into_iter()
                    .map(|task| TaskModel::new_from(task)
                        .using(timestamp_formatter.deref()))
                    .collect(),
                page_size: pagination_response.page_size,
                max_page_size: pagination_response.max_page_size,
                page_number: pagination_response.page_number,
                max_page_number: pagination_response.max_page_number,
            },
        };
    }
}
