use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

pub trait DataTransferObject: Debug + Send + Sync + Clone + Serialize + for<'de> Deserialize<'de> {}

pub use derive::SerdelessDataTransferObject;
