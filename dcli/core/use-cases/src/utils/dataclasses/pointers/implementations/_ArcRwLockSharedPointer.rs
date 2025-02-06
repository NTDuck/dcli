use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

pub struct ArcRwLockSharedPointer<T: ?Sized>(Arc<RwLock<T>>);

impl<T: Sized> ArcRwLockSharedPointer<T> {
    pub fn new(object: T) -> Self {
        return Self(Arc::new(RwLock::new(object)));
    }
}

impl<T: ?Sized> ArcRwLockSharedPointer<T> {
    pub fn unwrap(&self) -> RwLockReadGuard<'_, T> {
        return self.0.read().unwrap();
    }

    pub fn unwrap_mut(&self) -> RwLockWriteGuard<'_, T> {
        return self.0.write().unwrap();
    }
}

impl<T: ?Sized> Clone for ArcRwLockSharedPointer<T> {
    fn clone(&self) -> Self {
        return Self(Arc::clone(&self.0));
    }
}
