use crate::interfaces::ddd::domain::Identifier;
use crate::interfaces::ddd::domain::ValueObject;

pub trait Entity: ValueObject {
    type Id: Identifier;

    fn getId(&self) -> &Self::Id;
}

pub use derive::Entity;
