use std::fmt::Debug;

use axiom_derive::New;

#[derive(New, PartialEq, Debug)]
struct OrdinaryStructWithNoFields {}

#[test]
fn testOrdinaryStructWithNoFields() {
    verifyTraitBounds(
        OrdinaryStructWithNoFields::new(),
        OrdinaryStructWithNoFields {},
    );
}

#[derive(New, PartialEq, Debug)]
struct OrdinaryStructWithThreeFields {
    text: String,
    number: u64,
    flag: bool,
}

#[test]
fn testOrdinaryStructWithThreeFields() {
    verifyTraitBounds(
        OrdinaryStructWithThreeFields::new("tomfoolery".to_owned(), 42, false),
        OrdinaryStructWithThreeFields {
            text: "tomfoolery".to_owned(),
            number: 42,
            flag: false,
        },
    );
}

#[derive(New, PartialEq, Debug)]
struct OrdinaryStructWithGenerics<T, U> {
    pointer: Box<Box<Box<T>>>,
    vector: Vec<U>,
}

#[test]
fn testOrdinaryStructWithGenerics() {
    verifyTraitBounds(
        OrdinaryStructWithGenerics::new(
            Box::new(Box::new(Box::new(OrdinaryStructWithNoFields::new()))),
            vec![
                OrdinaryStructWithNoFields::new(),
                OrdinaryStructWithNoFields::new(),
                OrdinaryStructWithNoFields::new(),
            ],
        ),
        OrdinaryStructWithGenerics {
            pointer: Box::new(Box::new(Box::new(OrdinaryStructWithNoFields {}))),
            vector: vec![
                OrdinaryStructWithNoFields {},
                OrdinaryStructWithNoFields {},
                OrdinaryStructWithNoFields {},
            ],
        },
    );
}

#[derive(New, PartialEq, Debug)]
struct OrdinaryStructWithLifetimes<'a, 'b, 'c> {
    text: &'a str,
    number: &'b u64,
    flag: &'c bool,
}

#[test]
fn testOrdinaryStructWithLifetimes() {
    let (text, number, flag) = (
        "tomfoolery".to_owned(),
        42,
        false,
    );

    verifyTraitBounds(
        OrdinaryStructWithLifetimes::new(
            &text,
            &number,
            &flag,
        ),
        OrdinaryStructWithLifetimes {
            text: &text,
            number: &number,
            flag: &flag,
        },
    );
}

#[derive(New, PartialEq, Debug)]
struct TupleStructWithNoFields();

#[test]
fn testTupleStructWithNoFields() {
    verifyTraitBounds(
        TupleStructWithNoFields::new(),
        TupleStructWithNoFields(),
    );
}

#[derive(New, PartialEq, Debug)]
struct TupleStructWithThreeFields(String, u64, bool);

#[test]
fn testTupleStructWithThreeFields() {
    verifyTraitBounds(
        TupleStructWithThreeFields::new("tomfoolery".to_owned(), 42, false),
        TupleStructWithThreeFields("tomfoolery".to_owned(), 42, false),
    );
}

#[derive(New, PartialEq, Debug)]
struct NewType(String);

#[test]
fn testNewType() {
    verifyTraitBounds(
        NewType::new("tomfoolery".to_owned()),
        NewType("tomfoolery".to_owned()),
    );
}

#[derive(New, PartialEq, Debug)]
struct TupleStructWithGenerics<T, U>(
    Box<Box<Box<T>>>,
    Vec<U>,
);

#[test]
fn testTupleStructWithGenerics() {
    verifyTraitBounds(
        TupleStructWithGenerics::new(
            Box::new(Box::new(Box::new(
                TupleStructWithNoFields::new(),
            ))),
            vec![
                TupleStructWithNoFields::new(),
                TupleStructWithNoFields::new(),
                TupleStructWithNoFields::new(),
            ],
        ),
        TupleStructWithGenerics(
            Box::new(Box::new(Box::new(
                TupleStructWithNoFields(),
            ))),
            vec![
                TupleStructWithNoFields(),
                TupleStructWithNoFields(),
                TupleStructWithNoFields(),
            ],
        ),
    );
}

#[derive(New, PartialEq, Debug)]
struct NewTypeWithGenerics<T>(Box<Box<Box<T>>>);

#[test]
fn testNewTypeWithGenerics() {
    verifyTraitBounds(
        NewTypeWithGenerics::new(
            Box::new(Box::new(Box::new(
                TupleStructWithNoFields(),
            ))),
        ),
        NewTypeWithGenerics(
            Box::new(Box::new(Box::new(
                TupleStructWithNoFields(),
            ))),
        ),
    );
}

#[derive(New, PartialEq, Debug)]
struct TupleStructWithLifetimes<'a, 'b, 'c>(
    &'a str,
    &'b u64,
    &'c bool,
);

#[test]
fn testTupleStructWithLifetimes() {
    let (text, number, flag) = (
        "tomfoolery".to_owned(),
        42,
        false,
    );

    verifyTraitBounds(
        TupleStructWithLifetimes::new(
            &text,
            &number,
            &flag,
        ), TupleStructWithLifetimes(
            &text,
            &number,
            &flag,
        ),
    );
}

#[derive(New, PartialEq, Debug)]
struct EnumWithOnlyUnitVariants;

#[test]
fn testEnumWithOnlyUnitVariants() {
    verifyTraitBounds(
        EnumWithOnlyUnitVariants::new(), EnumWithOnlyUnitVariants,
    );
}

#[derive(New, PartialEq, Debug)]
enum EnumWithOnlyStructVariants {
    Quid {
        text: String,
    },
    Pro {
        number: u64,
    },
    Quo {
        flag: bool,
    },
}

#[test]
fn testEnumWithOnlyStructVariants() {
    verifyTraitBounds(
        EnumWithOnlyStructVariants::new_quid(
            "tomfoolery".to_owned(),
        ),
        EnumWithOnlyStructVariants::Quid {
            text: "tomfoolery".to_owned(),
        },
    );
    verifyTraitBounds(
        EnumWithOnlyStructVariants::new_pro(
            42,
        ),
        EnumWithOnlyStructVariants::Pro {
            number: 42,
        },
    );
    verifyTraitBounds(
        EnumWithOnlyStructVariants::new_quo(
            false,
        ),
        EnumWithOnlyStructVariants::Quo {
            flag: false,
        },
    );
}

#[derive(New, PartialEq, Debug)]
enum EnumWithOnlyTupleVariants {
    Quid(String),
    Pro(u64),
    Quo(bool),
}

#[test]
fn testEnumWithOnlyTupleVariants() {
    verifyTraitBounds(
        EnumWithOnlyTupleVariants::new_quid("tomfoolery".to_owned()),
        EnumWithOnlyTupleVariants::Quid("tomfoolery".to_owned()),
    );
    verifyTraitBounds(
        EnumWithOnlyTupleVariants::new_pro(42),
        EnumWithOnlyTupleVariants::Pro(42),
    );
    verifyTraitBounds(
        EnumWithOnlyTupleVariants::new_quo(false),
        EnumWithOnlyTupleVariants::Quo(false),
    );
}

#[derive(New, PartialEq, Debug)]
enum EnumWithMixedVariants {
    Quid {
        text: String,
        number: u64,
        flag: bool,
    },
    Pro(String, u64, bool),
    Quo,
}

#[test]
fn testEnumWithMixedVariants() {
    verifyTraitBounds(
        EnumWithMixedVariants::new_quid(
            "tomfoolery".to_owned(),
            42,
            false,
        ),
        EnumWithMixedVariants::Quid {
            text: "tomfoolery".to_owned(),
            number: 42,
            flag: false,
        },
    );
    verifyTraitBounds(
        EnumWithMixedVariants::new_pro(
            "tomfoolery".to_owned(), 
            42, 
            false,
        ),
        EnumWithMixedVariants::Pro(
            "tomfoolery".to_owned(),
            42,
            false,
        ),
    );
    verifyTraitBounds(
        EnumWithMixedVariants::new_quo(), EnumWithMixedVariants::Quo,
    );
}


fn verifyTraitBounds<T: Debug + PartialEq>(factoryConstructed: T, manuallyConstructed: T) {
    assert_eq!(factoryConstructed, manuallyConstructed);
}
