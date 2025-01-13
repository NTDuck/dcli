use ddd::Entity;

use super::utils::UUID;

#[derive(Clone, PartialEq, Eq)]
pub struct TaskIdentifier(pub UUID);

impl ddd::ValueObject for TaskIdentifier {}
impl ddd::Identifier for TaskIdentifier {}

pub struct Task {
    pub identifier: TaskIdentifier,
    pub description: String,
    pub is_active: bool,
}

impl ddd::Entity for Task {
    type Identifier = TaskIdentifier;

    fn get_id(&self) -> &Self::Identifier {
        return &self.identifier;
    }
}

impl PartialEq for Task {   
    fn eq(&self, other: &Self) -> bool {
        let self_id = self.get_id();
        let other_id = other.get_id();

        return self_id == other_id;
    }
}

impl Eq for Task {}
