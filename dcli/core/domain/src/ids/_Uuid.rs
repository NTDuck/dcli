#[derive(Clone, Copy)]
pub struct Uuid(u128);

impl From<u128> for Uuid {
    fn from(uuid: u128) -> Self {
        return Self(uuid);
    }
}
