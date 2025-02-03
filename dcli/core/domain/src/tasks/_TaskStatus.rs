#[derive(Clone, PartialEq, Eq, Copy)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}
