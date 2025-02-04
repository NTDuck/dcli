use std::time::Instant;

use crate::tasks::*;

#[derive(ddd::Entity)]
pub struct Task {
    #[ddd(field(Identifier))]
    pub id: TaskId,
    pub description: TaskDescription,
    pub status: TaskStatus,

    pub created_at: Instant,
}
