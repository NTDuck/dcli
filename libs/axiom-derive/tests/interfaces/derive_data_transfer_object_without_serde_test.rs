use axiom::interfaces::DataTransferObject;
use axiom_derive::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::templates::common_combinations::test_common_combinations;

test_common_combinations!(
    derives = #[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)],
    test_fn = verify_trait_bounds,
);

// Allow `DataTransferObject` usage
// without adding `axiom` as a dependency
pub mod axiom {
    pub mod interfaces {
        use std::fmt::Debug;

        use serde::Deserialize;
        use serde::Serialize;

        pub trait DataTransferObject: Debug + Send + Sync + Clone + Serialize + for<'de> Deserialize<'de> {}
    }
}

fn verify_trait_bounds(_: impl DataTransferObject) {}
