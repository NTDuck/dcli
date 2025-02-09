use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;

use use_cases::gateways::pointers::handles::abc::PointerHandle;

use crate::pointers::strategies::abc::PointerStrategy;

pub struct StrategizedPointerHandle<Strategy: PointerStrategy> {
    untyped: ManuallyDrop<Strategy::Untyped>,
}

unsafe impl<Strategy> PointerHandle for StrategizedPointerHandle<Strategy>
where
    Strategy: PointerStrategy,
{
    fn new<T>(obj: T) -> Self {
        let typed = Strategy::into_typed(obj);
        let untyped = Strategy::into_untyped(typed);
        return Self::new_from_untyped(untyped);
    }
    
    fn unwrap<'br, T: 'br>(&'br self) -> impl Deref<Target = T> + 'br {
        return Strategy::unwrap(self.as_ref());
    }
    
    fn unwrap_mut<'br, T: 'br>(&'br self) -> impl DerefMut<Target = T> + 'br {
        return Strategy::unwrap_mut(self.as_ref());
    }

    unsafe fn shallow_clone<T>(&self) -> Self {
        let typed = Strategy::shallow_clone::<T>(self.as_ref());
        let untyped = Strategy::into_untyped(typed);
        return Self::new_from_untyped(untyped);
    }

    unsafe fn drop<T>(&mut self) {
        std::ptr::drop_in_place(self.as_mut::<T>());
    }
}

impl<Strategy> StrategizedPointerHandle<Strategy>
where
    Strategy: PointerStrategy,
{
    fn new_from_untyped(untyped: Strategy::Untyped) -> Self {
        return Self {
            untyped: ManuallyDrop::new(untyped),
        };
    }

    fn as_ref<T>(&self) -> &Strategy::Typed<T> {
        let raw_const_ptr = (self.untyped.deref() as *const Strategy::Untyped)
            .cast::<Strategy::Typed<T>>();

        Strategy::static_binary_compatibility_check::<T>();

        return unsafe {
            &*raw_const_ptr
        };
    }

    fn as_mut<T>(&mut self) -> &mut Strategy::Typed<T> {
        let raw_mut_ptr = (self.untyped.deref_mut() as *mut Strategy::Untyped)
            .cast::<Strategy::Typed<T>>();

        Strategy::static_binary_compatibility_check::<T>();

        return unsafe {
            &mut *raw_mut_ptr
        };
    }
}
