use crate::interfaces::ddd::domain::Identifier;
use crate::interfaces::ddd::domain::ValueObject;

pub trait Entity: ValueObject {
    type Id: Identifier;

    fn get_id(&self) -> &Self::Id;
}

pub use derive::Entity;
