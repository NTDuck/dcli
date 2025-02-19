use std::fmt::Debug;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use axiom::interfaces::ddd::domain::Identifier;
use axiom_derive::Identifier;

// Allow `Identifier` usage
// without adding `axiom` as a dependency
pub mod axiom {
    pub mod interfaces {
        pub mod ddd {
            pub mod domain {
                #![allow(dead_code)]

                use std::fmt::Debug;
                use std::hash::Hash;

                pub trait Identifier: ValueObject + Hash {}
                pub trait ValueObject: Debug + Send + Sync + Clone + PartialEq + Eq {}
            }
        }
    }
}

#[derive(Identifier)]
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

    let mut hasher = DefaultHasher::new();
    instance.hash(&mut hasher);                         // Hash
    let _ = hasher.finish();
}

#[derive(Identifier)]
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

    let mut hasher = DefaultHasher::new();
    instance.hash(&mut hasher);                         // Hash
    let _ = hasher.finish();
}

#[derive(Identifier)]
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

    let mut hasher = DefaultHasher::new();
    instance.hash(&mut hasher);                         // Hash
    let _ = hasher.finish();
}

#[derive(Identifier)]
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

    let mut hasher = DefaultHasher::new();
    instance.hash(&mut hasher);                         // Hash
    let _ = hasher.finish();
}

#[derive(Identifier)]
struct StructWithNamedFieldsAndBoundedGenerics<T, U>
where
    T: Sized + Debug + Send + Sync + Clone + PartialEq + Hash,
    U: Debug + Send + Sync + Clone + Copy + PartialEq + PartialOrd + Hash,
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

    let mut hasher = DefaultHasher::new();
    instance.hash(&mut hasher);                         // Hash
    let _ = hasher.finish();
}

#[derive(Identifier)]
struct StructWithUnnamedFieldsAndBoundedGenerics<T: Sized + Debug + Send + Sync + Clone + PartialEq + Hash, U: Debug + Send + Sync + Clone + Copy + PartialEq + PartialOrd + Hash>(Box<T>, Vec<U>);

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
    
    let mut hasher = DefaultHasher::new();
    instance.hash(&mut hasher);                         // Hash
    let _ = hasher.finish();
}

#[allow(dead_code)]
#[derive(Identifier)]
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
        
    let mut hasher = DefaultHasher::new();
    instance.hash(&mut hasher);                         // Hash
    let _ = hasher.finish();
}

#[allow(dead_code)]
#[derive(Identifier)]
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

    let _: &dyn Debug = &&instance;                     // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq
    
    let mut hasher = DefaultHasher::new();
    instance.hash(&mut hasher);                         // Hash
    let _ = hasher.finish();
}

#[allow(dead_code)]
#[derive(Identifier)]
enum EnumWithStructAndTupleVariantsAndBoundedGenerics<T>
where
    T: Identifier,
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
        vector: vec![UnitStruct],
        number: 42,
        flag: false,
    };

    let _: &dyn Debug = &&instance;                     // Debug
    let _: &dyn Send = &instance;                       // Send
    let _: &dyn Sync = &instance;                       // Sync
    
    let clonedInstance = instance.clone();              // Clone
    assert_eq!(instance, clonedInstance);               // PartialEq
    assert_eq!(instance, instance);                     // Eq
    
    let mut hasher = DefaultHasher::new();
    instance.hash(&mut hasher);                         // Hash
    let _ = hasher.finish();
}