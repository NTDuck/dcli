use crate::tasks::*;

#[derive(Clone)]
pub struct Task {
    pub id: TaskId,
    pub description: TaskDescription,
    pub status: TaskStatus,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Eq for Task {}
