#![allow(non_snake_case)]

/// Defines a class-like module.
///
/// Class-like modules are file-based,
/// follow underscore-prefixed PascalCase naming conventions,
/// and may contain one or a few tightly-coupled type declaration(s).
/// Visibility of declared items is public within the containing namespace (i.e.
/// the parent namespace-like module).
///
/// Example: `_MyClass.rs`
///
/// See: [The LoB Principle](https://htmx.org/essays/locality-of-behaviour/)
///
/// A class-like module belongs to a namespace-like module.
#[macro_export]
macro_rules! class_mod {
    ($visibility:vis $module:ident) => {
        #[allow(non_snake_case)]
        mod $module;

        #[allow(unused_imports)]
        $visibility use $module::*;
    };
}
