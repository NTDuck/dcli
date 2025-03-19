use crate::interfaces::ddd::domain::Identifier;
use crate::interfaces::ddd::domain::ValueObject;

pub trait Entity: ValueObject {
    type Identifier: Identifier;

    fn get_id(&self) -> &Self::Identifier;
}

pub use derive::Entity;
