use modules::*;

namespace!(pub domain);

pub use ddd_derive::*;
pub use domain::*;

#[derive(ddd_derive::ValueObject)]
struct Foo {
    size: usize,
    flag: bool,
}