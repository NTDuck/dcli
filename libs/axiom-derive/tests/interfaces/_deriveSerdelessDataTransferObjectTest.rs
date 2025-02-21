use axiom::interfaces::DataTransferObject;
use axiom_derive::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

// Allow `DataTransferObject` usage
// without adding `axiom` as a dependency
pub mod axiom {
    pub mod interfaces {
        use std::fmt::Debug;

        use serde::Serialize;
        use serde::Deserialize;

        pub trait DataTransferObject: Debug + Send + Sync + Clone + Serialize + for<'de> Deserialize<'de> {}
    }
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
struct UnitStruct;

#[test]
fn testUnitStruct() {
    verifyTraitBounds(UnitStruct);
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
struct StructWithNoNamedFields {}

#[test]
fn testStructWithNoNamedFields() {
    verifyTraitBounds(StructWithNoNamedFields {});
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
struct StructWithNoUnnamedFields();

#[test]
fn testStructWithNoUnnamedFields() {
    verifyTraitBounds(StructWithNoUnnamedFields());
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
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

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
struct StructWithUnnamedFields(String, u64, bool);

#[test]
fn testStructWithUnnamedFields() {
    verifyTraitBounds(StructWithUnnamedFields(
        "tomfoolery".to_owned(),
        42,
        false,
    ));
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
struct StructWithNamedFieldsAndBoundedGenerics<T, U> {
    pointer: Box<T>,
    vector: Vec<U>,
}

#[test]
fn testStructWithNamedFieldsAndBoundedGenerics() {
    verifyTraitBounds(StructWithNamedFieldsAndBoundedGenerics {
        pointer: Box::new(UnitStruct),
        vector: vec![UnitStruct, UnitStruct, UnitStruct],
    });
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
struct StructWithUnnamedFieldsAndBoundedGenerics<T, U>(Box<T>, Vec<U>);

#[test]
fn testStructWithUnnamedFieldsAndBoundedGenerics() {
    verifyTraitBounds(StructWithUnnamedFieldsAndBoundedGenerics (
        Box::new(UnitStruct),
        vec![UnitStruct, UnitStruct, UnitStruct],
    ));
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
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

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
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

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
enum EnumWithStructAndTupleVariantsAndBoundedGenerics<T> {
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
        vec![UnitStruct, UnitStruct, UnitStruct],
        42,
        false,
    ));
    verifyTraitBounds(EnumWithStructAndTupleVariantsAndBoundedGenerics::Quo {
        vector: vec![UnitStruct, UnitStruct, UnitStruct],
        number: 42,
        flag: false,
    });
}

fn verifyTraitBounds(_: impl DataTransferObject) {}
