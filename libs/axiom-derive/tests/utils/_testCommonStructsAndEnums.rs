macro_rules! testCommonOrdinaryStructs {
    ($derivedAttrs:tt, $testFn:ident) => {
        #[derive $derivedAttrs]
        struct OrdinaryStructWithNoFields {}

        #[test]
        fn testOrdinaryStructWithNoFields() {
            $testFn(OrdinaryStructWithNoFields {});
        }

        #[derive $derivedAttrs]
        struct OrdinaryStructWithThreeFields {
            text: String,
            number: u64,
            flag: bool,
        }

        #[test]
        fn testOrdinaryStructWithThreeFields() {
            $testFn(OrdinaryStructWithThreeFields {
                text: "tomfoolery".to_owned(),
                number: 42,
                flag: false,
            });
        }

        #[derive $derivedAttrs]
        struct OrdinaryStructWithGenerics<T, U> {
            pointer: Box<Box<Box<T>>>,
            vector: Vec<U>,
        }

        #[test]
        fn testOrdinaryStructWithGenerics() {
            $testFn(OrdinaryStructWithGenerics {
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

macro_rules! testCommonTupleStructs {
    ($derivedAttrs:tt, $testFn:ident) => {
        #[derive $derivedAttrs]
        struct TupleStructWithNoFields();

        #[test]
        fn testTupleStructWithNoFields() {
            $testFn(TupleStructWithNoFields());
        }

        #[derive $derivedAttrs]
        struct TupleStructWithThreeFields(String, u64, bool);

        #[test]
        fn testTupleStructWithThreeFields() {
            $testFn(TupleStructWithThreeFields(
                "tomfoolery".to_owned(),
                42,
                false,
            ));
        }

        #[derive $derivedAttrs]
        struct NewType(String);

        #[test]
        fn testNewType() {
            $testFn(NewType(
                "tomfoolery".to_owned(),
            ));
        }

        #[derive $derivedAttrs]
        struct TupleStructWithGenerics<T, U>(
            Box<Box<Box<T>>>,
            Vec<U>,
        );

        #[test]
        fn testTupleStructWithGenerics() {
            $testFn(TupleStructWithGenerics(
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

        #[derive $derivedAttrs]
        struct NewTypeWithGenerics<T>(Box<Box<Box<T>>>);

        #[test]
        fn testNewTypeWithGenerics() {
            $testFn(NewTypeWithGenerics(
                Box::new(Box::new(Box::new(
                    TupleStructWithNoFields(),
                )))
            ));
        }
    };
}

macro_rules! testCommonUnitStructs {
    ($derivedAttrs:tt, $testFn:ident) => {
        #[derive $derivedAttrs]
        struct UnitStruct;

        #[test]
        fn testUnitStruct() {
            $testFn(UnitStruct);
        }
    };
}

macro_rules! testCommonStructs {
    ($derivedAttrs:tt, $testFn:ident) => {
        testCommonOrdinaryStructs!($derivedAttrs, $testFn);
        testCommonTupleStructs!($derivedAttrs, $testFn);
        testCommonUnitStructs!($derivedAttrs, $testFn);
    };
}

macro_rules! testEmptyEnum {
    ($derivedAttrs:tt) => {
        // Skip tests, as there is no way to
        // instantiate an `EmptyEnum`
        #[allow(dead_code)]
        #[derive $derivedAttrs]
        enum EmptyEnum {}
    };
}

macro_rules! testCommonEnumsWithOnlyStructVariants {
    ($derivedAttrs:tt, $testFn:ident) => {
        #[derive $derivedAttrs]
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
            $testFn(EnumWithOnlyStructVariants::Quid {
                text: "tomfoolery".to_owned(),
            });
            $testFn(EnumWithOnlyStructVariants::Pro {
                number: 42,
            });
            $testFn(EnumWithOnlyStructVariants::Quo {
                flag: false,
            });
        }
    };
}

macro_rules! testCommonEnumsWithOnlyTupleVariants {
    ($derivedAttrs:tt, $testFn:ident) => {
        #[derive $derivedAttrs]
        enum EnumWithOnlyTupleVariants {
            Quid(String),
            Pro(u64),
            Quo(bool),
        }

        #[test]
        fn testEnumWithOnlyTupleVariants() {
            $testFn(EnumWithOnlyTupleVariants::Quid(
                "tomfoolery".to_owned(),
            ));
            $testFn(EnumWithOnlyTupleVariants::Pro(
                42,
            ));
            $testFn(EnumWithOnlyTupleVariants::Quo(
                false,
            ));
        }
    };
}

macro_rules! testCommonEnumsWithOnlyUnitVariants {
    ($derivedAttrs:tt, $testFn:ident) => {
        #[derive $derivedAttrs]
        enum EnumWithOnlyUnitVariants {
            Quid,
            Pro,
            Quo,
        }

        #[test]
        fn testEnumWithOnlyUnitVariants() {
            $testFn(EnumWithOnlyUnitVariants::Quid);
            $testFn(EnumWithOnlyUnitVariants::Pro);
            $testFn(EnumWithOnlyUnitVariants::Quo);
        }
    };
}

macro_rules! testCommonEnumsWithMixedVariants {
    ($derivedAttrs:tt, $testFn:ident) => {
        #[derive $derivedAttrs]
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
            $testFn(EnumWithMixedVariants::Quid {
                text: "tomfoolery".to_owned(),
                number: 42,
                flag: false,
            });
            $testFn(EnumWithMixedVariants::Pro(
                "tomfoolery".to_owned(),
                42,
                false,
            ));
            $testFn(EnumWithMixedVariants::Quo);
        }

        #[derive $derivedAttrs]
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
        fn testEnumWithMixedVariantsAndGenerics() {
            $testFn(EnumWithMixedVariantsAndGenerics::Quid {
                pointer: Box::new(Box::new(Box::new(
                    EnumWithMixedVariants::Quo,
                ))),
                vector: vec! [
                    EnumWithMixedVariants::Quo,
                    EnumWithMixedVariants::Quo,
                    EnumWithMixedVariants::Quo,
                ],
            });
            $testFn(EnumWithMixedVariantsAndGenerics::Pro(
                Box::new(Box::new(Box::new(
                    EnumWithMixedVariants::Quo,
                ))),
                vec! [
                    EnumWithMixedVariants::Quo,
                    EnumWithMixedVariants::Quo,
                    EnumWithMixedVariants::Quo,
                ],
            ));
            $testFn(EnumWithMixedVariantsAndGenerics::<
                EnumWithMixedVariants,
                EnumWithMixedVariants,
            >::Quo);
        }
    };
}

macro_rules! testCommonEnums {
    ($derivedAttrs:tt, $testFn:ident) => {
        testEmptyEnum!($derivedAttrs);
        testCommonEnumsWithOnlyStructVariants!($derivedAttrs, $testFn);
        testCommonEnumsWithOnlyTupleVariants!($derivedAttrs, $testFn);
        testCommonEnumsWithOnlyUnitVariants!($derivedAttrs, $testFn);
        testCommonEnumsWithMixedVariants!($derivedAttrs, $testFn);
    };
}

macro_rules! testCommonStructsAndEnums {
    ($derivedAttrs:tt, $testFn:ident) => {
        testCommonStructs!($derivedAttrs, $testFn);
        testCommonEnums!($derivedAttrs, $testFn);
    };
}

pub(crate) use testCommonStructsAndEnums;

pub(crate) use testCommonStructs;
pub(crate) use testCommonEnums;

pub(crate) use testCommonOrdinaryStructs;
pub(crate) use testCommonTupleStructs;
pub(crate) use testCommonUnitStructs;

pub(crate) use testEmptyEnum;
pub(crate) use testCommonEnumsWithOnlyStructVariants;
pub(crate) use testCommonEnumsWithOnlyTupleVariants;
pub(crate) use testCommonEnumsWithOnlyUnitVariants;
pub(crate) use testCommonEnumsWithMixedVariants;
