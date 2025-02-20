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

                pub trait ValueObject: Debug + Send + Sync + Clone + PartialEq + Eq {}
            }
        }
    }
}

#[derive(ValueObject)]
struct UnitStruct;

#[test]
fn testUnitStruct() {
    verifyTraitBounds(UnitStruct);
}

#[derive(ValueObject)]
struct StructWithNoNamedFields {}

#[test]
fn testStructWithNoNamedFields() {
    verifyTraitBounds(StructWithNoNamedFields {});
}

#[derive(ValueObject)]
struct StructWithNoUnnamedFields();

#[test]
fn testStructWithNoUnnamedFields() {
    verifyTraitBounds(StructWithNoUnnamedFields());
}

#[derive(ValueObject)]
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

#[derive(ValueObject)]
struct StructWithUnnamedFields(String, u64, bool);

#[test]
fn testStructWithUnnamedFields() {
    verifyTraitBounds(StructWithUnnamedFields(
        "tomfoolery".to_owned(),
        42,
        false,
    ));
}

#[derive(ValueObject)]
struct StructWithNamedFieldsAndBoundedGenerics<T, U>
where
    T: Debug + Send + Sync + Clone + PartialEq + Eq,
    U: Debug + Send + Sync + Clone + PartialEq + Eq,
{
    pointer: Box<T>,
    vector: Vec<U>,
}

#[test]
fn testStructWithNamedFieldsAndBoundedGenerics() {
    verifyTraitBounds(StructWithNamedFieldsAndBoundedGenerics {
        pointer: Box::new("tomfoolery".to_owned()),
        vector: vec![0, 1, 2, 3, 4, 5],
    });
}

#[derive(ValueObject)]
struct StructWithUnnamedFieldsAndBoundedGenerics<
    T: Debug + Send + Sync + Clone + PartialEq + Eq,
    U: Debug + Send + Sync + Clone + PartialEq + Eq
>(Box<T>, Vec<U>);

#[test]
fn testStructWithUnnamedFieldsAndBoundedGenerics() {
    verifyTraitBounds(StructWithUnnamedFieldsAndBoundedGenerics (
        Box::new("tomfoolery".to_owned()),
        vec![0, 1, 2, 3, 4, 5],
    ));
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
    verifyTraitBounds(EnumWithOnlyUnitVariants::Quid);
    verifyTraitBounds(EnumWithOnlyUnitVariants::Pro);
    verifyTraitBounds(EnumWithOnlyUnitVariants::Quo);
}

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
    verifyTraitBounds(EnumWithStructAndTupleVariants::Quid);
    verifyTraitBounds(EnumWithStructAndTupleVariants::Pro(
        "tomfoolery".to_owned(),
        42,
        false,
    ));
    verifyTraitBounds(EnumWithStructAndTupleVariants::Quo {
        text: "tomfoolery".to_owned(),
        number: 42,
        flag: false,
    });
}

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
    verifyTraitBounds(EnumWithStructAndTupleVariantsAndBoundedGenerics::<UnitStruct>::Quid);
    verifyTraitBounds(EnumWithStructAndTupleVariantsAndBoundedGenerics::Pro(
        vec![
            EnumWithOnlyUnitVariants::Quid,
            EnumWithOnlyUnitVariants::Pro,
            EnumWithOnlyUnitVariants::Quo,
        ],
        42,
        false,
    ));
    verifyTraitBounds(EnumWithStructAndTupleVariantsAndBoundedGenerics::Quo {
        vector: vec![
            EnumWithOnlyUnitVariants::Quid,
            EnumWithOnlyUnitVariants::Pro,
            EnumWithOnlyUnitVariants::Quo,
        ],
        number: 42,
        flag: false,
    });
}

fn verifyTraitBounds(_: impl ValueObject) {}
