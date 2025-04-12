use domain::tasks::Task;

use crate::params::Param;

impl From<Param<usize>> for Task {
    fn from(Param(id): Param<usize>) -> Self {
        Self::from(Param((id, DEFAULT_TASK_DESCRIPTION, DEFAULT_TASK_STATUS)))
    }
}

impl From<Param<(usize, &'static str, &'static str)>> for Task {
    fn from(Param((id, description, status)): Param<(usize, &'static str, &'static str)>) -> Self {
        let id = Param(id).into();
        let description = Param(description).into();
        let status = Param(status).into();

        Self {
            id,
            description,
            status,
        }
    }
}

const DEFAULT_TASK_DESCRIPTION: &str = "description";
const DEFAULT_TASK_STATUS: &str = "pending";
