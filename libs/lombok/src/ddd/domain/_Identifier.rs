use std::hash::Hash;

use crate::ddd::domain::ValueObject;

pub trait Identifier: ValueObject + Copy + Hash {}
