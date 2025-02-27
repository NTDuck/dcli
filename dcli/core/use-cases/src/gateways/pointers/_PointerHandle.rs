use std::ops::Deref;
use std::ops::DerefMut;

pub unsafe trait PointerHandle: Sized {
    fn new<T>(obj: T) -> Self;

    fn as_ref<'br, T: 'br>(&'br self) -> impl Deref<Target = T> + 'br;
    fn as_mut<'br, T: 'br>(&'br self) -> impl DerefMut<Target = T> + 'br;

    unsafe fn shallow_clone<T>(&self) -> Self;
    unsafe fn drop<T>(&mut self);
}
