use axiom::interfaces::ddd::domain::Entity;
use axiom_derive::Entity;
use axiom_derive::Identifier;

#[derive(Entity)]
struct StructWithNamedFields {
    #[axiom(attributes(ddd::Identifier))]
    id: Uuid,
    text: String,
    number: u64,
    flag: bool,
}

#[test]
fn test_struct_with_named_fields() {
    verify_trait_bounds(StructWithNamedFields {
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
fn test_struct_with_named_fields_and_bounded_generics() {
    let struct_with_named_fields = StructWithNamedFields {
        id: Uuid(42),
        text: "tomfoolery".to_owned(),
        number: 42,
        flag: false,
    };

    verify_trait_bounds(StructWithNamedFieldsAndBoundedGenerics {
        id: Uuid(42),
        pointer: Box::new(struct_with_named_fields.clone()),
        vector: vec![struct_with_named_fields.clone()],
    });
}

#[derive(Identifier)]
struct Uuid(u128);

// Allow `Entity` usage
// without adding `axiom` as a dependency
pub mod axiom {
    pub mod interfaces {
        pub mod ddd {
            pub mod domain {
                use std::fmt::Debug;
                use std::hash::Hash;

                pub trait Entity: ValueObject {
                    type Identifier: Identifier;

                    fn get_id(&self) -> &Self::Identifier;
                }

                #[allow(dead_code)]
                pub trait Identifier: ValueObject + Hash {}

                pub trait ValueObject: Debug + Send + Sync + Clone + PartialEq + Eq {}
            }
        }
    }
}

fn verify_trait_bounds(_: impl Entity) {}
