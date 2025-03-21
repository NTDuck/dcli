use std::ops::Deref;

use axiom::behaviours::New;
use boundaries::tasks::ViewTasksInputBoundary;
use boundaries::tasks::ViewTasksOutputBoundary;
use boundaries::tasks::ViewTasksResponseModel;
use boundaries::utils::dataclasses::pagination::PaginationResponse;
use domain::tasks::Task;
use gateways::formatters::time::TimestampFormatter;
use gateways::pointers::PointerHandle;
use gateways::repositories::tasks::TaskRepository;

use crate::utils::assemblers::TaskModelAssembler;
use crate::utils::pointers::SharedPointer;

#[derive(New)]
pub struct ViewTasksInteractor<Handle: PointerHandle> {
    output_boudnary: SharedPointer<Box<dyn ViewTasksOutputBoundary>, Handle>,

    timestamp_formatter: SharedPointer<Box<dyn TimestampFormatter>, Handle>,
    task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,
}

impl<Handle: PointerHandle> ViewTasksInputBoundary for ViewTasksInteractor<Handle> {
    fn apply(&self, request: ViewTasksRequestModel) -> Result<ViewTasksResponseModel, ViewTasksErrorModel> {
        let pagination_response = self.task_repository.as_ref().show_reverse_chronologically_ordered(request.pagination_request);
        let response_model = ViewTasksResponseModel::new_from(pagination_response)
            .using(self.timestamp_formatter.as_ref());

        return Ok(response_model);
    }
    
    fn accept(&self, request: boundaries::tasks::ViewTasksRequestModel) {
        todo!()
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

pub struct ViewTasksResponseModelAssembler<Handle: PointerHandle> {
    timestamp_formatter: SharedPointer<Box<dyn TimestampFormatter>, Handle>,
    task_model_assembler: TaskModelAssembler<Handle>,
}

impl<Handle: PointerHandle> ViewTasksResponseModelAssembler<Handle> {
    pub fn new()

    pub fn assemble(&self, pagination_response: PaginationResponse<Task>) -> ViewTasksResponseModel {
        return ViewTasksResponseModel {

        }
    }
}