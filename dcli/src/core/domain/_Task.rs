use ddd::{self, Entity};

// #[derive(ddd::Identifier)]
// pub struct TaskIdentifier(pub u128);

#[derive(Clone, PartialEq, Eq, ddd_derive::Identifier)]
pub struct TaskIdentifier(pub u128);

// impl ddd::Identifier for TaskIdentifier {}

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
