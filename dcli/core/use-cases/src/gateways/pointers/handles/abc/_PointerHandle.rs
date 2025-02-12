use std::ops::Deref;
use std::ops::DerefMut;

pub unsafe trait PointerHandle: Sized {
    fn new<T>(obj: T) -> Self;

    fn read<'br, T: 'br>(&'br self) -> impl Deref<Target = T> + 'br;
    fn write<'br, T: 'br>(&'br self) -> impl DerefMut<Target = T> + 'br;

    unsafe fn shallowClone<T>(&self) -> Self;
    unsafe fn drop<T>(&mut self);
}
