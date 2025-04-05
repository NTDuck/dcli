use domain::tasks::TaskStatus;

use crate::params::Param;

impl From<Param<&'static str>> for TaskStatus {
    fn from(Param(status): Param<&'static str>) -> Self {
        match status {
            "pending" => Self::Pending,
            "in-progress" => Self::InProgress,
            "completed" => Self::Completed,
            _ => unreachable!(),
        }
    }
}