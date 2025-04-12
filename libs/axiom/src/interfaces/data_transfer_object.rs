use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait DataTransferObject: Debug + Send + Sync + Clone + Serialize + DeserializeOwned {}

pub use derive::DataTransferObjectWithoutDeserialize;
pub use derive::DataTransferObjectWithoutSerde;
