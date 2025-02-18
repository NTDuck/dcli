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
