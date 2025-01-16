#![allow(non_snake_case)]

/// # TODO
/// - Add support for `auto` and `unsafe auto` traits
/// - Add support for generics
#[macro_export]
macro_rules! trait_alias {
    ($visibility:vis trait $trait:ident: $($bound:ident),*) => {
        $visibility trait $trait: $( $bound + )* {}

        impl<T> $trait for T
        where T: $( $bound + )* {}
    };

    ($visibility:vis unsafe trait $trait:ident: $($bound:ident),*) => {
        $visibility unsafe trait $trait: $( $bound + )* {}

        unsafe impl<T> $trait for T
        where T: $( $bound + )* {}
    };
}
