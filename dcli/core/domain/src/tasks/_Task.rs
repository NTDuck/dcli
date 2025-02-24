use axiom::interfaces::ddd;

use crate::tasks::TaskDescription;
use crate::tasks::TaskStatus;

#[derive(ddd::Entity)]
pub struct Task {
    #[axiom(attributes(ddd::Identifier))]
    pub id: TaskId,
    pub description: TaskDescription,
    pub status: TaskStatus,
}

pub use crate::ids::Snowflake as TaskId;
