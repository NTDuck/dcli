use axiom::interfaces::ddd;

use crate::ids::Snowflake;
use crate::tasks::TaskDescription;
use crate::tasks::TaskStatus;

#[derive(ddd::Entity)]
pub struct Task {
    #[axiom(attributes(ddd::Identifier))]
    pub id: TaskId,
    pub description: TaskDescription,
    pub status: TaskStatus,
}

pub type TaskId = Snowflake;
