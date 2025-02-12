use domain::utils::dataclasses::ids::Uuid;

pub trait UuidFactory {
    fn generate(&self) -> Uuid;
}
