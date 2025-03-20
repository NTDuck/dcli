use std::marker::PhantomData;

pub trait DeferredNewFrom<F, U>: Sized {
    fn new(from: F, using: U) -> Self;

    fn new_from(from: F) -> Deferred<Self, F, U> {
        return Deferred::from(from);
    }
}

pub struct Deferred<T, F, U> {
    from: F,
    _marker: PhantomData<(T, U)>,
}

impl<T, F, U> From<F> for Deferred<T, F, U> {
    fn from(from: F) -> Self {
        return Self {
            from,
            _marker: PhantomData,
        };
    }
}

impl<T, F, U> Deferred<T, F, U>
where
    T: DeferredNewFrom<F, U>,
{
    pub fn using(self, using: U) -> T {
        return T::new(self.from, using);
    }
}
