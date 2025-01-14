#![allow(non_snake_case)]

/// # TODO
/// - Add support for `auto` and `unsafe auto` traits
/// - Add support for generics
#[macro_export]
macro_rules! trait_alias {
    ($visibility:vis trait $trait_:ident: $($bound:ident),*) => {
        $visibility trait $trait_: $( $bound + )* {}

        impl<T> $trait_ for T
        where T: $( $bound + )* {}
    };

    ($visibility:vis unsafe trait $trait_:ident: $($bound:ident),*) => {
        $visibility unsafe trait $trait_: $( $bound + )* {}

        unsafe impl<T> $trait_ for T
        where T: $( $bound + )* {}
    };
}
