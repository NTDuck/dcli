use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ArcMutexSharedPointer<T: ?Sized>(Arc<Mutex<T>>);

#[allow(dead_code)]
impl<T> ArcMutexSharedPointer<T> {
    pub fn new(object: T) -> Self {
        return Self(Arc::new(Mutex::new(object)));
    }

    pub fn unwrap(&self) -> impl Deref<Target = T> + '_ {
        return self.0.lock().unwrap();
    }

    pub fn unwrap_mut(&self) -> impl DerefMut<Target = T> + '_ {
        return self.0.lock().unwrap();
    }
}

impl<T> Clone for ArcMutexSharedPointer<T> {
    fn clone(&self) -> Self {
        return Self(Arc::clone(&self.0));
    }
}
