macro_rules! __test_common_ordinary_structs {
    ($derive_clause:tt, $test_fn:ident) => {
        #$derive_clause
        struct OrdinaryStructWithNoFields {}

        #[test]
        fn test_ordinary_struct_with_no_fields() {
            $test_fn(OrdinaryStructWithNoFields {});
        }

        #$derive_clause
        struct OrdinaryStructWithThreeFields {
            text: String,
            number: u64,
            flag: bool,
        }

        #[test]
        fn test_ordinary_struct_with_three_fields() {
            $test_fn(OrdinaryStructWithThreeFields {
                text: "tomfoolery".to_owned(),
                number: 42,
                flag: false,
            });
        }

        #$derive_clause
        struct OrdinaryStructWithGenerics<T, U> {
            pointer: Box<Box<Box<T>>>,
            vector: Vec<U>,
        }

        #[test]
        fn test_ordinary_struct_with_generics() {
            $test_fn(OrdinaryStructWithGenerics {
                pointer: Box::new(Box::new(Box::new(
                    OrdinaryStructWithNoFields {},
                ))),
                vector: vec! [
                    OrdinaryStructWithNoFields {},
                    OrdinaryStructWithNoFields {},
                    OrdinaryStructWithNoFields {},
                ],
            });
        }
    };
}

macro_rules! __test_common_tuple_structs {
    ($derive_clause:tt, $test_fn:ident) => {
        #$derive_clause
        struct TupleStructWithNoFields();

        #[test]
        fn test_tuple_struct_with_no_fields() {
            $test_fn(TupleStructWithNoFields());
        }

        #$derive_clause
        struct TupleStructWithThreeFields(String, u64, bool);

        #[test]
        fn test_tuple_struct_with_three_fields() {
            $test_fn(TupleStructWithThreeFields(
                "tomfoolery".to_owned(),
                42,
                false,
            ));
        }

        #$derive_clause
        struct NewType(String);

        #[test]
        fn test_new_type() {
            $test_fn(NewType(
                "tomfoolery".to_owned(),
            ));
        }

        #$derive_clause
        struct TupleStructWithGenerics<T, U>(
            Box<Box<Box<T>>>,
            Vec<U>,
        );

        #[test]
        fn test_tuple_struct_with_generics() {
            $test_fn(TupleStructWithGenerics(
                Box::new(Box::new(Box::new(
                    TupleStructWithNoFields(),
                ))),
                vec! [
                    TupleStructWithNoFields(),
                    TupleStructWithNoFields(),
                    TupleStructWithNoFields(),
                ],
            ));
        }

        #$derive_clause
        struct NewTypeWithGenerics<T>(Box<Box<Box<T>>>);

        #[test]
        fn test_new_type_with_generics() {
            $test_fn(NewTypeWithGenerics(
                Box::new(Box::new(Box::new(
                    TupleStructWithNoFields(),
                )))
            ));
        }
    };
}

macro_rules! __test_common_unit_structs {
    ($derive_clause:tt, $test_fn:ident) => {
        #$derive_clause
        struct UnitStruct;

        #[test]
        fn test_unit_struct() {
            $test_fn(UnitStruct);
        }
    };
}

macro_rules! __test_common_structs {
    ($derive_clause:tt, $test_fn:ident) => {
        crate::utils::templates::common_combinations::__test_common_ordinary_structs!($derive_clause, $test_fn);
        crate::utils::templates::common_combinations::__test_common_tuple_structs!($derive_clause, $test_fn);
        crate::utils::templates::common_combinations::__test_common_unit_structs!($derive_clause, $test_fn);
    };
}

macro_rules! __test_empty_enum {
    ($derive_clause:tt) => {
        #[allow(dead_code)]
        // Skip tests, as there is no way to
        // instantiate an `EmptyEnum`
        #$derive_clause
        enum EmptyEnum {}
    };
}

macro_rules! __test_common_enums_with_only_struct_variants {
    ($derive_clause:tt, $test_fn:ident) => {
        #$derive_clause
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
            $test_fn(EnumWithOnlyStructVariants::Quid {
                text: "tomfoolery".to_owned(),
            });
            $test_fn(EnumWithOnlyStructVariants::Pro {
                number: 42,
            });
            $test_fn(EnumWithOnlyStructVariants::Quo {
                flag: false,
            });
        }
    };
}

macro_rules! __test_common_enums_with_only_tuple_variants {
    ($derive_clause:tt, $test_fn:ident) => {
        #$derive_clause
        enum EnumWithOnlyTupleVariants {
            Quid(String),
            Pro(u64),
            Quo(bool),
        }

        #[test]
        fn test_enum_with_only_tuple_variants() {
            $test_fn(EnumWithOnlyTupleVariants::Quid(
                "tomfoolery".to_owned(),
            ));
            $test_fn(EnumWithOnlyTupleVariants::Pro(
                42,
            ));
            $test_fn(EnumWithOnlyTupleVariants::Quo(
                false,
            ));
        }
    };
}

macro_rules! __test_common_enums_with_only_unit_variants {
    ($derive_clause:tt, $test_fn:ident) => {
        #$derive_clause
        enum EnumWithOnlyUnitVariants {
            Quid,
            Pro,
            Quo,
        }

        #[test]
        fn test_enum_with_only_unit_variants() {
            $test_fn(EnumWithOnlyUnitVariants::Quid);
            $test_fn(EnumWithOnlyUnitVariants::Pro);
            $test_fn(EnumWithOnlyUnitVariants::Quo);
        }
    };
}

macro_rules! __test_common_enums_with_mixed_variants {
    ($derive_clause:tt, $test_fn:ident) => {
        #$derive_clause
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
            $test_fn(EnumWithMixedVariants::Quid {
                text: "tomfoolery".to_owned(),
                number: 42,
                flag: false,
            });
            $test_fn(EnumWithMixedVariants::Pro(
                "tomfoolery".to_owned(),
                42,
                false,
            ));
            $test_fn(EnumWithMixedVariants::Quo);
        }

        #$derive_clause
        enum EnumWithMixedVariantsAndGenerics<T, U> {
            Quid {
                pointer: Box<Box<Box<T>>>,
                vector: Vec<U>,
            },
            Pro(
                Box<Box<Box<T>>>,
                Vec<U>,
            ),
            Quo,
        }

        #[test]
        fn test_enum_with_mixed_variants_and_generics() {
            $test_fn(EnumWithMixedVariantsAndGenerics::Quid {
                pointer: Box::new(Box::new(Box::new(
                    EnumWithMixedVariants::Quo,
                ))),
                vector: vec! [
                    EnumWithMixedVariants::Quo,
                    EnumWithMixedVariants::Quo,
                    EnumWithMixedVariants::Quo,
                ],
            });
            $test_fn(EnumWithMixedVariantsAndGenerics::Pro(
                Box::new(Box::new(Box::new(
                    EnumWithMixedVariants::Quo,
                ))),
                vec! [
                    EnumWithMixedVariants::Quo,
                    EnumWithMixedVariants::Quo,
                    EnumWithMixedVariants::Quo,
                ],
            ));
            $test_fn(EnumWithMixedVariantsAndGenerics::<
                EnumWithMixedVariants,
                EnumWithMixedVariants,
            >::Quo);
        }
    };
}

macro_rules! __test_common_enums {
    ($derive_clause:tt, $test_fn:ident) => {
        crate::utils::templates::common_combinations::__test_empty_enum!($derive_clause);
        crate::utils::templates::common_combinations::__test_common_enums_with_only_struct_variants!($derive_clause, $test_fn);
        crate::utils::templates::common_combinations::__test_common_enums_with_only_tuple_variants!($derive_clause, $test_fn);
        crate::utils::templates::common_combinations::__test_common_enums_with_only_unit_variants!($derive_clause, $test_fn);
        crate::utils::templates::common_combinations::__test_common_enums_with_mixed_variants!($derive_clause, $test_fn);
    };
}

macro_rules! test_common_combinations {
    (
        derives = #$derive_clause:tt,
        test_fn = $test_fn:ident,
    ) => {
        crate::utils::templates::common_combinations::__test_common_structs!($derive_clause, $test_fn);
        crate::utils::templates::common_combinations::__test_common_enums!($derive_clause, $test_fn);
    };
}

pub(crate) use test_common_combinations;

pub(crate) use __test_common_structs;
pub(crate) use __test_common_enums;

pub(crate) use __test_common_ordinary_structs;
pub(crate) use __test_common_tuple_structs;
pub(crate) use __test_common_unit_structs;

pub(crate) use __test_empty_enum;
pub(crate) use __test_common_enums_with_only_struct_variants;
pub(crate) use __test_common_enums_with_only_tuple_variants;
pub(crate) use __test_common_enums_with_only_unit_variants;
pub(crate) use __test_common_enums_with_mixed_variants;
