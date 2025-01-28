use crate::tasks::*;

pub struct Task {
    pub id: TaskId,
    pub description: TaskDescription,
    pub status: TaskStatus,
}
