use domain::ids::Uuid;

pub trait UuidFactory {
    fn create(&self) -> Uuid;
}
