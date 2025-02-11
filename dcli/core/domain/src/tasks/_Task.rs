use crate::tasks::TaskDescription;
use crate::tasks::TaskId;
use crate::tasks::TaskStatus;
use crate::utils::Timestamp;

#[derive(ddd::Entity)]
pub struct Task {
    #[ddd(attributes(Identifier))]
    pub id: TaskId,
    pub description: TaskDescription,
    pub status: TaskStatus,

    pub created_at: Timestamp,
}
