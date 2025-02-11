#[allow(non_snake_case)]

#[macro_export]
macro_rules! test_group {
    ($module:ident) => {
        #[allow(non_snake_case)]
        mod $module;
    };
}
