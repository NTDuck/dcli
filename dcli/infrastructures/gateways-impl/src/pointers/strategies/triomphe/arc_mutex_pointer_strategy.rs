use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Mutex;

use triomphe::Arc;

use crate::pointers::handles::PointerStrategy;

pub struct ArcMutexPointerStrategy;

impl PointerStrategy for ArcMutexPointerStrategy {
    type Typed<T> = Arc<Mutex<T>>;
    type Untyped = Self::Typed<()>;

    fn into_typed<T>(obj: T) -> Self::Typed<T> {
        Arc::new(Mutex::new(obj))
    }

    fn into_untyped<T>(typed: Self::Typed<T>) -> Self::Untyped {
        unsafe {
            std::mem::transmute::<Self::Typed<T>, Self::Untyped>(typed)
        }
    }

    fn shallow_clone<T>(typed: &Self::Typed<T>) -> Self::Typed<T> {
        Arc::clone(typed)
    }

    fn as_ref<'br, T: 'br>(
        typed: &'br Self::Typed<T>,
    ) -> impl Deref<Target = T> + 'br {
        typed.lock().unwrap()
    }

    fn as_mut<'br, T: 'br>(
        typed: &'br Self::Typed<T>,
    ) -> impl DerefMut<Target = T> + 'br {
        typed.lock().unwrap()
    }
}
