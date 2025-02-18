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

struct StructWithNoFields {}


struct StructWithNamedFields {
    text: String,
    number: u64,
    flag: bool,
}

#[test]
fn testUnitStruct() {
    let instance = UnitStruct;

    let _: &dyn std::fmt::Debug = &instance;        // Debug
    let _: &dyn Send = &instance;                   // Send
    let _: &dyn Sync = &instance;                   // Sync
    let _ = instance.clone();                       // Clone
    let _: &dyn PartialEq<UnitStruct> = &instance;  // PartialEq
    // let _: &dyn Eq = &Instance;                     // Eq
    
    static Instance: UnitStruct = UnitStruct;
    let _: &'static UnitStruct = &Instance;         // 'static
}
