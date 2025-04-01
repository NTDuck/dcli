use axiom::behaviours::New;
use ureq::Agent;
use use_cases::boundaries::tasks::ViewTasksBoundary;
use use_cases::boundaries::tasks::ViewTasksRequestModel;
use use_cases::boundaries::tasks::ViewTasksResponseModel;
use use_cases::gateways::pointers::PointerHandle;
use use_cases::gateways::pointers::SharedPointer;

use crate::utils::dataclasses::tasks::ViewTasksRequestObject;
use crate::utils::dataclasses::tasks::ViewTasksViewModel;

#[derive(New)]
pub struct ViewTasksRemoteInteractor<Handle: PointerHandle> {
    agent: SharedPointer<Agent, Handle>,
    uri: &'static str,
}

impl<Handle: PointerHandle> ViewTasksBoundary for ViewTasksRemoteInteractor<Handle> {
    fn apply(&self, request: ViewTasksRequestModel) -> ViewTasksResponseModel {
        let request: ViewTasksRequestObject = request.into();

        let query = serde_qs::to_string(&request).unwrap();
        let uri = format!("{}?{}", self.uri, query);

        let response = self.agent.as_ref()
            .get(uri)
            .call().unwrap()
            .into_body()
            .read_json::<ViewTasksViewModel>().unwrap();

        let response = response.into();

        return response;
    }
}
