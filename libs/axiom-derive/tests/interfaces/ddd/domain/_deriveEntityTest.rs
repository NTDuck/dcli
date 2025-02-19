use axiom_derive::Entity;
use axiom_derive::Identifier;
use axiom_derive::NewType;

// Allow `Entity` usage
// without adding `axiom` as a dependency
pub mod axiom {
    pub mod interfaces {
        pub mod ddd {
            pub mod domain {
                #![allow(dead_code)]

                use std::fmt::Debug;
                use std::hash::Hash;

                pub trait Entity: ValueObject {
                    type Id: Identifier;

                    fn getId(&self) -> &Self::Id;
                }

                pub trait Identifier: ValueObject + Hash {}
                pub trait ValueObject: Debug + Send + Sync + Clone + PartialEq + Eq {}
            }
        }
    }
}

#[derive(Identifier, NewType)]
struct Uuid(u128);

impl From<u128> for Uuid {
    fn from(arg0: u128) -> Self {
        return Self(arg0);
    }
}

#[allow(dead_code)]
#[derive(Entity)]
struct StructWithNamedFields {
    #[axiom(attributes(Identifier))]
    id: Uuid,
    text: String,
    number: u64,
    flag: bool,
}

#[test]
fn testStructWithNamedFields() {

}
