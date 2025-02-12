#[macro_export]
macro_rules! testGroup {
    ($module:ident) => {
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        mod $module;
    };
}
