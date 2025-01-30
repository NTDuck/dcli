use domain::Uuid;
use use_cases::dataproviders::generators::ids::UuidGenerator;

pub struct UuidV4Generator {}

impl UuidGenerator for UuidV4Generator {
    fn generate(&self) -> Uuid {
        let uuid_v4 = uuid::Uuid::new_v4();
        let uuid_as_u128 = uuid_v4.as_u128();
        return Uuid::from(uuid_as_u128);
    }
}
