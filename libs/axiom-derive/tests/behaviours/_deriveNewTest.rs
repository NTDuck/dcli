use std::fmt::Debug;

use axiom_derive::New;

#[derive(New, PartialEq, Debug)]
struct UnitStruct;

#[test]
fn testUnitStruct() {
    verifyTraitBounds(
        UnitStruct::new(),
        UnitStruct,
    );
}

#[derive(New, PartialEq, Debug)]
struct StructWithNoNamedFields {}

#[test]
fn testStructWithNoNamedFields() {
    verifyTraitBounds(
        StructWithNoNamedFields::new(),
        StructWithNoNamedFields {},
    );
}

#[derive(New, PartialEq, Debug)]
struct StructWithNoUnnamedFields();

#[test]
fn testStructWithNoUnnamedFields() {
    verifyTraitBounds(
        StructWithNoUnnamedFields::new(),
        StructWithNoUnnamedFields(),
    );
}

#[derive(New, PartialEq, Debug)]
struct StructWithNamedFields {
    text: String,
    number: u64,
    flag: bool,
}

#[test]
fn testStructWithNamedFields() {
    verifyTraitBounds(
        StructWithNamedFields::new(
            "tomfoolery".to_owned(),
            42,
            false,
        ),
        StructWithNamedFields {
            text: "tomfoolery".to_owned(),
            number: 42,
            flag: false,
        },
    );
}

#[derive(New, PartialEq, Debug)]
struct StructWithUnnamedFields(String, u64, bool);

#[test]
fn testStructWithUnnamedFields() {
    verifyTraitBounds(
        StructWithUnnamedFields::new(
            "tomfoolery".to_owned(),
            42,
            false,
        ),
        StructWithUnnamedFields(
            "tomfoolery".to_owned(),
            42,
            false,
        ),
    );
}

#[derive(New, PartialEq, Debug)]
struct StructWithNamedFieldsAndLifetimes<'a, 'b, 'c> {
    text: &'a str,
    number: &'b u64,
    flag: &'c bool,
}

#[test]
fn testStructWithNamedFieldsAndLifetimes() {
    let (text, number, flag) = (
        "tomfoolery".to_owned(),
        42,
        false,
    );

    verifyTraitBounds(
        StructWithNamedFieldsAndLifetimes::new(
            &text,
            &number,
            &flag,
        ),
        StructWithNamedFieldsAndLifetimes {
            text: &text,
            number: &number,
            flag: &flag,
        },
    );
}

#[derive(New, PartialEq, Debug)]
struct StructWithUnnamedFieldsAndLifetimes<'a, 'b, 'c>(&'a str, &'b u64, &'c bool);

#[test]
fn testStructWithUnnamedFieldsAndLifetimes() {
    let (text, number, flag) = (
        "tomfoolery".to_owned(),
        42,
        false,
    );

    verifyTraitBounds(
        StructWithUnnamedFieldsAndLifetimes::new(
            &text,
            &number,
            &flag,
        ),
        StructWithUnnamedFieldsAndLifetimes(
            &text,
            &number,
            &flag,
        ),
    );
}

#[derive(New, PartialEq, Debug)]
struct StructWithNamedFieldsAndBoundedGenerics<T, U>
where
    T: ?Sized,
    U: Debug + Clone + Copy + PartialEq + PartialOrd,
{
    pointer: Box<T>,
    vector: Vec<U>,
}

#[test]
fn testStructWithNamedFieldsAndBoundedGenerics() {
    verifyTraitBounds(
        StructWithNamedFieldsAndBoundedGenerics::new(
            Box::new("tomfoolery".to_owned()),
            vec![0, 1, 2, 3, 4, 5],
        ),
        StructWithNamedFieldsAndBoundedGenerics {
            pointer: Box::new("tomfoolery".to_owned()),
            vector: vec![0, 1, 2, 3, 4, 5],
        },
    );
}

#[derive(New, PartialEq, Debug)]
struct StructWithUnnamedFieldsAndBoundedGenerics<T: ?Sized, U: Debug + Clone + Copy + PartialEq + PartialOrd>(Box<T>, Vec<U>);

#[test]
fn testStructWithUnnamedFieldsAndBoundedGenerics() {
    verifyTraitBounds(
        StructWithUnnamedFieldsAndBoundedGenerics::new(
            Box::new("tomfoolery".to_owned()),
            vec![0, 1, 2, 3, 4, 5],
        ),
        StructWithUnnamedFieldsAndBoundedGenerics (
            Box::new("tomfoolery".to_owned()),
            vec![0, 1, 2, 3, 4, 5],
        ),
    );
}

#[derive(New, PartialEq, Debug)]
struct StructedWithNamedFieldsAndLifetimesAndBoundedGenerics<'a, 'b, T, U>
where
    T: ?Sized,
    U: Debug + Clone + Copy + PartialEq + PartialOrd,
{
    pointer: &'a Box<T>,
    vector: &'b Vec<U>,
}

#[test]
fn testStructWithNamedFieldsAndLifetimesAndBoundedGenerics() {
    let (pointer, vector) = (
        Box::new("tomfoolery".to_owned()),
        vec![0, 1, 2, 3, 4, 5],
    );

    verifyTraitBounds(
        StructedWithNamedFieldsAndLifetimesAndBoundedGenerics::new(
            &pointer,
            &vector,
        ),
        StructedWithNamedFieldsAndLifetimesAndBoundedGenerics {
            pointer: &pointer,
            vector: &vector,
        },
    );
}

#[derive(New, PartialEq, Debug)]
struct StructWithUnnamedFieldsAndLifetimesAndBoundedGenerics<'a, 'b, T: ?Sized, U: Debug + Clone + Copy + PartialEq + PartialOrd>(&'a Box<T>, &'b Vec<U>);

#[test]
fn testStructWithUnnamedFieldsAndLifetimesAndBoundedGenerics() {
    let (pointer, vector) = (
        Box::new("tomfoolery".to_owned()),
        vec![0, 1, 2, 3, 4, 5],
    );

    verifyTraitBounds(
        StructWithUnnamedFieldsAndLifetimesAndBoundedGenerics::new(
            &pointer,
            &vector,
        ),
        StructWithUnnamedFieldsAndLifetimesAndBoundedGenerics(
            &pointer,
            &vector,
        ),
    );
}

#[derive(New, PartialEq, Debug)]
enum EnumWithOnlyVariants {
    Quid,
    Pro,
    Quo,
}

#[test]
fn testEnumWithOnlyVariants() {
    verifyTraitBounds(
        EnumWithOnlyVariants::newQuid(),
        EnumWithOnlyVariants::Quid,
    );
    verifyTraitBounds(
        EnumWithOnlyVariants::newPro(),
        EnumWithOnlyVariants::Pro,
    );
    verifyTraitBounds(
        EnumWithOnlyVariants::newQuo(),
        EnumWithOnlyVariants::Quo,
    );
}

#[derive(New, PartialEq, Debug)]
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
    verifyTraitBounds(
        EnumWithStructAndTupleVariants::newQuid(),
        EnumWithStructAndTupleVariants::Quid,
    );
    verifyTraitBounds(
        EnumWithStructAndTupleVariants::newPro(
            "tomfoolery".to_owned(),
            42,
            false,
        ),
        EnumWithStructAndTupleVariants::Pro(
            "tomfoolery".to_owned(),
            42,
            false,
        ),
    );
    verifyTraitBounds(
        EnumWithStructAndTupleVariants::newQuo(
            "tomfoolery".to_owned(),
            42,
            false,
        ),
        EnumWithStructAndTupleVariants::Quo {
            text: "tomfoolery".to_owned(),
            number: 42,
            flag: false,
        },
    );
}

#[derive(New, PartialEq, Debug)]
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
    verifyTraitBounds(
        EnumWithStructAndTupleVariantsAndBoundedGenerics::<UnitStruct>::newQuid(),
        EnumWithStructAndTupleVariantsAndBoundedGenerics::Quid,
    );
    verifyTraitBounds(
        EnumWithStructAndTupleVariantsAndBoundedGenerics::newPro(
            vec![0, 1, 2, 3, 4],
            42,
            false,
        ),
        EnumWithStructAndTupleVariantsAndBoundedGenerics::Pro(
            vec![0, 1, 2, 3, 4],
            42,
            false,
        ),
    );
    verifyTraitBounds(
        EnumWithStructAndTupleVariantsAndBoundedGenerics::newQuo(
            vec![0, 1, 2, 3, 4],
            42,
            false,
        ),
        EnumWithStructAndTupleVariantsAndBoundedGenerics::Quo {
            vector: vec![0, 1, 2, 3, 4],
            number: 42,
            flag: false,
        },
    );
}

fn verifyTraitBounds<T: Debug + PartialEq>(factoryConstructed: T, manuallyConstructed: T) {
    assert_eq!(factoryConstructed, manuallyConstructed);
}