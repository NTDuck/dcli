use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::pointers::PointerHandle;

/// Inspired by [archery](https://github.com/orium/archery).
/// 
/// See: [Higher Kinded Types in Rust](https://joshlf.com/post/2018/10/18/rust-higher-kinded-types-already/)
pub struct SharedPointer<T, Handle: PointerHandle> {
    handle: ManuallyDrop<Handle>,
    // Prevent `Send` and `Sync` auto implementation
    _marker: PhantomData<(T, *mut ())>,
}

unsafe impl<T, Handle> Send for SharedPointer<T, Handle>
where
    T: Sync + Send,
    Handle: PointerHandle + Send
{}

unsafe impl<T, Handle> Sync for SharedPointer<T, Handle>
where
    T: Sync + Send,
    Handle: PointerHandle + Sync
{}

impl<T, Handle> Unpin for SharedPointer<T, Handle>
where
    Handle: PointerHandle,
{}

impl<T, Handle> SharedPointer<T, Handle>
where
    Handle: PointerHandle,
{
    pub fn new(obj: T) -> Self {
        let handle = Handle::new(obj);
        return Self::new_from_handle(handle);
    }

    pub fn as_ref(&self) -> impl Deref<Target = T> + '_ {
        return self.handle.as_ref();
    }

    pub fn as_mut(&self) -> impl DerefMut<Target = T> + '_ {
        return self.handle.as_mut();
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
    Handle: PointerHandle,
{
    fn clone(&self) -> Self {
        let handle = unsafe {
            self.handle.shallow_clone::<T>()
        };
        return Self::new_from_handle(handle);
    }
}

impl<T, Handle> Drop for SharedPointer<T, Handle>
where
    Handle: PointerHandle,
{
    fn drop(&mut self) {
        unsafe {
            self.handle.drop::<T>();
        }
    }
}
