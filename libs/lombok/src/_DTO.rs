use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub trait DataTransferObject: Debug + Send + Sync + Clone + Serialize + for<'de> Deserialize<'de> + 'static {}
