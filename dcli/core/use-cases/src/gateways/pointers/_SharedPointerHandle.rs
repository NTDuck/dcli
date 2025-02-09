use std::ops::Deref;
use std::ops::DerefMut;

pub unsafe trait SharedPointerHandle: Sized {
    fn new<T>(obj: T) -> Self;

    fn unwrap<'a, T: 'a>(&'a self) -> impl Deref<Target = T> + 'a;
    fn unwrap_mut<'a, T: 'a>(&'a self) -> impl DerefMut<Target = T> + 'a;

    unsafe fn clone<T>(&self) -> Self;
    unsafe fn drop<T>(&mut self);
}
