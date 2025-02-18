use std::fmt::Debug;

use axiom_derive::New;

#[derive(New, PartialEq, Debug)]
struct StructWithNoFields {}

#[test]
fn testStructWithNoFields() {
    let factoryConstructedInstance = StructWithNoFields::new();
    let manuallyConstructedInstance = StructWithNoFields {};

    assert_eq!(factoryConstructedInstance, manuallyConstructedInstance);
}

#[derive(New, PartialEq, Debug)]
struct UnitStruct;

#[test]
fn testUnitStruct() {
    let factoryConstructedInstance = UnitStruct::new();
    let manuallyConstructedInstance = UnitStruct;

    assert_eq!(factoryConstructedInstance, manuallyConstructedInstance);
}

#[derive(New, PartialEq, Debug)]
struct StructWithNamedFields {
    text: String,
    number: u64,
    flag: bool,
}

#[test]
fn testStructWithNamedFields() {
    let factoryConstructedInstance = StructWithNamedFields::new(
        "tomfoolery".to_owned(),
        42,
        false,
    );
    let manuallyConstructedInstance = StructWithNamedFields {
        text: "tomfoolery".to_owned(),
        number: 42,
        flag: false,
    };

    assert_eq!(factoryConstructedInstance, manuallyConstructedInstance);
}

#[derive(New, PartialEq, Debug)]
struct StructWithUnnamedFields(String, u64, bool);

#[test]
fn testStructWithUnnamedFields() {
    let factoryConstructedInstance = StructWithUnnamedFields::new(
        "tomfoolery".to_owned(), 42, false,
    );
    let manuallyConstructedInstance = StructWithUnnamedFields(
        "tomfoolery".to_owned(), 42, false,
    );

    assert_eq!(factoryConstructedInstance, manuallyConstructedInstance);
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

    let factoryConstructedInstance = StructWithNamedFieldsAndLifetimes::new(
        &text,
        &number,
        &flag,
    );
    let manuallyConstructedInstance = StructWithNamedFieldsAndLifetimes {
        text: &text,
        number: &number,
        flag: &flag,
    };

    assert_eq!(factoryConstructedInstance, manuallyConstructedInstance);
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

    let factoryConstructedInstance = StructWithUnnamedFieldsAndLifetimes::new(&text, &number, &flag);
    let manuallyConstructedInstance = StructWithUnnamedFieldsAndLifetimes(&text, &number, &flag);
    
    assert_eq!(factoryConstructedInstance, manuallyConstructedInstance);
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
    let factoryConstructedInstance = StructWithNamedFieldsAndBoundedGenerics::new(
        Box::new("tomfoolery".to_owned()),
        vec![0, 1, 2, 3, 4, 5],
    );
    let manuallyConstructedInstance = StructWithNamedFieldsAndBoundedGenerics {
        pointer: Box::new("tomfoolery".to_owned()),
        vector: vec![0, 1, 2, 3, 4, 5],
    };

    assert_eq!(factoryConstructedInstance, manuallyConstructedInstance);
}

#[derive(New, PartialEq, Debug)]
struct StructWithUnnamedFieldsAndBoundedGenerics<T: ?Sized, U: Debug + Clone + Copy + PartialEq + PartialOrd>(Box<T>, Vec<U>);

#[test]
fn testStructWithUnnamedFieldsAndBoundedGenerics() {
    let factoryConstructedInstance = StructWithUnnamedFieldsAndBoundedGenerics::new(
        Box::new("tomfoolery".to_owned()),
        vec![0, 1, 2, 3, 4, 5],
    );
    let manuallyConstructedInstance = StructWithUnnamedFieldsAndBoundedGenerics (
        Box::new("tomfoolery".to_owned()),
        vec![0, 1, 2, 3, 4, 5],
    );

    assert_eq!(factoryConstructedInstance, manuallyConstructedInstance);
}