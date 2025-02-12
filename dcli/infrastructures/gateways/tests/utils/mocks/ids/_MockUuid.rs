use domain::utils::dataclasses::ids::Uuid;
use fake::Dummy;
use fake::Faker;
use fake::Fake;

pub fn mockUuid() -> Uuid {
    return Faker.fake::<MockUuid>()
        .into();
}

#[derive(Dummy)]
pub struct MockUuid(u128);

impl Into<Uuid> for MockUuid {
    fn into(self) -> Uuid {
        return Uuid::from(self.0);
    }
}
