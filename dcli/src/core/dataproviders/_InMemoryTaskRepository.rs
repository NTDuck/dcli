use crate::core::domain::{Task, TaskIdentifier};

use super::abc;

pub struct InMemoryTaskRepository {
    base: abc::InMemoryRepository<Task>,
}

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
    pub fn new() -> Self {
        return Self {
            base: abc::InMemoryRepository::<Task>::new(),
        };
    }
}

impl ddd::ReadRepository<Task> for InMemoryTaskRepository {
    fn get_by_id(&self, identifier: TaskIdentifier) -> Option<Task> {
        return self.base.get_by_id(identifier);
    }

    fn show(&self, offset: usize, limit: usize) -> Vec<Task> {
        return self.base.show(offset, limit);
    }

    fn size(&self) -> usize {
        return self.base.size();
    }

    fn contains(&self, identifier: TaskIdentifier) -> bool {
        return self.base.contains(identifier);
    }
}

impl ddd::Repository<Task> for InMemoryTaskRepository {
    fn save(&self, entity: Task) {
        return self.base.save(entity);   
    }

    fn delete(&self, identifier: TaskIdentifier) {
        return self.base.delete(identifier);
    }
}