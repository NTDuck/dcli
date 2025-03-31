#[cfg(feature = "uuid")]
mod uuid_v4_factory;

#[cfg(feature = "uuid")]
pub use self::uuid_v4_factory::*;
