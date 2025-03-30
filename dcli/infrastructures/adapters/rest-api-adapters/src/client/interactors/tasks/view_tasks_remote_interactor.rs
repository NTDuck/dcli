use axum::http::Uri;
use boundaries::tasks::ViewTasksBoundary;
use boundaries::tasks::ViewTasksRequestModel;
use boundaries::tasks::ViewTasksResponseModel;
use gateways::pointers::PointerHandle;
use gateways::pointers::SharedPointer;
use ureq::Agent;

pub struct ViewTasksRemoteInteractor<Handle: PointerHandle> {
    agent: SharedPointer<Agent, Handle>,
    uri: Uri,
}

impl<Handle: PointerHandle> ViewTasksBoundary for ViewTasksRemoteInteractor<Handle> {
    fn apply(&self, request: ViewTasksRequestModel) -> ViewTasksResponseModel {
        let query = serde_qs::to_string(&request).unwrap();
        let uri = format!("{}?{}", self.uri.clone(), query);

        let response = self.agent.as_ref()
            .get(uri)
            .call().unwrap()
            .into_body()
            .read_json().unwrap();

        return response;
    }
}
