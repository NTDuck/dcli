use domain::ids::Uuid;

pub trait UuidFactory {
    fn newUuid(&self) -> Uuid;
}
