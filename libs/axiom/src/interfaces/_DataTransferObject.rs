use std::fmt::Debug;

pub trait DataTransferObject: Debug + Send + Sync + Clone + Serialize + for<'de> Deserialize<'de> {}

pub use derive::DataTransferObject;
pub use serde::Deserialize;
pub use serde::Serialize;
