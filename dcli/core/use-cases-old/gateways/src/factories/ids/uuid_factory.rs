use domain::ids::Uuid;

use crate::utils::interfaces::Gateway;

pub trait UuidFactory: Gateway {
    fn create(&self) -> Uuid;
}
