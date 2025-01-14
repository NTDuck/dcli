use crate::domain;
use crate::utils::types::EqualityComparable;

pub trait Entity: EqualityComparable {
    type Identifier: domain::Identifier;

    fn get_id(&self) -> &Self::Identifier;
}
