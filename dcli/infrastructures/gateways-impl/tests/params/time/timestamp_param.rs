use domain::time::Timestamp;

use crate::params::Param;

impl From<Param<usize>> for Timestamp {
    fn from(Param(millis): Param<usize>) -> Self {
        Self::from_millis_since_epoch(millis as i64)
    }
}
