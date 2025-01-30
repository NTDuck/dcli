#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}
