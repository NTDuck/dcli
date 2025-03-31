use axum::http::Uri;
use ureq::Agent;
use use_cases::boundaries::tasks::ViewTasksBoundary;
use use_cases::boundaries::tasks::ViewTasksRequestModel;
use use_cases::boundaries::tasks::ViewTasksResponseModel;
use use_cases::gateways::pointers::PointerHandle;
use use_cases::gateways::pointers::SharedPointer;

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
