use std::hash::Hash;

use crate::domain::ValueObject;

pub trait Identifier: ValueObject + Copy + Hash {}
