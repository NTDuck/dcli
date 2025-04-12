use std::ops::Deref;
use std::ops::DerefMut;

use axiom_derive::NewType;

#[derive(NewType)]
struct IntegerWrapper(i32);

#[test]
fn test_integer_wrapper() { verify_trait_bounds(IntegerWrapper(42)); }

#[derive(NewType)]
struct StringWrapper(String);

#[test]
fn test_string_wrapper() {
    verify_trait_bounds(StringWrapper("tomfoolery".to_owned()));
}

#[derive(NewType)]
struct VectorWrapper<T>(Vec<T>);

#[test]
fn test_vector_wrapper() {
    verify_trait_bounds(VectorWrapper(vec![0, 1, 2, 3, 4]));
}

#[derive(NewType)]
struct StringSliceWrapper<'br>(&'br str);

#[test]
fn test_string_slice_wrapper() {
    verify_trait_bounds(StringSliceWrapper("tomfoolery"));
}

#[allow(clippy::implied_bounds_in_impls)]
fn verify_trait_bounds(_: impl Deref + DerefMut) {}
