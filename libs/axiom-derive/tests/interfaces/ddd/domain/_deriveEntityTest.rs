use axiom::interfaces::ddd::domain::Entity;
use axiom_derive::Entity;
use axiom_derive::Identifier;

// Allow `Entity` usage
// without adding `axiom` as a dependency
pub mod axiom {
    pub mod interfaces {
        pub mod ddd {
            pub mod domain {
                use std::fmt::Debug;
                use std::hash::Hash;

                pub trait Entity: ValueObject {
                    type Id: Identifier;

                    fn getId(&self) -> &Self::Id;
                }

                #[allow(dead_code)]
                pub trait Identifier: ValueObject + Hash {}

                pub trait ValueObject: Debug + Send + Sync + Clone + PartialEq + Eq {}
            }
        }
    }
}

#[derive(Entity)]
struct StructWithNamedFields {
    #[axiom(attributes(ddd::Identifier))]
    id: Uuid,
    text: String,
    number: u64,
    flag: bool,
}

#[test]
fn testStructWithNamedFields() {
    verifyTraitBounds(StructWithNamedFields {
        id: Uuid(42),
        text: "tomfoolery".to_owned(),
        number: 42,
        flag: false,
    });
}

// Requires trait bounds to compile,
// which is unprecedented
#[derive(Entity)]
struct StructWithNamedFieldsAndBoundedGenerics<T: Entity, U: Entity> {
    #[axiom(attributes(ddd::Identifier))]
    id: Uuid,
    pointer: Box<T>,
    vector: Vec<U>,
}

#[test]
fn testStructWithNamedFieldsAndBoundedGenerics() {
    let structWithNamedFields = StructWithNamedFields {
        id: Uuid(42),
        text: "tomfoolery".to_owned(),
        number: 42,
        flag: false,
    };

    verifyTraitBounds(StructWithNamedFieldsAndBoundedGenerics {
        id: Uuid(42),
        pointer: Box::new(structWithNamedFields.clone()),
        vector: vec![structWithNamedFields.clone()],
    });
}

#[derive(Identifier)]
struct Uuid(u128);

fn verifyTraitBounds(_: impl Entity) {}
