use std::ops::Deref;
use std::ops::DerefMut;

use axiom_derive::NewType;

#[derive(NewType)]
struct IntegerWrapper(i32);

#[test]
fn testIntegerWrapper() {
    verifyTraitBounds(IntegerWrapper(42));
}

#[derive(NewType)]
struct StringWrapper(String);

#[test]
fn testStringWrapper() {
    verifyTraitBounds(StringWrapper("tomfoolery".to_owned()));
}

#[derive(NewType)]
struct VectorWrapper<T>(Vec<T>);

#[test]
fn testVectorWrapper() {
    verifyTraitBounds(VectorWrapper(vec![0, 1, 2, 3, 4]));
}

#[derive(NewType)]
struct StringSliceWrapper<'br>(&'br str);

#[test]
fn testStringSliceWrapper() {
    verifyTraitBounds(StringSliceWrapper("tomfoolery"));
}

fn verifyTraitBounds(_: impl Deref + DerefMut) {}
