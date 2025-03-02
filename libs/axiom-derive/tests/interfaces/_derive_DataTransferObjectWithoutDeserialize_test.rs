use axiom::interfaces::DataTransferObject;
use axiom_derive::DataTransferObjectWithoutDeserialize;
use serde::Deserialize;

use crate::utils::*;

testCommonStructsAndEnums!{
    (DataTransferObjectWithoutDeserialize, Deserialize),
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
