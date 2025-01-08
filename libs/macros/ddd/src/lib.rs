#[allow(non_snake_case)]
mod utils;

#[allow(non_snake_case)]
mod _Identifier;

use _Identifier::impl_identifier;

utils::derive!(Identifier, impl_identifier);
