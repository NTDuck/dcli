use crate::dataproviders::gateways::tasks::TaskGateway;
use crate::interactors::utils::contracts::MutableRunnableInteractor;

pub struct ClearTasksInteractor<'deps> {
    task_gateway: &'deps mut dyn TaskGateway,
}

impl<'deps> MutableRunnableInteractor for ClearTasksInteractor<'deps> {
    fn run(&mut self) {
        self.task_gateway.clear();
    }
}
