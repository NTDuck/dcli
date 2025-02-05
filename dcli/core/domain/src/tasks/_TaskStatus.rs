#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}
