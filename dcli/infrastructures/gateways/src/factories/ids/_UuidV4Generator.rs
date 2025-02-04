use domain::Uuid;
use types::New;
use use_cases::gateways::factories::ids::UuidFactory;

#[derive(New)]
pub struct UuidV4Factory;

impl UuidFactory for UuidV4Factory {
    fn generate(&self) -> Uuid {
        let uuid_v4 = uuid::Uuid::new_v4();
        let uuid_as_u128 = uuid_v4.as_u128();
        return Uuid::from(uuid_as_u128);
    }
}
