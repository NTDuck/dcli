/// Defines a namespace-like module.
///
/// Namespace-like modules are directory-based,
/// follow snake_case naming conventions,
/// and contain no type declarations.
///
/// Example: `my_namespace/mod.rs`
///
/// A namespace-like module can contain one or more class-like modules and/or
/// namespace-like modules.
#[macro_export]
macro_rules! namespace {
    ($visibility:vis $module:ident) => {
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        $visibility mod $module;
    };
}
