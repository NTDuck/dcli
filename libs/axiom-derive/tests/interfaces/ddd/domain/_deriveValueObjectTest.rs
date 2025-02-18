use std::fmt::Debug;

use axiom::interfaces::ddd::domain::ValueObject;
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

    let _: &dyn Debug = &instance;                      // Debug
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
    
    let _: &dyn Debug = &instance;                      // Debug
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
    
    let _: &dyn Debug = &instance;                      // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq    
}

#[derive(ValueObject)]
struct StructWithUnnamedFields(String, u64, bool);

#[test]
fn testStructWithUnnamedFields() {
    let instance = StructWithUnnamedFields(
        "tomfoolery".to_owned(),
        42,
        false,
    );
    
    let _: &dyn Debug = &instance;                      // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq    
}

#[derive(ValueObject)]
struct StructWithNamedFieldsAndBoundedGenerics<T, U>
where
    T: Sized + Debug + Send + Sync + Clone + PartialEq,
    U: Debug + Send + Sync + Clone + Copy + PartialEq + PartialOrd,
{
    pointer: Box<T>,
    vector: Vec<U>,
}

#[test]
fn testStructWithNamedFieldsAndBoundedGenerics() {
    let instance = StructWithNamedFieldsAndBoundedGenerics {
        pointer: Box::new("tomfoolery".to_owned()),
        vector: vec![0, 1, 2, 3, 4, 5],
    };
    
    let _: &dyn Debug = &&instance;                     // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq    
}

#[derive(ValueObject)]
struct StructWithUnnamedFieldsAndBoundedGenerics<T: Sized + Debug + Send + Sync + Clone + PartialEq, U: Debug + Send + Sync + Clone + Copy + PartialEq + PartialOrd>(Box<T>, Vec<U>);

#[test]
fn testStructWithUnnamedFieldsAndBoundedGenerics() {
    let instance = StructWithUnnamedFieldsAndBoundedGenerics (
        Box::new("tomfoolery".to_owned()),
        vec![0, 1, 2, 3, 4, 5],
    );

    let _: &dyn Debug = &&instance;                     // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq        
}

#[allow(dead_code)]
#[derive(ValueObject)]
enum EnumWithOnlyUnitVariants {
    Quid,
    Pro,
    Quo,
}

#[test]
fn testEnumWithOnlyUnitVariants() {
    let instance = EnumWithOnlyUnitVariants::Quid;
    
    let _: &dyn Debug = &instance;                      // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq    
}

#[allow(dead_code)]
#[derive(ValueObject)]
enum EnumWithStructAndTupleVariants {
    Quid,
    Pro(String, u64, bool),
    Quo {
        text: String,
        number: u64,
        flag: bool,
    },
}

#[test]
fn testEnumWithStructAndTupleVariants() {
    let instance = EnumWithStructAndTupleVariants::Quo {
        text: "tomfoolery".to_owned(),
        number: 42,
        flag: false,
    };
    
    let _: &dyn Debug = &instance;                      // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq    
}

#[allow(dead_code)]
#[derive(ValueObject)]
enum EnumWithStructAndTupleVariantsAndBoundedGenerics<T>
where
    T: ValueObject,
{
    Quid,
    Pro(Vec<T>, u64, bool),
    Quo {
        vector: Vec<T>,
        number: u64,
        flag: bool,
    },
}

#[test]
fn testEnumWithStructAndTupleVariantsAndBoundedGenerics() {
    let instance = EnumWithStructAndTupleVariantsAndBoundedGenerics::Quo {
        vector: vec![
            EnumWithOnlyUnitVariants::Quid,
            EnumWithOnlyUnitVariants::Pro,
            EnumWithOnlyUnitVariants::Quo,
        ],
        number: 42,
        flag: false,
    };
    
    let _: &dyn Debug = &instance;                      // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq    
}
