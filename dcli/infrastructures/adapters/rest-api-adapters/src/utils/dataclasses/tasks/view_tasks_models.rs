use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;
use use_cases::boundaries::tasks::ViewTasksErrResponseModel;
use use_cases::boundaries::tasks::ViewTasksOkResponseModel;
use use_cases::boundaries::tasks::ViewTasksRequestModel;
use use_cases::boundaries::tasks::ViewTasksResponseModel;
#[cfg(feature = "server")]
use use_cases::dataclasses::pagination::PaginationRequest;
#[cfg(feature = "client")]
use use_cases::dataclasses::pagination::PaginationResponse;
use use_cases::dataclasses::tasks::TaskModel;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksRequestObject {
    pub page_number: usize,
    pub max_page_size: usize,
}

#[cfg(feature = "client")]
impl From<ViewTasksRequestModel> for ViewTasksRequestObject {
    fn from(request: ViewTasksRequestModel) -> Self {
        let pagination_request = request.pagination_request;

        return Self {
            page_number: pagination_request.page_number,
            max_page_size: pagination_request.max_page_size,
        };
    }
}

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
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

#[cfg(feature = "client")]
impl Into<ViewTasksResponseModel> for ViewTasksViewModel {
    fn into(self) -> ViewTasksResponseModel {
        match self {
            Self::Ok(response) => Ok(response.into()),
            Self::Err(response) => Err(response.into()),
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

#[cfg(feature = "server")]
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

#[cfg(feature = "client")]
impl Into<ViewTasksOkResponseModel> for ViewTasksOkViewModel {
    fn into(self) -> ViewTasksOkResponseModel {
        return ViewTasksOkResponseModel {
            pagination_response: PaginationResponse {
                items: self.tasks,

                page_size: self.page_size,
                max_page_size: self.max_page_size,
                page_number: self.page_number,
                max_page_number: self.max_page_number,
            },
        };
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksErrViewModel;

#[cfg(feature = "server")]
impl From<ViewTasksErrResponseModel> for ViewTasksErrViewModel {
    fn from(_: ViewTasksErrResponseModel) -> Self {
        return Self;
    }
}

#[cfg(feature = "client")]
impl Into<ViewTasksErrResponseModel> for ViewTasksErrViewModel {
    fn into(self) -> ViewTasksErrResponseModel {
        return ViewTasksErrResponseModel;
    }
}
