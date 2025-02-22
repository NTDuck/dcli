use axiom::interfaces::ddd::domain::ValueObject;
use axiom_derive::ValueObject;

use crate::utils::*;

testCommonStructsAndEnums!{
    (ValueObject),
    verifyTraitBounds
}

// Allow `ValueObject` usage
// without adding `axiom` as a dependency
pub mod axiom {
    pub mod interfaces {
        pub mod ddd {
            pub mod domain {
                use std::fmt::Debug;

                pub trait ValueObject: Debug + Send + Sync + Clone + PartialEq + Eq {}
            }
        }
    }
}

fn verifyTraitBounds(_: impl ValueObject) {}
