use domain::ids::Uuid;

pub trait UuidGenerator {
    fn generate(&self) -> Uuid;
}
