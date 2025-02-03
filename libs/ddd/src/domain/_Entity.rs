use crate::domain::Identifier;
use crate::domain::ValueObject;

pub trait Entity: ValueObject {
    type Id: Identifier;

    fn get_id(&self) -> &Self::Id;
}
