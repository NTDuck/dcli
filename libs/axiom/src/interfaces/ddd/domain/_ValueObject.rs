use std::fmt::Debug;

pub trait ValueObject: Debug + Send + Sync + Clone + PartialEq + Eq {}
