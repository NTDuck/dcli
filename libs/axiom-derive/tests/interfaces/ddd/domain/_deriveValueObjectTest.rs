use axiom_derive::ValueObject;

pub mod axiom {
    pub mod interfaces {
        pub mod ddd {
            pub mod domain {
                use std::fmt::Debug;
                pub trait ValueObject: Debug + Send + Sync + Clone + PartialEq + Eq + 'static {}
            }
        }
    }
}

#[derive(ValueObject)]
struct UnitStruct;

#[test]
fn testUnitStruct() {
    static instance: UnitStruct = UnitStruct;
    
    let _: &dyn std::fmt::Debug = &instance;            // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq
    
    let _: &'static UnitStruct = &instance;             // 'static
}

#[derive(ValueObject)]
struct StructWithNoFields {}

#[test]
fn testStructWithNoFields() {
    static instance: StructWithNoFields = StructWithNoFields {};
    
    let _: &dyn std::fmt::Debug = &instance;            // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq
    
    let _: &'static StructWithNoFields = &instance;     // 'static
}

#[derive(ValueObject)]
struct StructWithNamedFields {
    text: &'static str,
    number: u64,
    flag: bool,
}

#[test]
fn testStructWithNamedFields() {
    static instance: StructWithNamedFields = StructWithNamedFields {
        text: "tomfoolery",
        number: 42,
        flag: false,
    };
    
    let _: &dyn std::fmt::Debug = &instance;            // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq
    
    let _: &'static StructWithNamedFields = &instance;  // 'static
}
