use domain::Task;

pub trait TaskGateway {
    fn save(&self, task: Task);
    fn deactivate(&self, task_id: u128);
    fn remove(&self, task_id: u128);

    fn show(&self, offset: usize, limit: usize) -> Option<Vec<Task>>;
    fn show_active(&self, offset: usize, limit: usize) -> Option<Vec<Task>>;
    fn show_inactive(&self, offset: usize, limit: usize) -> Option<Vec<Task>>;
}
