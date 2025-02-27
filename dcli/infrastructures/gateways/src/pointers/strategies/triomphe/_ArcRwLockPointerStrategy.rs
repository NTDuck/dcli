use std::ops::Deref;
use std::ops::DerefMut;
use triomphe::Arc;
use std::sync::RwLock;

use crate::pointers::handles::PointerStrategy;

pub struct ArcRwLockSharedPointerStrategy;

impl PointerStrategy for ArcRwLockSharedPointerStrategy {
    type Typed<T> = Arc<RwLock<T>>;
    type Untyped = Self::Typed<()>;

    fn into_typed<T>(obj: T) -> Self::Typed<T> {
        return Arc::new(RwLock::new(obj));
    }

    fn into_untyped<T>(typed: Self::Typed<T>) -> Self::Untyped {
        return unsafe {
            std::mem::transmute::<Self::Typed<T>, Self::Untyped>(typed)
        };
    }

    fn shallow_clone<T>(typed: &Self::Typed<T>) -> Self::Typed<T> {
        return Arc::clone(typed);
    }

    fn as_ref<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl Deref<Target = T> + 'br {
        return typed.read().unwrap();
    }

    fn as_mut<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl DerefMut<Target = T> + 'br {
        return typed.write().unwrap();
    }
}