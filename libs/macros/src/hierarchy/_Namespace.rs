/// Defines a namespace-like module.
///
/// Namespace-like modules are directory-based,
/// follow snake_case naming conventions,
/// and contain no type declarations.
///
/// Example: `my_namespace/mod.rs`
///
/// A namespace-like module can contain one or more class-like modules and/or namespace-like modules.
#[macro_export]
macro_rules! namespace_mod {
    (pub $module_name:ident) => {
        pub mod $module_name;
    };

    ($module_name:ident) => {
        mod $module_name;
    };
}

#[allow(unused_imports)]
pub(crate) use namespace_mod;
