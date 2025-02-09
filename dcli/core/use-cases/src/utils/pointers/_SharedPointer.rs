use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::gateways::pointers::SharedPointerHandle;

pub struct SharedPointer<T, Handle: SharedPointerHandle> {
    handle: ManuallyDrop<Handle>,
    _marker: PhantomData<T>,
}

impl<T, Handle> SharedPointer<T, Handle>
where
    Handle: SharedPointerHandle,
{
    pub fn new(obj: T) -> Self {
        return Self {
            handle: ManuallyDrop::new(Handle::new(obj)),
            _marker: PhantomData,
        };
    }

    pub fn unwrap(&self) -> impl Deref<Target = T> + '_ {
        return self.handle.unwrap();
    }

    pub fn unwrap_mut(&self) -> impl DerefMut<Target = T> + '_ {
        return self.handle.unwrap_mut();
    }
}

impl<T, Handle> Clone for SharedPointer<T, Handle>
where
    Handle: SharedPointerHandle,
{
    fn clone(&self) -> Self {
        return Self {
            handle: ManuallyDrop::new(unsafe {
                self.handle
                    .deref()
                    .clone::<T>()
            }),
            _marker: PhantomData,
        }
    }
}

impl<T, Handle> Drop for SharedPointer<T, Handle>
where
    Handle: SharedPointerHandle,
{
    fn drop(&mut self) {
        unsafe {
            self.handle.drop::<T>();
        }
    }
}
