use std::ops::Deref;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uuid(u128);

impl Deref for Uuid {
    type Target = u128;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl From<u128> for Uuid {
    fn from(uuid: u128) -> Self {
        return Self(uuid);
    }
}
