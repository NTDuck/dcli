use domain::time::Timestamp;

use crate::utils::interfaces::Gateway;

pub trait TimestampProvider: Gateway {
    fn get_current_timestamp(&self) -> Timestamp;
}
