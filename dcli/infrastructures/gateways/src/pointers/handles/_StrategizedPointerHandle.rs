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
        let typed = Strategy::typed_from_obj(obj);
        let untyped = Strategy::into_untyped(typed);
        return Self::new_from_untyped(untyped);
    }
    
    fn unwrap<'br, T: 'br>(&'br self) -> impl Deref<Target = T> + 'br {
        return Strategy::unwrap(self.as_ref());
    }
    
    fn unwrap_mut<'br, T: 'br>(&'br self) -> impl DerefMut<Target = T> + 'br {
        return Strategy::unwrap_mut(self.as_ref());
    }

    unsafe fn shallow_copy<T>(&self) -> Self {
        let typed = Strategy::shallow_copy_from_typed::<T>(self.as_ref());
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
        let raw_ref = (self.untyped.deref() as *const Strategy::Untyped)
            .cast::<Strategy::Typed<T>>();

        Self::static_size_check::<T>();

        return unsafe {
            &*raw_ref
        };
    }

    fn as_mut<T>(&mut self) -> &mut Strategy::Typed<T> {
        let raw_mut = (self.untyped.deref_mut() as *mut Strategy::Untyped)
            .cast::<Strategy::Typed<T>>();

        Self::static_size_check::<T>();

        return unsafe {
            &mut *raw_mut
        };
    }

    fn static_size_check<T>() {
        std::hint::black_box(std::mem::transmute_copy::<Strategy::Untyped, Strategy::Typed<T>>);
    }
}
