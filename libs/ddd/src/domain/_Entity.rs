use crate::{domain::Identifier, utils::types::EqualityComparable};

pub trait Entity: EqualityComparable {
    type Identifier: Identifier;

    fn get_id(&self) -> &Self::Identifier;
}
