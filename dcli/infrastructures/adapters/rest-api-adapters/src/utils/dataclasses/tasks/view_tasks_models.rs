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
    fn from(model: ViewTasksRequestModel) -> Self {
        let pagination_request = model.pagination_request;

        Self {
            page_number: pagination_request.page_number,
            max_page_size: pagination_request.max_page_size,
        }
    }
}

#[cfg(feature = "server")]
impl From<ViewTasksRequestObject> for ViewTasksRequestModel {
    fn from(model: ViewTasksRequestObject) -> Self {
        ViewTasksRequestModel {
            pagination_request: PaginationRequest {
                page_number: model.page_number,
                max_page_size: model.max_page_size,
            },
        }
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
    fn from(model: ViewTasksResponseModel) -> Self {
        let model = model
            .map(ViewTasksOkViewModel::from)
            .map_err(ViewTasksErrViewModel::from);

        match model {
            Ok(model) => Self::Ok(model),
            Err(model) => Self::Err(model),
        }
    }
}

#[cfg(feature = "client")]
impl From<ViewTasksViewModel> for ViewTasksResponseModel {
    fn from(model: ViewTasksViewModel) -> Self {
        match model {
            ViewTasksViewModel::Ok(model) => Ok(model.into()),
            ViewTasksViewModel::Err(model) => Err(model.into()),
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
    fn from(model: ViewTasksOkResponseModel) -> Self {
        let pagination_response = model.pagination_response;

        Self {
            tasks: pagination_response.items,

            page_size: pagination_response.page_size,
            max_page_size: pagination_response.max_page_size,
            page_number: pagination_response.page_number,
            max_page_number: pagination_response.max_page_number,
        }
    }
}

#[cfg(feature = "client")]
impl From<ViewTasksOkViewModel> for ViewTasksOkResponseModel {
    fn from(model: ViewTasksOkViewModel) -> Self {
        ViewTasksOkResponseModel {
            pagination_response: PaginationResponse {
                items: model.tasks,

                page_size: model.page_size,
                max_page_size: model.max_page_size,
                page_number: model.page_number,
                max_page_number: model.max_page_number,
            },
        }
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct ViewTasksErrViewModel;

#[cfg(feature = "server")]
impl From<ViewTasksErrResponseModel> for ViewTasksErrViewModel {
    fn from(_: ViewTasksErrResponseModel) -> Self {
        Self
    }
}

#[cfg(feature = "client")]
impl From<ViewTasksErrViewModel> for ViewTasksErrResponseModel {
    fn from(_: ViewTasksErrViewModel) -> Self {
        ViewTasksErrResponseModel
    }
}
