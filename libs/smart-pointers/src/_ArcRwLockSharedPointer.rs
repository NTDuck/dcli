use std::marker::Unsize;
use std::ops::CoerceUnsized;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;

#[repr(transparent)]
pub struct ArcRwLockSharedPointer<T: ?Sized>(Arc<RwLockWrapper<T>>);

impl<T> ArcRwLockSharedPointer<T> {
    pub fn new(object: T) -> Self {
        return Self(Arc::new(RwLockWrapper(RwLock::new(object))));
    }

    pub fn unwrap(&self) -> impl Deref<Target = T> + '_ {
        return self.0.0.read().unwrap();
    }

    pub fn unwrap_mut(&self) -> impl DerefMut<Target = T> + '_ {
        return self.0.0.write().unwrap();
    }
}

impl<T, U> CoerceUnsized<ArcRwLockSharedPointer<U>> for ArcRwLockSharedPointer<T>
where
    T: Unsize<U>,
    Arc<RwLockWrapper<T>>: CoerceUnsized<Arc<RwLockWrapper<U>>>,
{}

#[repr(transparent)]
struct RwLockWrapper<T: ?Sized>(RwLock<T>);

impl<T, U> CoerceUnsized<RwLockWrapper<U>> for RwLockWrapper<T>
where
    T: Unsize<U>,
    RwLock<T>: CoerceUnsized<RwLock<U>>,
{}
