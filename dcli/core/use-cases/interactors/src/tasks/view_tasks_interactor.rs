use boundaries::tasks::ViewTasksBoundary;
use boundaries::tasks::ViewTasksInputBoundary;
use boundaries::tasks::ViewTasksOkResponseModel;
use boundaries::tasks::ViewTasksOutputBoundary;
use boundaries::tasks::ViewTasksRequestModel;
use boundaries::tasks::ViewTasksResponseModel;
use domain::tasks::Task;
use gateways::formatters::time::TimestampFormatter;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;
use gateways::repositories::tasks::TaskRepository;
use models::pagination::PaginationResponse;

use crate::utils::assemblers::tasks::TaskModelAssembler;

pub struct ViewTasksInteractor<Handle: PointerHandle> {
    output_boundary: SharedPointer<Box<dyn ViewTasksOutputBoundary>, Handle>,

    task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,

    response_model_assembler: ViewTasksResponseModelAssembler<Handle>,
}

impl<Handle: PointerHandle> ViewTasksInteractor<Handle> {
    pub fn new(
        output_boundary: SharedPointer<Box<dyn ViewTasksOutputBoundary>, Handle>,
        timestamp_formatter: SharedPointer<Box<dyn TimestampFormatter>, Handle>,
        task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,
    ) -> Self {
        return Self {
            output_boundary: output_boundary.clone(),
            task_repository: task_repository.clone(),
            response_model_assembler: ViewTasksResponseModelAssembler::new(timestamp_formatter.clone()),
        };
    }
}

impl<Handle: PointerHandle> ViewTasksInputBoundary for ViewTasksInteractor<Handle> {
    fn accept(&self, request: ViewTasksRequestModel) {
        let response = self.apply(request);
        self.output_boundary.as_ref().accept(response);
    }
}

impl<Handle: PointerHandle> ViewTasksBoundary for ViewTasksInteractor<Handle> {   
    fn apply(&self, request: ViewTasksRequestModel) -> ViewTasksResponseModel {
        let pagination_response = self.task_repository.as_ref().show_reverse_chronologically_ordered(request.pagination_request);
        let response_model = self.response_model_assembler.assemble(pagination_response);

        return response_model;
    }
}

pub struct ViewTasksResponseModelAssembler<Handle: PointerHandle> {
    task_model_assembler: TaskModelAssembler<Handle>,
}

impl<Handle: PointerHandle> ViewTasksResponseModelAssembler<Handle> {
    pub fn new(
        timestamp_formatter: SharedPointer<Box<dyn TimestampFormatter>, Handle>,
    ) -> Self {
        return Self {
            task_model_assembler: TaskModelAssembler::new(timestamp_formatter.clone()),
        };
    }

    pub fn assemble(&self, pagination_response: PaginationResponse<Task>) -> ViewTasksResponseModel {
        return Ok(ViewTasksOkResponseModel {
            pagination_response: PaginationResponse {
                items: pagination_response.items
                    .into_iter()
                    .map(|task| self.task_model_assembler.assemble(task))
                    .collect(),
                page_size: pagination_response.page_size,
                max_page_size: pagination_response.max_page_size,
                page_number: pagination_response.page_number,
                max_page_number: pagination_response.max_page_number,
            },
        });
    }
}
