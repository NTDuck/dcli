use crate::utils::types::{Cloneable, EqualityComparable};

pub trait ValueObject: Cloneable + EqualityComparable {}
