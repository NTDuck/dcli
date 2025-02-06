// use std::marker::Unsize;
// use std::ops::CoerceUnsized;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;

// #[repr(transparent)]
pub struct ArcRwLockSharedPointer<T: ?Sized>(Arc<RwLock<T>>);

impl<T> ArcRwLockSharedPointer<T> {
    pub fn new(object: T) -> Self {
        return Self(Arc::new(RwLock::new(object)));
    }

    pub fn unwrap(&self) -> impl Deref<Target = T> + '_ {
        return self.0.read().unwrap();
    }

    pub fn unwrap_mut(&self) -> impl DerefMut<Target = T> + '_ {
        return self.0.write().unwrap();
    }
}
