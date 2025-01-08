use common::define_procedural_macro;

#[allow(non_snake_case)]
mod _Identifier;

use _Identifier::impl_identifier;

define_procedural_macro!(Identifier, impl_identifier);
