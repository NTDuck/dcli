use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;

use crate::utils::pointers::SharedPointerStrategy;

pub struct ArcRwLockShardPointerStrategy;

impl SharedPointerStrategy for ArcRwLockShardPointerStrategy {
    type Typed<T> = Arc<RwLock<T>>;
    type Untyped = Self::Typed<()>;

    fn typed_from_obj<T>(obj: T) -> Self::Typed<T> {
        return Arc::new(RwLock::new(obj));
    }

    fn shallow_copy_from_typed<T>(typed: &Self::Typed<T>) -> Self::Typed<T> {
        return Arc::clone(typed);
    }

    fn unwrap<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl Deref<Target = T> + 'br {
        return typed.read().unwrap();
    }

    fn unwrap_mut<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl DerefMut<Target = T> + 'br {
        return typed.write().unwrap();
    }

    // fn into_untyped<T>(typed: Self::Typed<T>) -> Self::Untyped {
    //     let raw_ref = Arc::into_raw(typed) as *const RwLock<()>;
    //     return unsafe {
    //         Arc::from_raw(raw_ref)
    //     };
    // }
}