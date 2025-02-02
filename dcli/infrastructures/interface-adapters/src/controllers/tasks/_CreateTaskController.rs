use use_cases::contracts::interactors::FallibleMutableConsumerInteractor;
use use_cases::interactors::tasks::CreateTaskInteractor;
use use_cases::interactors::tasks::CreateTaskRequestModel;

pub struct CreateTaskController<'int, 'deps> {
    interactor: &'int mut CreateTaskInteractor<'deps>,
}

impl<'int, 'deps> CreateTaskController<'int, 'deps> {
    pub fn apply(&mut self, request_object: CreateTaskRequestObject) {
        let request_model = request_object;
        self.interactor.consume(request_model);
    }
}

pub type CreateTaskRequestObject = CreateTaskRequestModel;
