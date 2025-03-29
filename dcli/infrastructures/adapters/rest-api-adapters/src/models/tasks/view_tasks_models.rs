use axiom::interfaces::DataTransferObjectWithoutSerde;
use boundaries::tasks::ViewTasksErrResponseModel;
use boundaries::tasks::ViewTasksOkResponseModel;
use boundaries::tasks::ViewTasksRequestModel;
use boundaries::tasks::ViewTasksResponseModel;
use models::pagination::PaginationRequest;
use models::tasks::TaskModel;
use serde::Deserialize;
use serde::Serialize;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksRequestObject {
    pub page_number: usize,
    pub max_page_size: usize,
}

impl Into<ViewTasksRequestModel> for ViewTasksRequestObject {
    fn into(self) -> ViewTasksRequestModel {
        return ViewTasksRequestModel {
            pagination_request: PaginationRequest {
                page_number: self.page_number,
                max_page_size: self.max_page_size,
            },
        };
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ViewTasksViewModel {
    Ok(ViewTasksOkViewModel),
    Err(ViewTasksErrViewModel),
}

impl From<ViewTasksResponseModel> for ViewTasksViewModel {
    fn from(response: ViewTasksResponseModel) -> Self {
        let response = response
            .map(ViewTasksOkViewModel::from)
            .map_err(ViewTasksErrViewModel::from);

        match response {
            Ok(response) => Self::Ok(response),
            Err(response) => Self::Err(response),
        }
    }
}

impl Into<Result<ViewTasksOkViewModel, ViewTasksErrViewModel>> for ViewTasksViewModel {
    fn into(self) -> Result<ViewTasksOkViewModel, ViewTasksErrViewModel> {
        match self {
            Self::Ok(response) => Ok(response),
            Self::Err(response) => Err(response),
        }
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksOkViewModel {
    pub tasks: Vec<TaskModel>,
    
    pub page_size: usize,
    pub max_page_size: usize,
    pub page_number: usize,
    pub max_page_number: usize,
}

impl From<ViewTasksOkResponseModel> for ViewTasksOkViewModel {
    fn from(response: ViewTasksOkResponseModel) -> Self {
        let pagination_response = response.pagination_response;

        return Self {
            tasks: pagination_response.items,

            page_size: pagination_response.page_size,
            max_page_size: pagination_response.max_page_size,
            page_number: pagination_response.page_number,
            max_page_number: pagination_response.max_page_number,
        };
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksErrViewModel;

impl From<ViewTasksErrResponseModel> for ViewTasksErrViewModel {
    fn from(_: ViewTasksErrResponseModel) -> Self {
        return Self;
    }
}
