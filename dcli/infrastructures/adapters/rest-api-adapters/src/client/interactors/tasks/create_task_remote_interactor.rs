use boundaries::tasks::CreateTaskBoundary;
use boundaries::tasks::CreateTaskRequestModel;
use boundaries::tasks::CreateTaskResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;
use ureq::Agent;

use crate::utils::dataclasses::Endpoints;

pub struct CreateTaskRemoteInteractor<Handle: PointerHandle> {
    agent: SharedPointer<Agent, Handle>,
    endpoints: SharedPointer<Endpoints, Handle>,
}

impl<Handle: PointerHandle> CreateTaskBoundary for CreateTaskRemoteInteractor<Handle> {
    fn apply(&self, request: CreateTaskRequestModel) -> CreateTaskResponseModel {
        let uri = format!("{}://{}:{}/{}/{}",
            self.endpoints.as_ref().scheme, self.endpoints.as_ref().domain, self.endpoints.as_ref().port,
            self.endpoints.as_ref().task_router_path, self.endpoints.as_ref().create_task_handler_path,
        );

        let response = self.agent.as_ref()
            .post(uri)
            .send_json(request).unwrap()
            .into_body()
            .read_json().unwrap();

        return response;
    }
}
