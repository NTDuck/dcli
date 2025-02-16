use std::ops::Deref;
use std::ops::DerefMut;
use triomphe::Arc;
use std::sync::Mutex;

use crate::pointers::strategies::abc::PointerStrategy;

pub struct ArcMutexPointerStrategy;

impl PointerStrategy for ArcMutexPointerStrategy {
    type Typed<T> = Arc<Mutex<T>>;
    type Untyped = Self::Typed<()>;

    fn intoTyped<T>(obj: T) -> Self::Typed<T> {
        return Arc::new(Mutex::new(obj));
    }

    fn intoUntyped<T>(typed: Self::Typed<T>) -> Self::Untyped {
        return unsafe {
            std::mem::transmute::<Self::Typed<T>, Self::Untyped>(typed)
        };
    }

    fn shallowClone<T>(typed: &Self::Typed<T>) -> Self::Typed<T> {
        return Arc::clone(typed);
    }

    fn read<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl Deref<Target = T> + 'br {
        return typed.lock().unwrap();
    }

    fn write<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl DerefMut<Target = T> + 'br {
        return typed.lock().unwrap();
    }
}