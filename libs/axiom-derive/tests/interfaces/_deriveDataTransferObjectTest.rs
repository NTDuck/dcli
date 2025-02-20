// use std::fmt::Debug;

use axiom::interfaces::DataTransferObject;
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
    verifyTraitBounds(UnitStruct);
}

#[derive(DataTransferObject, Serialize, Deserialize)]
struct StructWithNoNamedFields {}

#[test]
fn testStructWithNoNamedFields() {
    verifyTraitBounds(StructWithNoNamedFields {});
}

#[derive(DataTransferObject, Serialize, Deserialize)]
struct StructWithNoUnnamedFields();

#[test]
fn testStructWithNoUnnamedFields() {
    verifyTraitBounds(StructWithNoUnnamedFields());
}

#[derive(DataTransferObject, Serialize, Deserialize)]
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

#[derive(DataTransferObject, Serialize, Deserialize)]
struct StructWithUnnamedFields(String, u64, bool);

#[test]
fn testStructWithUnnamedFields() {
    verifyTraitBounds(StructWithUnnamedFields(
        "tomfoolery".to_owned(),
        42,
        false,
    ));
}

// #[derive(DataTransferObject, Serialize, Deserialize)]
// struct StructWithNamedFieldsAndBoundedGenerics<T, U>
// where
//     T: Debug + Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>,
//     U: Debug + Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>,
// {
//     pointer: Box<T>,
//     vector: Vec<U>,
// }

// #[test]
// fn testStructWithNamedFieldsAndBoundedGenerics() {
//     verifyTraitBounds(StructWithNamedFieldsAndBoundedGenerics {
//         pointer: Box::new("tomfoolery".to_owned()),
//         vector: vec![0, 1, 2, 3, 4, 5],
//     });
// }

// #[derive(DataTransferObject, Serialize, Deserialize)]
// struct StructWithUnnamedFieldsAndBoundedGenerics<
//     T: Debug + Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>,
//     U: Debug + Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>,
// >(Box<T>, Vec<U>);

// #[test]
// fn testStructWithUnnamedFieldsAndBoundedGenerics() {
//     verifyTraitBounds(StructWithUnnamedFieldsAndBoundedGenerics (
//         Box::new("tomfoolery".to_owned()),
//         vec![0, 1, 2, 3, 4, 5],
//     ));
// }

#[allow(dead_code)]
#[derive(DataTransferObject, Serialize, Deserialize)]
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

#[derive(DataTransferObject, Serialize, Deserialize)]
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

// #[derive(DataTransferObject, Serialize, Deserialize)]
// enum EnumWithStructAndTupleVariantsAndBoundedGenerics<T>
// where
//     T: DataTransferObject,
//     Vec<T>: for<'a> Deserialize<'a>,
// {
//     Quid,
//     Pro(Vec<T>, u64, bool),
//     Quo {
//         vector: Vec<T>,
//         number: u64,
//         flag: bool,
//     },
// }

// #[test]
// fn testEnumWithStructAndTupleVariantsAndBoundedGenerics() {
//     verifyTraitBounds(EnumWithStructAndTupleVariantsAndBoundedGenerics::<UnitStruct>::Quid);
//     verifyTraitBounds(EnumWithStructAndTupleVariantsAndBoundedGenerics::Pro(
//         vec![
//             EnumWithOnlyUnitVariants::Quid,
//             EnumWithOnlyUnitVariants::Pro,
//             EnumWithOnlyUnitVariants::Quo,
//         ],
//         42,
//         false,
//     ));
//     verifyTraitBounds(EnumWithStructAndTupleVariantsAndBoundedGenerics::Quo {
//         vector: vec![
//             EnumWithOnlyUnitVariants::Quid,
//             EnumWithOnlyUnitVariants::Pro,
//             EnumWithOnlyUnitVariants::Quo,
//         ],
//         number: 42,
//         flag: false,
//     });
// }

fn verifyTraitBounds(_: impl DataTransferObject) {}
