use crate::gateways::repositories::tasks::TaskRepository;
use crate::utils::contracts::interactors::MutableRunnableInteractor;

pub struct ClearTasksInteractor<'deps> {
    task_repository: &'deps mut dyn TaskRepository,
}

impl<'deps> MutableRunnableInteractor for ClearTasksInteractor<'deps> {
    fn run(&mut self) {
        self.task_repository.clear();
    }
}
