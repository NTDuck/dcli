use crate::domain;
use crate::utils::types::Concurrent;

pub trait ReadRepository<Entity: domain::Entity>: Concurrent {
    fn get_by_id(&self, identifier: Entity::Identifier) -> Option<Entity>;

    fn show(&self, offset: usize, limit: usize) -> Vec<Entity>;

    fn size(&self) -> usize;

    fn contains(&self, identifier: Entity::Identifier) -> bool {
        let entity = self.get_by_id(identifier);
        return entity.is_some();
    }
}

pub trait Repository<Entity: domain::Entity>: ReadRepository<Entity> {
    fn save(&self, entity: Entity);
    fn delete(&self, identifier: Entity::Identifier);
}
