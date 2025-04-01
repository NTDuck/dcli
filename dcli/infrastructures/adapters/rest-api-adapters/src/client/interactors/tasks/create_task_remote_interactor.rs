use axiom::behaviours::New;
use ureq::Agent;
use use_cases::boundaries::tasks::CreateTaskBoundary;
use use_cases::boundaries::tasks::CreateTaskRequestModel;
use use_cases::boundaries::tasks::CreateTaskResponseModel;
use use_cases::gateways::pointers::PointerHandle;
use use_cases::gateways::pointers::SharedPointer;

use crate::utils::dataclasses::tasks::CreateTaskRequestObject;
use crate::utils::dataclasses::tasks::CreateTaskViewModel;

#[derive(New)]
pub struct CreateTaskRemoteInteractor<Handle: PointerHandle> {
    agent: SharedPointer<Agent, Handle>,
    uri: &'static str,
}

impl<Handle: PointerHandle> CreateTaskBoundary for CreateTaskRemoteInteractor<Handle> {
    fn apply(&self, request: CreateTaskRequestModel) -> CreateTaskResponseModel {
        let request: CreateTaskRequestObject = request.into();

        let response = self.agent.as_ref()
            .post(self.uri)
            .send_json(request).unwrap()
            .into_body()
            .read_json::<CreateTaskViewModel>().unwrap();

        let response = response.into();

        return response;
    }
}
