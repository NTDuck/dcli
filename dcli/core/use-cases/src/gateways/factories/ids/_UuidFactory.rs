use domain::ids::Uuid;

pub trait UuidFactory {
    fn new_uuid(&self) -> Uuid;
}
