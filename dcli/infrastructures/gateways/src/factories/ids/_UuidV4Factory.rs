use domain::utils::dataclasses::ids::Uuid;
use use_cases::gateways::factories::ids::UuidFactory;

pub struct UuidV4Factory;

impl UuidV4Factory {
    pub const fn new() -> Self {
        return Self;
    }
}

impl UuidFactory for UuidV4Factory {
    fn generate(&self) -> Uuid {
        let uuid_v4 = uuid::Uuid::new_v4();
        let uuid_as_u128 = uuid_v4.as_u128();
        return Uuid::from(uuid_as_u128);
    }
}
