use domain::tasks::Task;

use crate::boundaries::tasks::ViewTasksBoundary;
use crate::boundaries::tasks::ViewTasksOkResponseModel;
use crate::boundaries::tasks::ViewTasksRequestModel;
use crate::boundaries::tasks::ViewTasksResponseModel;
use crate::gateways::formatters::time::TimestampFormatter;
use crate::gateways::pointers::PointerHandle;
use crate::gateways::pointers::SharedPointer;
use crate::gateways::repositories::tasks::TaskRepository;
use crate::utils::assemblers::tasks::TaskModelAssembler;
use crate::utils::dataclasses::pagination::PaginationResponse;

pub struct ViewTasksInteractor<Handle: PointerHandle> {
    task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,

    response_model_assembler: ViewTasksResponseModelAssembler<Handle>,
}

impl<Handle: PointerHandle> ViewTasksInteractor<Handle> {
    pub fn new(
        timestamp_formatter: SharedPointer<Box<dyn TimestampFormatter>, Handle>,
        task_repository: SharedPointer<Box<dyn TaskRepository>, Handle>,
    ) -> Self {
        return Self {
            task_repository: task_repository.clone(),
            response_model_assembler: ViewTasksResponseModelAssembler::new(
                timestamp_formatter.clone(),
            ),
        };
    }
}

impl<Handle: PointerHandle> ViewTasksBoundary for ViewTasksInteractor<Handle> {
    fn apply(&self, request: ViewTasksRequestModel) -> ViewTasksResponseModel {
        let pagination_response = self
            .task_repository
            .as_ref()
            .show_reverse_chronologically_ordered(request.pagination_request);
        let response_model =
            self.response_model_assembler.assemble(pagination_response);

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
            task_model_assembler: TaskModelAssembler::new(
                timestamp_formatter.clone(),
            ),
        };
    }

    pub fn assemble(
        &self,
        pagination_response: PaginationResponse<Task>,
    ) -> ViewTasksResponseModel {
        return Ok(ViewTasksOkResponseModel {
            pagination_response: PaginationResponse {
                items: pagination_response
                    .items
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
