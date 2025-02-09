use std::ops::Deref;
use std::ops::DerefMut;

pub unsafe trait SharedPointerHandle: Sized {
    fn new<T>(obj: T) -> Self;

    fn unwrap<'br, T: 'br>(&'br self) -> impl Deref<Target = T> + 'br;
    fn unwrap_mut<'br, T: 'br>(&'br self) -> impl DerefMut<Target = T> + 'br;

    unsafe fn clone<T>(&self) -> Self;
    unsafe fn drop<T>(&mut self);
}
