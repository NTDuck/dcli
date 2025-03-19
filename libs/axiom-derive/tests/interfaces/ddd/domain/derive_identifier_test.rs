use axiom::interfaces::ddd::domain::Identifier;
use axiom_derive::Identifier;

use crate::utils::templates::common_combinations::*;

test_common_combinations!{
    (Identifier),
    verify_trait_bounds
}

// Allow `Identifier` usage
// without adding `axiom` as a dependency
pub mod axiom {
    pub mod interfaces {
        pub mod ddd {
            pub mod domain {
                use std::fmt::Debug;
                use std::hash::Hash;

                pub trait Identifier: ValueObject + Hash {}

                pub trait ValueObject: Debug + Send + Sync + Clone + PartialEq + Eq {}
            }
        }
    }
}

fn verify_trait_bounds(_: impl Identifier) {}
