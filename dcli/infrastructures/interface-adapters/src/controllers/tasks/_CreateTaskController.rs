use use_cases::boundaries::tasks::CreateTaskBoundary;
use use_cases::boundaries::tasks::CreateTaskErrorModel;
use use_cases::boundaries::tasks::CreateTaskRequestModel;
use use_cases::boundaries::tasks::CreateTaskResponseModel;

pub struct CreateTaskController<'bdrs> {
    interactor: &'bdrs mut dyn CreateTaskBoundary,
}

impl<'bdr> CreateTaskController<'bdr> {
    pub fn apply(&mut self, request: CreateTaskRequestObject) -> Result<CreateTaskViewModel, CreateTaskErrorViewModel> {
        let request = request.into();
        return match self.interactor.apply(request) {
            Ok(response) => Ok(CreateTaskViewModel::from(response)),
            Err(error) => Err(CreateTaskErrorViewModel::from(error)),
        };
    }
}

pub type CreateTaskRequestObject = CreateTaskRequestModel;
pub type CreateTaskViewModel = CreateTaskResponseModel;
pub type CreateTaskErrorViewModel = CreateTaskErrorModel;
