use domain::TaskStatus;
use fake::Dummy;

#[derive(Dummy)]
pub enum MockTaskStatus {
    Pending,
    InProgress,
    Completed,
}

impl Into<TaskStatus> for MockTaskStatus {
    fn into(self) -> TaskStatus {
        return match self {
            MockTaskStatus::Pending => TaskStatus::Pending,
            MockTaskStatus::InProgress => TaskStatus::InProgress,
            MockTaskStatus::Completed => TaskStatus::Completed,
        };
    }
}