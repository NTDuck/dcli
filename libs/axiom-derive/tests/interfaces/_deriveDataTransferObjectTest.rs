use std::fmt::Debug;

use axiom_derive::DataTransferObject;
use serde::Deserialize;
use serde::Serialize;

// Allow `DataTransferObject` usage
// without adding `axiom` as a dependency
pub mod axiom {
    pub mod interfaces {
        use std::fmt::Debug;

        use serde::Serialize;
        use serde::Deserialize;

        #[allow(dead_code)]
        pub trait DataTransferObject: Debug + Send + Sync + Clone + Serialize + for<'de> Deserialize<'de> {}
    }
}

#[derive(DataTransferObject, Serialize, Deserialize)]
struct UnitStruct;

#[test]
fn testUnitStruct() {
    let instance = UnitStruct;

    let _: &dyn Debug = &instance;                      // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let _ = instance.clone();                           // Clone
    
    let serialized = serde_json::to_string(&instance)
        .unwrap();                                      // Serialize

    let _ = serde_json::from_str::<UnitStruct>(&serialized)
        .unwrap();                                      // Deserializer
}
