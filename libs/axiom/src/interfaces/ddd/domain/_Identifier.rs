use std::hash::Hash;

use crate::interfaces::ddd::domain::ValueObject;

pub trait Identifier: ValueObject + Hash {}
