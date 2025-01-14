use std::{collections::HashMap, hash::{BuildHasher, RandomState}};

pub struct InMemoryReadRepository<Entity, Hasher = RandomState>
where
    Entity: ddd::Entity,
    Hasher: BuildHasher,
{
    entities_by_ids: std::sync::RwLock<HashMap<Entity::Identifier, Entity, Hasher>>,
}

impl<Entity> InMemoryReadRepository<Entity, RandomState>
where
    Entity: ddd::Entity,
{
    pub fn new() -> Self {
        return Self::with_hasher(RandomState::new());
    }
}

impl<Entity, Hasher> InMemoryReadRepository<Entity, Hasher>
where
    Entity: ddd::Entity,
    Hasher: BuildHasher,
{
    pub fn with_hasher(hasher: Hasher) -> Self {
        return Self {
            entities_by_ids: std::sync::RwLock::new(HashMap::with_hasher(hasher)),
        }
    }

    fn unwrap_read_entities_by_ids(&self) -> std::sync::RwLockReadGuard<HashMap<Entity::Identifier, Entity, Hasher>> {
        return self.entities_by_ids.read().unwrap();
    }

    fn unwrap_write_entities_by_ids(&self) -> std::sync::RwLockWriteGuard<HashMap<Entity::Identifier, Entity, Hasher>> {
        return self.entities_by_ids.write().unwrap();
    }
}

unsafe impl<Entity, Hasher> Sync for InMemoryReadRepository<Entity, Hasher>
where
    Entity: ddd::Entity,
    Hasher: BuildHasher,
{}

unsafe impl<Entity, Hasher> Send for InMemoryReadRepository<Entity, Hasher>
where
    Entity: ddd::Entity,
    Hasher: BuildHasher,
{}

impl<Entity, Hasher> ddd::ReadRepository<Entity> for InMemoryReadRepository<Entity, Hasher>
where
    Entity: ddd::Entity + ddd::types::Cloneable,
    Entity::Identifier: ddd::Identifier + std::hash::Hash,
    Hasher: BuildHasher,
{
    fn get_by_id(&self, identifier: Entity::Identifier) -> Option<Entity> {
        let entities_by_ids = self.unwrap_read_entities_by_ids();
        return entities_by_ids.get(&identifier)
            .cloned();
    }

    fn show(&self, offset: usize, limit: usize) -> Vec<Entity> {
        let entities_by_ids = self.unwrap_read_entities_by_ids();
        return entities_by_ids.values()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect();
    }

    fn size(&self) -> usize {
        let entities_by_ids = self.unwrap_read_entities_by_ids();
        return entities_by_ids.len();
    }

    fn contains(&self, identifier: Entity::Identifier) -> bool {
        let entities_by_ids = self.unwrap_read_entities_by_ids();
        return entities_by_ids.contains_key(&identifier);
    }
}

impl<Entity, Hasher> ddd::Repository<Entity> for InMemoryReadRepository<Entity, Hasher>
where
    Entity: ddd::Entity + ddd::types::Cloneable,
    Entity::Identifier: ddd::Identifier + std::hash::Hash,
    Hasher: BuildHasher,
{
    fn save(&self, entity: Entity) {
        let mut entities_by_ids = self.unwrap_write_entities_by_ids();
        let identifier = entity.get_id().clone();
        entities_by_ids.insert(identifier, entity);
    }

    fn delete(&self, identifier: Entity::Identifier) {
        let mut entities_by_ids = self.unwrap_write_entities_by_ids();
        entities_by_ids.remove(&identifier);
    }
}
