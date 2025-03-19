use std::fmt::Debug;

use axiom_derive::New;

#[derive(New, PartialEq, Debug)]
struct OrdinaryStructWithNoFields {}

#[test]
fn test_ordinary_struct_with_no_fields() {
    verify_trait_bounds(
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
fn test_ordinary_struct_with_three_fields() {
    verify_trait_bounds(
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
fn test_ordinary_struct_with_generics() {
    verify_trait_bounds(
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
fn test_ordinary_struct_with_lifetimes() {
    let (text, number, flag) = (
        "tomfoolery".to_owned(),
        42,
        false,
    );

    verify_trait_bounds(
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
fn test_tuple_struct_with_no_fields() {
    verify_trait_bounds(
        TupleStructWithNoFields::new(),
        TupleStructWithNoFields(),
    );
}

#[derive(New, PartialEq, Debug)]
struct TupleStructWithThreeFields(String, u64, bool);

#[test]
fn test_tuple_struct_with_three_fields() {
    verify_trait_bounds(
        TupleStructWithThreeFields::new("tomfoolery".to_owned(), 42, false),
        TupleStructWithThreeFields("tomfoolery".to_owned(), 42, false),
    );
}

#[derive(New, PartialEq, Debug)]
struct NewType(String);

#[test]
fn test_new_type() {
    verify_trait_bounds(
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
fn test_tuple_struct_with_generics() {
    verify_trait_bounds(
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
fn test_new_type_with_generics() {
    verify_trait_bounds(
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
fn test_tuple_struct_with_lifetimes() {
    let (text, number, flag) = (
        "tomfoolery".to_owned(),
        42,
        false,
    );

    verify_trait_bounds(
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
fn test_enum_with_only_unit_variants() {
    verify_trait_bounds(
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
fn test_enum_with_only_struct_variants() {
    verify_trait_bounds(
        EnumWithOnlyStructVariants::new_quid(
            "tomfoolery".to_owned(),
        ),
        EnumWithOnlyStructVariants::Quid {
            text: "tomfoolery".to_owned(),
        },
    );
    verify_trait_bounds(
        EnumWithOnlyStructVariants::new_pro(
            42,
        ),
        EnumWithOnlyStructVariants::Pro {
            number: 42,
        },
    );
    verify_trait_bounds(
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
fn test_enum_with_only_tuple_variants() {
    verify_trait_bounds(
        EnumWithOnlyTupleVariants::new_quid("tomfoolery".to_owned()),
        EnumWithOnlyTupleVariants::Quid("tomfoolery".to_owned()),
    );
    verify_trait_bounds(
        EnumWithOnlyTupleVariants::new_pro(42),
        EnumWithOnlyTupleVariants::Pro(42),
    );
    verify_trait_bounds(
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
fn test_enum_with_mixed_variants() {
    verify_trait_bounds(
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
    verify_trait_bounds(
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
    verify_trait_bounds(
        EnumWithMixedVariants::new_quo(), EnumWithMixedVariants::Quo,
    );
}


fn verify_trait_bounds<T: Debug + PartialEq>(factory_constructed: T, manually_constructed: T) {
    assert_eq!(factory_constructed, manually_constructed);
}
