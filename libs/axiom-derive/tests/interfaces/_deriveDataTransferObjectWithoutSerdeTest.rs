use axiom::interfaces::DataTransferObject;
use axiom_derive::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::*;

testCommonStructsAndEnums!{
    (DataTransferObjectWithoutSerde, Serialize, Deserialize),
    verifyTraitBounds
}

// Allow `DataTransferObject` usage
// without adding `axiom` as a dependency
pub mod axiom {
    pub mod interfaces {
        use std::fmt::Debug;

        use serde::Serialize;
        use serde::Deserialize;

        pub trait DataTransferObject: Debug + Send + Sync + Clone + Serialize + for<'de> Deserialize<'de> {}
    }
}

fn verifyTraitBounds(_: impl DataTransferObject) {}
