use std::borrow::Borrow;
use std::fmt::Pointer;
use std::ops::Deref;
use std::ops::DerefMut;

pub trait SmartPointer<T: ?Sized>:
    Sized + AsRef<T> + Borrow<T> + Deref<Target = T> + DerefMut<Target = T> + Pointer {
    fn new(value: T) -> Self where T: Sized;

    fn unwrap(&self) -> &T;
    fn unwrap_mut(&mut self) -> &mut T;
}
