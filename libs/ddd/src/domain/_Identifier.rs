use crate::utils::types::Concurrent;
use crate::domain::ValueObject;

pub trait Identifier: ValueObject + Concurrent {}
