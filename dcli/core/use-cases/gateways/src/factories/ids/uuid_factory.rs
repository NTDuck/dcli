use domain::ids::Uuid;

pub trait UuidFactory: Send + Sync {
    fn create(&self) -> Uuid;
}
