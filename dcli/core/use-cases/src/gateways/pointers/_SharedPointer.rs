use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::gateways::pointers::SharedPointerType;

pub struct SharedPointer<T, P: SharedPointerType> {
    underlying_pointer: ManuallyDrop<P>,
    _marker: PhantomData<T>,
}

impl<T, P> SharedPointer<T, P>
where
    P: SharedPointerType,
{
    pub fn new(obj: T) -> Self {
        return Self {
            underlying_pointer: ManuallyDrop::new(P::new(obj)),
            _marker: PhantomData,
        };
    }

    // pub fn unwrap(&self) -> impl Deref<Target = T> + '_ {
    //     todo!()
    // }

    // pub fn unwrap_mut(&self) -> impl DerefMut<Target = T> + '_ {
    //     todo!()
    // }
}

impl<T, P> Clone for SharedPointer<T, P>
where
    P: SharedPointerType,
{
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T, P> Drop for SharedPointer<T, P>
where
    P: SharedPointerType,
{
    fn drop(&mut self) {
        todo!()
    }
}