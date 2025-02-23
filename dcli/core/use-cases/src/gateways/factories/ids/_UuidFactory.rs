use domain::utils::dataclasses::ids::Uuid;

pub trait UuidFactory {
    fn newUuid(&self) -> Uuid;
}
