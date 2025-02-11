#![allow(non_snake_case)]

#[macro_export]
macro_rules! test_suite {
    ($module:ident) => {
        #[allow(non_snake_case)]
        mod $module;
    };
}
