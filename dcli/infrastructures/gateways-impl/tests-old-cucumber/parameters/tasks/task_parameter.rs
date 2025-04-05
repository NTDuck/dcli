use std::str::FromStr;

use domain::tasks::Task;
use domain::tasks::TaskStatus;

use crate::parameters::ids::SnowflakeParameter;

#[derive(cucumber::Parameter)]
#[param(name = "task", regex = r".+")]
pub struct TaskParameter(Task);

impl FromStr for TaskParameter {
    type Err = <SnowflakeParameter as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let snowflake = SnowflakeParameter::from_str(s)?.into();
        let task = Task {
            id: snowflake,
            description: "description".try_into().unwrap(),
            status: TaskStatus::Pending,
        };
        Ok(Self(task))
    }
}

impl From<TaskParameter> for Task {
    fn from(parameter: TaskParameter) -> Self {
        parameter.0
    }
}
