use crate::domain::Identifier;

pub trait Entity: Eq + PartialEq {
    type Identifier: Identifier;

    fn get_id(&self) -> &Self::Identifier;
}
