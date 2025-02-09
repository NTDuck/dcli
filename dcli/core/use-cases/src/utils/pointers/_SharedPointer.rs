use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::gateways::pointers::SharedPointerHandle;

pub struct SharedPointer<T, Handle: SharedPointerHandle> {
    handle: ManuallyDrop<Handle>,
    _marker: PhantomData<T>,
    _marker_prevent_send_sync: PhantomData<*mut ()>,
}

unsafe impl<T, Handle> Send for SharedPointer<T, Handle>
where
    T: Sync + Send,
    Handle: SharedPointerHandle + Send
{}

unsafe impl<T, Handle> Sync for SharedPointer<T, Handle>
where
    T: Sync + Send,
    Handle: SharedPointerHandle + Sync
{}

impl<T, Handle> SharedPointer<T, Handle>
where
    Handle: SharedPointerHandle,
{
    pub fn new(obj: T) -> Self {
        let handle = Handle::new(obj);
        return Self::new_from_handle(handle);
    }

    fn new_from_handle(handle: Handle) -> Self {
        return Self {
            handle: ManuallyDrop::new(handle),
            _marker: PhantomData,
            _marker_prevent_send_sync: PhantomData,
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
        let handle = unsafe {
            self.handle
                .deref()
                .clone::<T>()
        };
        return Self::new_from_handle(handle);
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
