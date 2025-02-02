use crate::domain::Identifier;

pub trait Entity {
    type Id: Identifier;
}
