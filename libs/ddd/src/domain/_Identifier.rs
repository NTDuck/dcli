use crate::utils::Concurrent;
use crate::domain::ValueObject;

pub trait Identifier: ValueObject + Concurrent {}
