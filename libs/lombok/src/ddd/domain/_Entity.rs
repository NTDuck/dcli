use crate::ddd::domain::Identifier;
use crate::ddd::domain::ValueObject;

pub trait Entity: ValueObject {
    type Id: Identifier;

    fn getId(&self) -> &Self::Id;
}
