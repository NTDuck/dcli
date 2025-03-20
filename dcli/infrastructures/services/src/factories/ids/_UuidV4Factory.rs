use axiom::behaviours::New;
use domain::ids::Uuid;
use use_cases::gateways::factories::ids::UuidFactory;

#[derive(New)]
pub struct UuidV4Factory;

impl UuidFactory for UuidV4Factory {
    fn new_uuid(&self) -> Uuid {
        return Uuid::new(uuid::Uuid::new_v4().as_u128());
    }
}
