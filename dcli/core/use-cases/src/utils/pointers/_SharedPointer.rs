use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::gateways::pointers::SharedPointerHandle;

/// Inspired by [archery](https://github.com/orium/archery).
/// 
/// See: [Higher Kinded Types in Rust](https://joshlf.com/post/2018/10/18/rust-higher-kinded-types-already/)
pub struct SharedPointer<T, Handle: SharedPointerHandle> {
    handle: ManuallyDrop<Handle>,
    _marker: PhantomData<(
        T,   // Bind `T`
        *mut (),   // Prevent `Send` and `Sync` auto implementation
    )>,
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

impl<T, Handle> Unpin for SharedPointer<T, Handle>
where
    Handle: SharedPointerHandle,
{}

impl<T, Handle> SharedPointer<T, Handle>
where
    Handle: SharedPointerHandle,
{
    pub fn new(obj: T) -> Self {
        let handle = Handle::new(obj);
        return Self::new_from_handle(handle);
    }

    pub fn unwrap(&self) -> impl Deref<Target = T> + '_ {
        return self.handle.unwrap();
    }

    pub fn unwrap_mut(&self) -> impl DerefMut<Target = T> + '_ {
        return self.handle.unwrap_mut();
    }

    fn new_from_handle(handle: Handle) -> Self {
        return Self {
            handle: ManuallyDrop::new(handle),
            _marker: PhantomData,
        };
    }
}

impl<T, Handle> Clone for SharedPointer<T, Handle>
where
    Handle: SharedPointerHandle,
{
    fn clone(&self) -> Self {
        let handle = unsafe {
            self.handle.shallow_copy::<T>()
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
