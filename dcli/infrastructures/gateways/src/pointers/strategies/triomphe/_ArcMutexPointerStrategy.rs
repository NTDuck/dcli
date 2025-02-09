use std::ops::Deref;
use std::ops::DerefMut;
use triomphe::Arc;
use std::sync::Mutex;

use crate::pointers::strategies::abc::PointerStrategy;

pub struct ArcMutexPointerStrategy;

impl PointerStrategy for ArcMutexPointerStrategy {
    type Typed<T> = Arc<Mutex<T>>;
    type Untyped = Self::Typed<()>;

    fn into_typed<T>(obj: T) -> Self::Typed<T> {
        return Arc::new(Mutex::new(obj));
    }

    fn into_untyped<T>(typed: Self::Typed<T>) -> Self::Untyped {
        return unsafe {
            std::mem::transmute(typed)
        };
    }

    fn shallow_clone<T>(typed: &Self::Typed<T>) -> Self::Typed<T> {
        return Arc::clone(typed);
    }

    fn unwrap<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl Deref<Target = T> + 'br {
        return typed.lock().unwrap();
    }

    fn unwrap_mut<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl DerefMut<Target = T> + 'br {
        return typed.lock().unwrap();
    }
}