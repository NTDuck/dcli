use domain::ids::Uuid;

pub trait UuidFactory {
    fn generate(&self) -> Uuid;
}
