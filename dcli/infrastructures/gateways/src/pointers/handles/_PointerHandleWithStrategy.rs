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
        let typed = Strategy::intoTyped(obj);
        let untyped = Strategy::intoUntyped(typed);
        return Self::newFromUntyped(untyped);
    }
    
    fn read<'br, T: 'br>(&'br self) -> impl Deref<Target = T> + 'br {
        return Strategy::read(self.asTypedRef());
    }
    
    fn write<'br, T: 'br>(&'br self) -> impl DerefMut<Target = T> + 'br {
        return Strategy::write(self.asTypedRef());
    }

    unsafe fn shallowClone<T>(&self) -> Self {
        let typed = Strategy::shallowClone::<T>(self.asTypedRef());
        let untyped = Strategy::intoUntyped(typed);
        return Self::newFromUntyped(untyped);
    }

    unsafe fn drop<T>(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.asTypedMut::<T>());
        }
    }
}

impl<Strategy> PointerHandleWithStrategy<Strategy>
where
    Strategy: PointerStrategy,
{
    fn newFromUntyped(untyped: Strategy::Untyped) -> Self {
        return Self {
            untyped: ManuallyDrop::new(untyped),
        };
    }

    fn asTypedRef<T>(&self) -> &Strategy::Typed<T> {
        let rawConstPointer = (self.untyped.deref() as *const Strategy::Untyped)
            .cast::<Strategy::Typed<T>>();

        Strategy::checkBinaryCompatibility::<T>();

        return unsafe {
            &*rawConstPointer
        };
    }

    fn asTypedMut<T>(&mut self) -> &mut Strategy::Typed<T> {
        let rawMutPointer = (self.untyped.deref_mut() as *mut Strategy::Untyped)
            .cast::<Strategy::Typed<T>>();

        Strategy::checkBinaryCompatibility::<T>();

        return unsafe {
            &mut *rawMutPointer
        };
    }
}
