use domain::TaskId;

#[derive(ddd::ValueObject)]
pub struct HashableTaskId(TaskId);

impl From<TaskId> for HashableTaskId {
    fn from(task_id: TaskId) -> Self {
        return Self(task_id);
    }
}

impl std::hash::Hash for HashableTaskId {
    fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
        self.0.hash(state);
    }
}
