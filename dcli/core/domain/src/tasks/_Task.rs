use axiom::interfaces::ddd;

use crate::tasks::TaskDescription;
use crate::tasks::TaskStatus;
use crate::utils::dataclasses::ids::Uuid;
use crate::utils::dataclasses::time::Timestamp;

#[derive(ddd::Entity)]
pub struct Task {
    #[axiom(attributes(ddd::Identifier))]
    pub id: Uuid,
    pub description: TaskDescription,
    pub status: TaskStatus,

    pub createdAt: Timestamp,
}
