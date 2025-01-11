/// Defines a class-like module.
///
/// Class-like modules are file-based,
/// follow `_` + PascalCase naming conventions,
/// and may contain one or a few tightly-coupled type declaration(s).
///
/// Example: `_MyClass.rs`
/// 
/// See: [The LoB Principle](https://htmx.org/essays/locality-of-behaviour/)
///
/// A class-like module belongs to a namespace-like module.
#[macro_export]
macro_rules! class_mod {
    ($module_name:ident) => {
        #[allow(non_snake_case)]
        mod $module_name;
        
        #[allow(unused_imports)]
        pub use $module_name::*;
    };
}

#[allow(unused_imports)]
pub(crate) use class_mod;
