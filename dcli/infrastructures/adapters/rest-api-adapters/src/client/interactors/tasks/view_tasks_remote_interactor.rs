use boundaries::tasks::ViewTasksBoundary;
use boundaries::tasks::ViewTasksRequestModel;
use boundaries::tasks::ViewTasksResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;
use ureq::Agent;

use crate::utils::dataclasses::Endpoints;

pub struct ViewTasksRemoteInteractor<Handle: PointerHandle> {
    agent: SharedPointer<Agent, Handle>,
    endpoints: SharedPointer<Endpoints, Handle>,
}

impl<Handle: PointerHandle> ViewTasksBoundary for ViewTasksRemoteInteractor<Handle> {
    fn apply(&self, request: ViewTasksRequestModel) -> ViewTasksResponseModel {
        let queries = serde_qs::to_string(&request).unwrap();
        let uri = format!("{}://{}:{}/{}/{}?{}",
            self.endpoints.as_ref().scheme, self.endpoints.as_ref().domain, self.endpoints.as_ref().port,
            self.endpoints.as_ref().task_router_path, self.endpoints.as_ref().view_tasks_handler_path,
            queries,
        );

        let response = self.agent.as_ref()
            .get(uri)
            .call().unwrap()
            .into_body()
            .read_json().unwrap();

        return response;
    }
}
