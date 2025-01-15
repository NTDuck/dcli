use crate::core::domain::{Task, TaskIdentifier};

use super::abc;

pub type InMemoryTaskRepository = abc::InMemoryRepository<Task>;

impl Clone for Task {
    fn clone(&self) -> Self {
        return Self {
            identifier: self.identifier.clone(),
            description: self.description.clone(),
            is_active: self.is_active.clone(),
        };
    }
}

impl std::hash::Hash for TaskIdentifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        return self.0.0.hash(state);
    }
}

impl InMemoryTaskRepository {
    pub fn behave_in_a_specialized_way(&self) {
        println!("Hey, we can also do this? All hail Rust!");
    }
}