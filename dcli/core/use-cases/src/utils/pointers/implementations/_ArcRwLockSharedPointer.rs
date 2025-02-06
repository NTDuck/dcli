use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;

pub struct ArcRwLockSharedPointer<T: ?Sized>(Arc<RwLock<T>>);

#[allow(dead_code)]
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

impl<T> Clone for ArcRwLockSharedPointer<T> {
    fn clone(&self) -> Self {
        return Self(Arc::clone(&self.0));
    }
}