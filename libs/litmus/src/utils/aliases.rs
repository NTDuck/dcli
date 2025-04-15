use std::borrow::Cow;

pub type MaybeOwnedStr = Cow<'static, str>;

#[cfg(feature = "triomphe")]
pub type Arc<T> = triomphe::Arc<T>;

#[cfg(not(feature = "triomphe"))]
pub type Arc<T> = std::sync::Arc<T>;

#[cfg(feature = "ahash")]
pub type HashMap<K, V> = ahash::AHashMap<K, V>;

#[cfg(all(not(feature = "ahash"), feature = "fxhash"))]
pub type HashMap<K, V> = fxhash::FxHashMap<K, V>;

#[cfg(all(not(feature = "ahash"), not(feature = "fxhash")))]
pub type HashMap<K, V> = std::collections::HashMap<K, V>;
