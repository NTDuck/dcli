use std::time::Instant;

use ddd::Entity;

use crate::tasks::*;

#[derive(ddd::Entity, Clone)]
pub struct Task {
    #[entity(id)]
    pub id: TaskId,
    pub description: TaskDescription,
    pub status: TaskStatus,

    pub created_at: Instant,
}
