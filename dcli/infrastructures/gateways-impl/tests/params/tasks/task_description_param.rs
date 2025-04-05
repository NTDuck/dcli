use domain::tasks::TaskDescription;

use crate::params::Param;

impl From<Param<&'static str>> for TaskDescription {
    fn from(Param(description): Param<&'static str>) -> Self {
        description.try_into().unwrap()
    }
}
