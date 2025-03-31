use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;

use use_cases::gateways::pointers::PointerHandle;

use crate::pointers::handles::PointerStrategy;

pub struct PointerHandleWithStrategy<Strategy: PointerStrategy> {
    untyped: ManuallyDrop<Strategy::Untyped>,
}

unsafe impl<Strategy> PointerHandle for PointerHandleWithStrategy<Strategy>
where
    Strategy: PointerStrategy,
{
    fn new<T>(obj: T) -> Self {
        let typed = Strategy::into_typed(obj);
        let untyped = Strategy::into_untyped(typed);
        return Self::new_from_untyped(untyped);
    }
    
    fn as_ref<'br, T: 'br>(&'br self) -> impl Deref<Target = T> + 'br {
        return Strategy::as_ref(self.as_typed_ref());
    }
    
    fn as_mut<'br, T: 'br>(&'br self) -> impl DerefMut<Target = T> + 'br {
        return Strategy::as_mut(self.as_typed_ref());
    }

    unsafe fn shallow_clone<T>(&self) -> Self {
        let typed = Strategy::shallow_clone::<T>(self.as_typed_ref());
        let untyped = Strategy::into_untyped(typed);
        return Self::new_from_untyped(untyped);
    }

    unsafe fn drop<T>(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.as_typed_mut::<T>());
        }
    }
}

impl<Strategy> PointerHandleWithStrategy<Strategy>
where
    Strategy: PointerStrategy,
{
    fn new_from_untyped(untyped: Strategy::Untyped) -> Self {
        return Self {
            untyped: ManuallyDrop::new(untyped),
        };
    }

    fn as_typed_ref<T>(&self) -> &Strategy::Typed<T> {
        let raw_const_pointer = (self.untyped.deref() as *const Strategy::Untyped)
            .cast::<Strategy::Typed<T>>();

        Strategy::check_binary_compatibility::<T>();

        return unsafe {
            &*raw_const_pointer
        };
    }

    fn as_typed_mut<T>(&mut self) -> &mut Strategy::Typed<T> {
        let raw_mut_pointer = (self.untyped.deref_mut() as *mut Strategy::Untyped)
            .cast::<Strategy::Typed<T>>();

        Strategy::check_binary_compatibility::<T>();

        return unsafe {
            &mut *raw_mut_pointer
        };
    }
}
