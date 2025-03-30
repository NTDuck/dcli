use axum::http::Uri;
use boundaries::tasks::CreateTaskBoundary;
use boundaries::tasks::CreateTaskRequestModel;
use boundaries::tasks::CreateTaskResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;
use ureq::Agent;

use crate::utils::models::tasks::CreateTaskRequestObject;

pub struct CreateTaskRemoteInteractor<Handle: PointerHandle> {
    agent: SharedPointer<Agent, Handle>,
    uri: Uri,
}

impl<Handle: PointerHandle> CreateTaskBoundary for CreateTaskRemoteInteractor<Handle> {
    fn apply(&self, request: CreateTaskRequestModel) -> CreateTaskResponseModel {
        let request: CreateTaskRequestObject = request.into();

        let response = self.agent.as_ref()
            .post(self.uri.clone())
            .send_json(request).unwrap()
            .into_body()
            .read_json().unwrap();

        let response = 

        return response;
    }
}
