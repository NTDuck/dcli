use axiom_derive::ValueObject;

// Allow `ValueObject` usage
// without adding `axiom` as a dependency
pub mod axiom {
    pub mod interfaces {
        pub mod ddd {
            pub mod domain {
                use std::fmt::Debug;

                #[allow(dead_code)]
                pub trait ValueObject: Debug + Send + Sync + Clone + PartialEq + Eq {}
            }
        }
    }
}

#[derive(ValueObject)]
struct UnitStruct;

#[test]
fn testUnitStruct() {
    let instance = UnitStruct;

    let _: &dyn std::fmt::Debug = &instance;            // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq
}

#[derive(ValueObject)]
struct StructWithNoFields {}

#[test]
fn testStructWithNoFields() {
    let instance = StructWithNoFields {};
    
    let _: &dyn std::fmt::Debug = &instance;            // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq    
}

#[derive(ValueObject)]
struct StructWithNamedFields {
    text: String,
    number: u64,
    flag: bool,
}

#[test]
fn testStructWithNamedFields() {
    let instance = StructWithNamedFields {
        text: "tomfoolery".to_owned(),
        number: 42,
        flag: false,
    };
    
    let _: &dyn std::fmt::Debug = &instance;            // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq    
}
