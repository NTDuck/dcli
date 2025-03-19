macro_rules! test_common_ordinary_structs {
    ($derived_attrs:tt, $test_fn:ident) => {
        #[derive $derived_attrs]
        struct OrdinaryStructWithNoFields {}

        #[test]
        fn test_ordinary_struct_with_no_fields() {
            $test_fn(OrdinaryStructWithNoFields {});
        }

        #[derive $derived_attrs]
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

        #[derive $derived_attrs]
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

macro_rules! test_common_tuple_structs {
    ($derived_attrs:tt, $test_fn:ident) => {
        #[derive $derived_attrs]
        struct TupleStructWithNoFields();

        #[test]
        fn test_tuple_struct_with_no_fields() {
            $test_fn(TupleStructWithNoFields());
        }

        #[derive $derived_attrs]
        struct TupleStructWithThreeFields(String, u64, bool);

        #[test]
        fn test_tuple_struct_with_three_fields() {
            $test_fn(TupleStructWithThreeFields(
                "tomfoolery".to_owned(),
                42,
                false,
            ));
        }

        #[derive $derived_attrs]
        struct NewType(String);

        #[test]
        fn test_new_type() {
            $test_fn(NewType(
                "tomfoolery".to_owned(),
            ));
        }

        #[derive $derived_attrs]
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

        #[derive $derived_attrs]
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

macro_rules! test_common_unit_structs {
    ($derived_attrs:tt, $test_fn:ident) => {
        #[derive $derived_attrs]
        struct UnitStruct;

        #[test]
        fn test_unit_struct() {
            $test_fn(UnitStruct);
        }
    };
}

macro_rules! test_common_structs {
    ($derived_attrs:tt, $test_fn:ident) => {
        test_common_ordinary_structs!($derived_attrs, $test_fn);
        test_common_tuple_structs!($derived_attrs, $test_fn);
        test_common_unit_structs!($derived_attrs, $test_fn);
    };
}

macro_rules! test_empty_enum {
    ($derived_attrs:tt) => {
        // Skip tests, as there is no way to
        // instantiate an `EmptyEnum`
        #[allow(dead_code)]
        #[derive $derived_attrs]
        enum EmptyEnum {}
    };
}

macro_rules! test_common_enums_with_only_struct_variants {
    ($derived_attrs:tt, $test_fn:ident) => {
        #[derive $derived_attrs]
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

macro_rules! test_common_enums_with_only_tuple_variants {
    ($derived_attrs:tt, $test_fn:ident) => {
        #[derive $derived_attrs]
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

macro_rules! test_common_enums_with_only_unit_variants {
    ($derived_attrs:tt, $test_fn:ident) => {
        #[derive $derived_attrs]
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

macro_rules! test_common_enums_with_mixed_variants {
    ($derived_attrs:tt, $test_fn:ident) => {
        #[derive $derived_attrs]
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

        #[derive $derived_attrs]
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

macro_rules! test_common_enums {
    ($derived_attrs:tt, $test_fn:ident) => {
        test_empty_enum!($derived_attrs);
        test_common_enums_with_only_struct_variants!($derived_attrs, $test_fn);
        test_common_enums_with_only_tuple_variants!($derived_attrs, $test_fn);
        test_common_enums_with_only_unit_variants!($derived_attrs, $test_fn);
        test_common_enums_with_mixed_variants!($derived_attrs, $test_fn);
    };
}

macro_rules! test_common_combinations {
    ($derived_attrs:tt, $test_fn:ident) => {
        test_common_structs!($derived_attrs, $test_fn);
        test_common_enums!($derived_attrs, $test_fn);
    };
}

pub(crate) use test_common_combinations;

pub(crate) use test_common_structs;
pub(crate) use test_common_enums;

pub(crate) use test_common_ordinary_structs;
pub(crate) use test_common_tuple_structs;
pub(crate) use test_common_unit_structs;

pub(crate) use test_empty_enum;
pub(crate) use test_common_enums_with_only_struct_variants;
pub(crate) use test_common_enums_with_only_tuple_variants;
pub(crate) use test_common_enums_with_only_unit_variants;
pub(crate) use test_common_enums_with_mixed_variants;
