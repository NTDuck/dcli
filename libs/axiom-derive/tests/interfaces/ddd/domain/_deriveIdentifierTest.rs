use axiom::interfaces::ddd::domain::Identifier;
use axiom_derive::Identifier;

// Allow `Identifier` usage
// without adding `axiom` as a dependency
pub mod axiom {
    pub mod interfaces {
        pub mod ddd {
            pub mod domain {
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
    verifyTraitBounds(UnitStruct);
}

#[derive(Identifier)]
struct StructWithNoNamedFields {}

#[test]
fn testStructWithNoNamedFields() {
    verifyTraitBounds(StructWithNoNamedFields {});
}

#[derive(Identifier)]
struct StructWithNoUnnamedFields();

#[test]
fn testStructWithNoUnnamedFields() {
    verifyTraitBounds(StructWithNoUnnamedFields());
}

#[derive(Identifier)]
struct StructWithNamedFields {
    text: String,
    number: u64,
    flag: bool,
}

#[test]
fn testStructWithNamedFields() {
    verifyTraitBounds(StructWithNamedFields {
        text: "tomfoolery".to_owned(),
        number: 42,
        flag: false,
    });
}

#[derive(Identifier)]
struct StructWithUnnamedFields(String, u64, bool);

#[test]
fn testStructWithUnnamedFields() {
    verifyTraitBounds(StructWithUnnamedFields(
        "tomfoolery".to_owned(),
        42,
        false,
    ));
}

#[derive(Identifier)]
struct StructWithNamedFieldsAndBoundedGenerics<T, U>
where
    T: Identifier,
    U: Identifier,
{
    pointer: Box<T>,
    vector: Vec<U>,
}

#[test]
fn testStructWithNamedFieldsAndBoundedGenerics() {
    verifyTraitBounds(StructWithNamedFieldsAndBoundedGenerics {
        pointer: Box::new(UnitStruct),
        vector: vec![UnitStruct],
    });
}

#[derive(Identifier)]
struct StructWithUnnamedFieldsAndBoundedGenerics<
    T: Identifier,
    U: Identifier,
>(Box<T>, Vec<U>);

#[test]
fn testStructWithUnnamedFieldsAndBoundedGenerics() {
    verifyTraitBounds(StructWithUnnamedFieldsAndBoundedGenerics (
        Box::new(UnitStruct),
        vec![UnitStruct],
    ));
}

fn verifyTraitBounds(_: impl Identifier) {}
