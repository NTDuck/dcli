use std::fmt::Debug;

pub trait ValueObject: Debug + Clone + PartialEq + Eq {}
