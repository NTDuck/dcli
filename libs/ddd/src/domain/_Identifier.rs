use super::ValueObject;

pub trait Identifier: ValueObject + Send + Sync {}
