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
struct StructWithNamedFieldsAndLifetimes {
    text: String,
    number: u64,
    flag: bool,
}

#[test]
fn testStrutWithNamedFieldsAndLifetimes() {

}