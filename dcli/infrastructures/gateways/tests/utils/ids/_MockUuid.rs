use domain::Uuid;
use fake::Dummy;

#[derive(Dummy)]
pub struct MockUuid(u128);

impl Into<Uuid> for MockUuid {
    fn into(self) -> Uuid {
        return Uuid::from(self.0);
    }
}
