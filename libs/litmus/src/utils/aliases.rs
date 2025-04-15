use std::borrow::Cow;

pub type MaybeOwnedStr = Cow<'static, str>;

#[cfg(feature = "triomphe")]
pub type Arc<T> = triomphe::Arc<T>;

#[cfg(not(feature = "triomphe"))]
pub type Arc<T> = std::sync::Arc<T>;
